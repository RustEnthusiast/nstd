//! Low level memory allocation for Windows.
pub mod heap;
use self::heap::{
    nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_allocate_zeroed,
    nstd_os_windows_alloc_heap_deallocate, nstd_os_windows_alloc_heap_default,
    nstd_os_windows_alloc_heap_reallocate,
};
use crate::{NSTDAnyMut, NSTDUInt};

/// Describes an error returned from allocation functions for Windows.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDWindowsAllocError {
    /// No error occurred.
    NSTD_WINDOWS_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY,
    /// Deallocating memory failed.
    NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND,
    /// Getting a handle to a heap failed.
    NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND,
    /// A heap is invalid.
    NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP,
}

/// Allocates a new block of memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
///
/// # Panics
///
/// This operation will panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::mem::nstd_core_mem_zero,
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_deallocate,
///         NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<[u32; 5]>();
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate(SIZE);
///     assert!(!buf.is_null());
///
///     nstd_core_mem_zero(buf.cast(), SIZE);
///     assert!(*buf.cast::<[u32; 5]>() == [0u32; 5]);
///
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate(size: NSTDUInt) -> NSTDAnyMut {
    let heap = nstd_os_windows_alloc_heap_default();
    nstd_os_windows_alloc_heap_allocate(&heap, size)
}

/// Allocates a new block of zero-initialized memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
///
/// # Panics
///
/// This operation will panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     nstd_os_windows_alloc_allocate_zeroed, nstd_os_windows_alloc_deallocate,
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// const SIZE: usize = core::mem::size_of::<[i64; 5]>();
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate_zeroed(SIZE);
///     assert!(!buf.is_null());
///     assert!(*buf.cast::<[i64; 5]>() == [0i64; 5]);
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate_zeroed(size: NSTDUInt) -> NSTDAnyMut {
    let heap = nstd_os_windows_alloc_heap_default();
    nstd_os_windows_alloc_heap_allocate_zeroed(&heap, size)
}

/// Reallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and 0 will be
/// returned. If this is not the case and allocation fails, the pointer will remain untouched and a
/// value of nonzero is returned.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUInt new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     nstd_os_windows_alloc_allocate_zeroed, nstd_os_windows_alloc_deallocate,
///     nstd_os_windows_alloc_reallocate, NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// const SIZE: usize = core::mem::size_of::<[i16; 10]>();
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate_zeroed(SIZE);
///     assert!(!buf.is_null());
///     assert!(*buf.cast::<[i16; 10]>() == [0i16; 10]);
///
///     let mut errc = nstd_os_windows_alloc_reallocate(&mut buf, SIZE / 2);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
///     assert!(*buf.cast::<[i16; 5]>() == [0i16; 5]);
///
///     errc = nstd_os_windows_alloc_deallocate(&mut buf);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    new_size: NSTDUInt,
) -> NSTDWindowsAllocError {
    let heap = nstd_os_windows_alloc_heap_default();
    nstd_os_windows_alloc_heap_reallocate(&heap, ptr, new_size)
}

/// Deallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_deallocate,
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate(128);
///     assert!(!buf.is_null());
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(
    ptr: &mut NSTDAnyMut,
) -> NSTDWindowsAllocError {
    let heap = nstd_os_windows_alloc_heap_default();
    nstd_os_windows_alloc_heap_deallocate(&heap, ptr)
}
