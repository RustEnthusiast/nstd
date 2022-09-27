//! Low level memory allocation for Windows.
pub mod heap;
use self::heap::{
    nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_allocate_zeroed,
    nstd_os_windows_alloc_heap_deallocate, nstd_os_windows_alloc_heap_default,
    nstd_os_windows_alloc_heap_reallocate,
};
use crate::{alloc::NSTDAllocError, NSTDAnyMut, NSTDUInt};

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
/// - See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
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
/// - See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
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
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// - See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    new_size: NSTDUInt,
) -> NSTDAllocError {
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
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// - See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(ptr: &mut NSTDAnyMut) -> NSTDAllocError {
    let heap = nstd_os_windows_alloc_heap_default();
    nstd_os_windows_alloc_heap_deallocate(&heap, ptr)
}
