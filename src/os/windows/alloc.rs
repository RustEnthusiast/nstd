//! Low level memory allocation for Windows.
pub mod heap;
use self::heap::{
    nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_allocate_zeroed,
    nstd_os_windows_alloc_heap_deallocate, nstd_os_windows_alloc_heap_default,
    nstd_os_windows_alloc_heap_reallocate,
};
use crate::{alloc::NSTDAllocError, NSTDAnyMut, NSTDUSize, NSTD_NULL};

/// Allocates a new block of memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate(size: NSTDUSize) -> NSTDAnyMut {
    match nstd_os_windows_alloc_heap_default() {
        0 => NSTD_NULL,
        heap => nstd_os_windows_alloc_heap_allocate(heap, size),
    }
}

/// Allocates a new block of zero-initialized memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate_zeroed(size: NSTDUSize) -> NSTDAnyMut {
    match nstd_os_windows_alloc_heap_default() {
        0 => NSTD_NULL,
        heap => nstd_os_windows_alloc_heap_allocate_zeroed(heap, size),
    }
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
/// - `NSTDUSize new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    new_size: NSTDUSize,
) -> NSTDAllocError {
    match nstd_os_windows_alloc_heap_default() {
        0 => NSTDAllocError::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
        heap => nstd_os_windows_alloc_heap_reallocate(heap, ptr, new_size),
    }
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
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(ptr: &mut NSTDAnyMut) -> NSTDAllocError {
    match nstd_os_windows_alloc_heap_default() {
        0 => NSTDAllocError::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
        heap => nstd_os_windows_alloc_heap_deallocate(heap, ptr),
    }
}
