//! Low level memory allocation.
#[cfg(not(any(
    unix,
    windows,
    any(target_env = "wasi", target_os = "wasi"),
    target_os = "fuchsia",
    target_os = "solid_asp3",
    target_os = "vxworks"
)))]
extern crate alloc;
#[cfg(windows)]
use crate::os::windows::alloc::{
    nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_allocate_zeroed,
    nstd_os_windows_alloc_deallocate, nstd_os_windows_alloc_reallocate,
    NSTDWindowsAllocError::{self, *},
};
use crate::{NSTDAnyMut, NSTDUInt};
use cfg_if::cfg_if;
use nstdapi::nstdapi;

/// Describes an error returned from allocation functions.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[cfg(windows)]
impl From<NSTDWindowsAllocError> for NSTDAllocError {
    /// Converts an [NSTDWindowsAllocError] into an [NSTDAllocError].
    fn from(err: NSTDWindowsAllocError) -> Self {
        match err {
            NSTD_WINDOWS_ALLOC_ERROR_NONE => Self::NSTD_ALLOC_ERROR_NONE,
            NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY => Self::NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
            NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND => Self::NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND,
            NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND => Self::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
            NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP => Self::NSTD_ALLOC_ERROR_INVALID_HEAP,
            NSTD_WINDOWS_ALLOC_ERROR_INVALID_LAYOUT => Self::NSTD_ALLOC_ERROR_INVALID_LAYOUT,
        }
    }
}

/// A structure of function pointers making up an allocator VTable.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDAllocator {
    /// Allocates a contiguous sequence of `size` bytes in memory.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// Allocation will fail if `size` is greater than `NSTDInt`'s max value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate.
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
    pub allocate: unsafe extern "C" fn(NSTDUInt) -> NSTDAnyMut,
    /// Allocates a contiguous sequence of `size` bytes in memory.
    ///
    /// The initialized memory is zero-initialized.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// Allocation will fail if `size` is greater than `NSTDInt`'s max value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate.
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
    pub allocate_zeroed: unsafe extern "C" fn(NSTDUInt) -> NSTDAnyMut,
    /// Reallocates memory that was previously allocated by this allocator.
    ///
    /// Reallocation will fail if `new_size` is greater than `NSTDInt`'s max value.
    ///
    /// On successful reallocation, `ptr` will point to the new memory location and
    /// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and reallocation fails,
    /// the pointer will remain untouched and the appropriate error is returned.
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
    /// - Behavior is undefined if `ptr` is not a value returned by this allocator.
    ///
    /// - `size` must be the same value that was used to allocate the memory buffer.
    pub reallocate: unsafe extern "C" fn(&mut NSTDAnyMut, NSTDUInt, NSTDUInt) -> NSTDAllocError,
    /// Deallocates memory that was previously allocated by this allocator.
    ///
    /// On successful deallocation, `ptr` will be set to null and `NSTD_ALLOC_ERROR_NONE` will be
    /// returned. If this is not the case and deallocation fails, the pointer will remain untouched
    /// and the appropriate error is returned.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to
    /// null.
    ///
    /// - `NSTDUInt size` - The number of bytes currently allocated.
    ///
    /// # Returns
    ///
    /// `NSTDAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `ptr` is not a value returned by this allocator.
    ///
    /// - `size` must be the same value that was used to allocate the memory buffer.
    pub deallocate: unsafe extern "C" fn(&mut NSTDAnyMut, NSTDUInt) -> NSTDAllocError,
}

/// `nstd`'s default allocator.
#[nstdapi]
pub static NSTD_ALLOCATOR: NSTDAllocator = NSTDAllocator {
    allocate: nstd_alloc_allocate,
    allocate_zeroed: nstd_alloc_allocate_zeroed,
    reallocate: nstd_alloc_reallocate,
    deallocate: nstd_alloc_deallocate,
};

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
#[nstdapi]
pub unsafe extern "C" fn nstd_alloc_allocate(size: NSTDUInt) -> NSTDAnyMut {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "fuchsia",
            target_os = "solid_asp3",
            target_os = "vxworks"
        ))] {
            use crate::{core::NSTD_INT_MAX, NSTD_NULL};
            match size <= NSTD_INT_MAX as _ {
                true => libc::malloc(size),
                false => NSTD_NULL,
            }
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_allocate(size)
        } else {
            use crate::{core::ptr::raw::MAX_ALIGN, NSTD_NULL};
            use alloc::alloc::Layout;
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                return alloc::alloc::alloc(layout).cast();
            }
            NSTD_NULL
        }
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
#[nstdapi]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: NSTDUInt) -> NSTDAnyMut {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "fuchsia",
            target_os = "solid_asp3",
            target_os = "vxworks"
        ))] {
            use crate::{core::NSTD_INT_MAX, NSTD_NULL};
            match size <= NSTD_INT_MAX as _ {
                true => libc::calloc(size, 1),
                false => NSTD_NULL,
            }
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_allocate_zeroed(size)
        } else {
            use crate::{core::ptr::raw::MAX_ALIGN, NSTD_NULL};
            use alloc::alloc::Layout;
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                return alloc::alloc::alloc_zeroed(layout).cast();
            }
            NSTD_NULL
        }
    }
}

/// Reallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and
/// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and allocation fails, the
/// pointer will remain untouched and the appropriate error is returned.
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
#[nstdapi]
#[cfg_attr(windows, inline)]
#[allow(unused_variables)]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
    new_size: NSTDUInt,
) -> NSTDAllocError {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "fuchsia",
            target_os = "solid_asp3",
            target_os = "vxworks"
        ))] {
            use crate::core::NSTD_INT_MAX;
            if new_size > NSTD_INT_MAX as _ {
                return NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT;
            }
            let new_mem = libc::realloc(*ptr, new_size);
            if new_mem.is_null() {
                return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
            }
            *ptr = new_mem;
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_reallocate(ptr, new_size).into()
        } else {
            use crate::core::ptr::raw::MAX_ALIGN;
            use alloc::alloc::Layout;
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                let new_mem = alloc::alloc::realloc((*ptr).cast(), layout, new_size);
                if new_mem.is_null() {
                    return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
                }
                *ptr = new_mem.cast();
                return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
            }
            NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
        }
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
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub unsafe extern "C" fn nstd_alloc_deallocate(
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDAllocError {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "fuchsia",
            target_os = "solid_asp3",
            target_os = "vxworks"
        ))] {
            use crate::NSTD_NULL;
            libc::free(*ptr);
            *ptr = NSTD_NULL;
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_deallocate(ptr).into()
        } else {
            use crate::{core::ptr::raw::MAX_ALIGN, NSTD_NULL};
            use alloc::alloc::Layout;
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                alloc::alloc::dealloc((*ptr).cast(), layout);
                *ptr = NSTD_NULL;
                return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
            }
            NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
        }
    }
}
