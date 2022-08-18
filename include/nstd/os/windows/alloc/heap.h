#ifndef NSTD_OS_WINDOWS_ALLOC_HEAP_H
#define NSTD_OS_WINDOWS_ALLOC_HEAP_H
#include "../../../alloc.h"
#include "../../../nstd.h"
NSTDCPPSTART

/// A handle to a process heap.
typedef NSTDISize NSTDWindowsHeapHandle;

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeapHandle heap` - A handle to the default heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
NSTDAPI NSTDWindowsHeapHandle nstd_os_windows_alloc_heap_default();

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
NSTDAPI NSTDWindowsHeapHandle nstd_os_windows_alloc_heap_new(NSTDUSize size);

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
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_heap_allocate(NSTDWindowsHeapHandle heap, NSTDUSize size);

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
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_heap_allocate_zeroed(NSTDWindowsHeapHandle heap,
NSTDUSize size);

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
NSTDAPI NSTDAllocError nstd_os_windows_alloc_heap_reallocate(NSTDWindowsHeapHandle heap,
NSTDAnyMut *ptr, NSTDUSize size);

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
NSTDAPI NSTDAllocError nstd_os_windows_alloc_heap_deallocate(NSTDWindowsHeapHandle heap,
NSTDAnyMut *ptr);

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
NSTDAPI NSTDAllocError nstd_os_windows_alloc_heap_free(NSTDWindowsHeapHandle *heap);

NSTDCPPEND
#endif
