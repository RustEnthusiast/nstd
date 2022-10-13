#ifndef NSTD_OS_WINDOWS_ALLOC_ALLOC_H
#define NSTD_OS_WINDOWS_ALLOC_ALLOC_H
#include "../../../nstd.h"

/// Describes an error returned from allocation functions for Windows.
typedef enum {
    /// No error occurred.
    NSTD_WINDOWS_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY,
    /// Deallocating memory failed.
    NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND,
    /// Getting a handle to a heap failed.
    NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND,
    /// A heap is invalid.
    NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP
} NSTDWindowsAllocError;

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
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_allocate(NSTDUInt size);

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
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_allocate_zeroed(NSTDUInt size);

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
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
NSTDAPI NSTDWindowsAllocError nstd_os_windows_alloc_reallocate(NSTDAnyMut *ptr, NSTDUInt new_size);

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
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
NSTDAPI NSTDWindowsAllocError nstd_os_windows_alloc_deallocate(NSTDAnyMut *ptr);

#endif
