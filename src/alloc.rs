//! Low level memory allocation.
#[cfg(not(any(target_family = "unix", target_os = "windows")))]
extern crate alloc;
#[cfg(target_family = "unix")]
use crate::os::unix::alloc::{
    nstd_os_unix_alloc_allocate, nstd_os_unix_alloc_allocate_zeroed, nstd_os_unix_alloc_deallocate,
    nstd_os_unix_alloc_reallocate,
};
#[cfg(target_os = "windows")]
use crate::os::windows::alloc::{
    nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_allocate_zeroed,
    nstd_os_windows_alloc_deallocate, nstd_os_windows_alloc_reallocate,
    NSTDWindowsAllocError::{self, *},
};
use crate::{NSTDAnyMut, NSTDUInt};

/// The default alignment suitable for any scalar type.
///
/// Corresponds to `alignof(max_align_t)`.
/// The C/C++ standards specify that this value should be at least 8 or 16, I'm going with 16 for
/// safety but of course this is platform dependent so if you (the reader) know of a platform that
/// this value is smaller (or larger for that matter) on, please submit an issue/pull request.
#[cfg(not(any(target_family = "unix", target_os = "windows")))]
const DEFAULT_ALIGNMENT: usize = 16;

/// Describes an error returned from allocation functions.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDAllocError {
    /// No error occurred.
    NSTD_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
    /// Deallocating memory failed.
    NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND,
    /// Getting a handle to a heap failed.
    NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
    /// A heap is invalid.
    NSTD_ALLOC_ERROR_INVALID_HEAP,
    /// An allocation function received input parameters that resulted in an invalid memory layout.
    NSTD_ALLOC_ERROR_INVALID_LAYOUT,
}
impl NSTDAllocError {
    /// Converts an [NSTDWindowsAllocError] into an [NSTDAllocError].
    #[cfg(target_os = "windows")]
    fn from_windows(err: NSTDWindowsAllocError) -> Self {
        match err {
            NSTD_WINDOWS_ALLOC_ERROR_NONE => Self::NSTD_ALLOC_ERROR_NONE,
            NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY => Self::NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
            NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND => Self::NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND,
            NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND => Self::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
            NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP => Self::NSTD_ALLOC_ERROR_INVALID_HEAP,
        }
    }
}

/// Allocates a block of memory on the heap.
/// The number of bytes to be allocated is specified by `size`.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// - Behavior is undefined if `size` is zero.
///
/// - The new memory buffer should be considered uninitialized.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{nstd_alloc_allocate, nstd_alloc_deallocate};
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate(32);
///     assert!(!mem.is_null());
///     nstd_alloc_deallocate(&mut mem, 32);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: NSTDUInt) -> NSTDAnyMut {
    #[cfg(not(any(target_family = "unix", target_os = "windows")))]
    {
        use crate::NSTD_NULL;
        use alloc::alloc::Layout;
        if let Ok(layout) = Layout::from_size_align(size, DEFAULT_ALIGNMENT) {
            return alloc::alloc::alloc(layout).cast();
        }
        NSTD_NULL
    }
    #[cfg(target_family = "unix")]
    {
        nstd_os_unix_alloc_allocate(size)
    }
    #[cfg(target_os = "windows")]
    {
        nstd_os_windows_alloc_allocate(size)
    }
}

/// Allocates a block of zero-initialized memory on the heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// Behavior is undefined if `size` is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{nstd_alloc_allocate_zeroed, nstd_alloc_deallocate};
///
/// const SIZE: usize = core::mem::size_of::<[i16; 16]>();
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate_zeroed(SIZE);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<[i16; 16]>() == [0i16; 16]);
///
///     nstd_alloc_deallocate(&mut mem, SIZE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: NSTDUInt) -> NSTDAnyMut {
    #[cfg(not(any(target_family = "unix", target_os = "windows")))]
    {
        use crate::NSTD_NULL;
        use alloc::alloc::Layout;
        if let Ok(layout) = Layout::from_size_align(size, DEFAULT_ALIGNMENT) {
            return alloc::alloc::alloc_zeroed(layout).cast();
        }
        NSTD_NULL
    }
    #[cfg(target_family = "unix")]
    {
        nstd_os_unix_alloc_allocate_zeroed(size)
    }
    #[cfg(target_os = "windows")]
    {
        nstd_os_windows_alloc_allocate_zeroed(size)
    }
}

/// Reallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and 0 will be
/// returned. If this is not the case and allocation fails, the pointer will remain untouched and a
/// value of nonzero is returned.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUInt size` - The number of bytes currently allocated.
///
/// - `NSTDUInt new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_size` is zero.
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{
///     nstd_alloc_allocate_zeroed, nstd_alloc_deallocate, nstd_alloc_reallocate,
///     NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
/// };
///
/// const SIZE: usize = core::mem::size_of::<[u64; 64]>();
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate_zeroed(SIZE);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<[u64; 64]>() == [0u64; 64]);
///
///     assert!(nstd_alloc_reallocate(&mut mem, SIZE, SIZE / 2) == NSTD_ALLOC_ERROR_NONE);
///     assert!(*mem.cast::<[u64; 32]>() == [0u64; 32]);
///
///     nstd_alloc_deallocate(&mut mem, SIZE);
/// }
/// ```
#[cfg_attr(any(target_family = "unix", target_os = "windows"), inline)]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(
    any(target_family = "unix", target_os = "windows"),
    allow(unused_variables)
)]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
    new_size: NSTDUInt,
) -> NSTDAllocError {
    #[cfg(not(any(target_family = "unix", target_os = "windows")))]
    {
        use alloc::alloc::Layout;
        if let Ok(layout) = Layout::from_size_align(size, DEFAULT_ALIGNMENT) {
            let new_mem = alloc::alloc::realloc((*ptr).cast(), layout, new_size);
            if new_mem.is_null() {
                return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
            }
            *ptr = new_mem.cast();
            return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
        }
        NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
    }
    #[cfg(target_family = "unix")]
    {
        match nstd_os_unix_alloc_reallocate(ptr, new_size) {
            0 => NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
            _ => NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
        }
    }
    #[cfg(target_os = "windows")]
    {
        NSTDAllocError::from_windows(nstd_os_windows_alloc_reallocate(ptr, new_size))
    }
}

/// Deallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to null.
///
/// - `NSTDUInt size` - The number of bytes to free.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{nstd_alloc_allocate, nstd_alloc_deallocate};
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate(24);
///     assert!(!mem.is_null());
///     nstd_alloc_deallocate(&mut mem, 24);
/// }
/// ```
#[cfg_attr(any(target_family = "unix", target_os = "windows"), inline)]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(
    any(target_family = "unix", target_os = "windows"),
    allow(unused_variables)
)]
pub unsafe extern "C" fn nstd_alloc_deallocate(
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDAllocError {
    #[cfg(not(any(target_family = "unix", target_os = "windows")))]
    {
        use crate::NSTD_NULL;
        use alloc::alloc::Layout;
        if let Ok(layout) = Layout::from_size_align(size, DEFAULT_ALIGNMENT) {
            alloc::alloc::dealloc((*ptr).cast(), layout);
            *ptr = NSTD_NULL;
            return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
        }
        NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
    }
    #[cfg(target_family = "unix")]
    {
        nstd_os_unix_alloc_deallocate(ptr);
        NSTDAllocError::NSTD_ALLOC_ERROR_NONE
    }
    #[cfg(target_os = "windows")]
    {
        NSTDAllocError::from_windows(nstd_os_windows_alloc_deallocate(ptr))
    }
}
