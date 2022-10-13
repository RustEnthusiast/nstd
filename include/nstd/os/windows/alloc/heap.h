#ifndef NSTD_OS_WINDOWS_ALLOC_HEAP_H
#define NSTD_OS_WINDOWS_ALLOC_HEAP_H
#include "../../../nstd.h"
#include "alloc.h"

/// A handle to a process heap.
typedef struct {
    /// The private handle.
    NSTDInt handle;
} NSTDWindowsHeap;

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeap heap` - A handle to the default heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
NSTDAPI NSTDWindowsHeap nstd_os_windows_alloc_heap_default();

/// Creates a new private heap for the process.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The initial size of the heap, in bytes. If this parameter is 0,
/// the heap gets one page.
///
/// # Returns
///
/// `NSTDWindowsHeap heap` - A handle to the new private heap.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate>.
NSTDAPI NSTDWindowsHeap nstd_os_windows_alloc_heap_new(NSTDUInt size);

/// Returns the size of a memory block previously allocated by an `NSTDWindowsHeap`.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap.
///
/// - `NSTDAny ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDUInt size` - The number of bytes allocated at the memory block pointed to by `ptr`.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsize>.
NSTDAPI NSTDUInt nstd_os_windows_alloc_heap_size(const NSTDWindowsHeap *heap, NSTDAny ptr);

/// Allocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to allocate on.
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the new block of memory on the heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_heap_allocate(const NSTDWindowsHeap *heap, NSTDUInt size);

/// Allocates a zero-initialized block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to allocate on.
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the new block of memory on the heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_heap_allocate_zeroed(const NSTDWindowsHeap *heap,
NSTDUInt size);

/// Reallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to reallocate on.
///
/// - `NSTDAnyMut *ptr` - A pointer to the memory to reallocate.
///
/// - `NSTDUInt size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
NSTDAPI NSTDWindowsAllocError nstd_os_windows_alloc_heap_reallocate(const NSTDWindowsHeap *heap,
NSTDAnyMut *ptr, NSTDUInt size);

/// Deallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to deallocate memory from.
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
NSTDAPI NSTDWindowsAllocError nstd_os_windows_alloc_heap_deallocate(const NSTDWindowsHeap *heap,
NSTDAnyMut *ptr);

/// Destroys a private heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeap heap` - The heap to destroy.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy>.
NSTDAPI NSTDWindowsAllocError nstd_os_windows_alloc_heap_free(NSTDWindowsHeap heap);

#endif
