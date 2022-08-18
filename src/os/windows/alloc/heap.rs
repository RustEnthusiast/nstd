//! Process heap management for Windows.
use crate::{alloc::NSTDAllocError, NSTDAnyMut, NSTDISize, NSTDUSize, NSTD_NULL};
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapCreate, HeapDestroy, HeapFree, HeapReAlloc, HEAP_ZERO_MEMORY,
};

/// A handle to a process heap.
pub type NSTDWindowsHeapHandle = NSTDISize;

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeapHandle heap` - A handle to the default heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_default() -> NSTDWindowsHeapHandle {
    GetProcessHeap()
}

/// Creates a new private heap for the process.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The initial size of the heap, in bytes. If this parameter is 0,
/// the heap gets one page.
///
/// # Returns
///
/// `NSTDWindowsHeapHandle heap` - A handle to the new private heap.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_new(size: NSTDUSize) -> NSTDWindowsHeapHandle {
    HeapCreate(0, size, 0)
}

/// Allocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle heap` - A handle to the heap to allocate on.
///
/// - `NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the new block of memory on the heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate(
    heap: NSTDWindowsHeapHandle,
    size: NSTDUSize,
) -> NSTDAnyMut {
    HeapAlloc(heap, 0, size)
}

/// Allocates a zero-initialized block of memory on a heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle heap` - A handle to the heap to allocate on.
///
/// - `NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the new block of memory on the heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate_zeroed(
    heap: NSTDWindowsHeapHandle,
    size: NSTDUSize,
) -> NSTDAnyMut {
    HeapAlloc(heap, HEAP_ZERO_MEMORY, size)
}

/// Reallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle heap` - A handle to the heap to reallocate on.
///
/// - `NSTDAnyMut *ptr` - A pointer to the memory to reallocate.
///
/// - `NSTDUSize size` - The number of bytes to reallocate.
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
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_reallocate(
    heap: NSTDWindowsHeapHandle,
    ptr: &mut NSTDAnyMut,
    size: NSTDUSize,
) -> NSTDAllocError {
    let new_mem = HeapReAlloc(heap, 0, *ptr, size);
    if !new_mem.is_null() {
        *ptr = new_mem;
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY
}

/// Deallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle heap` - A handle to the heap to deallocate memory from.
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
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_deallocate(
    heap: NSTDWindowsHeapHandle,
    ptr: &mut NSTDAnyMut,
) -> NSTDAllocError {
    if HeapFree(heap, 0, *ptr) != 0 {
        *ptr = NSTD_NULL;
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND
}

/// Destroys a private heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle *heap` - The heap to destroy.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_free(
    heap: &mut NSTDWindowsHeapHandle,
) -> NSTDAllocError {
    if HeapDestroy(*heap) != 0 {
        *heap = 0;
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND
}
