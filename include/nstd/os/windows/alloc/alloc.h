#ifndef NSTD_OS_WINDOWS_ALLOC_ALLOC_H
#define NSTD_OS_WINDOWS_ALLOC_ALLOC_H
#include "../../../core/alloc.h"
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
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
///
/// # Safety
///
/// - Behavior is undefined if `layout`'s size is zero.
///
/// - The new memory buffer should be considered uninitialized.
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_allocate(NSTDAllocLayout layout);

/// Allocates a new block of zero-initialized memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
///
/// # Safety
///
/// Behavior is undefined if `layout`'s size is zero.
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_allocate_zeroed(NSTDAllocLayout layout);

/// Reallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDAllocLayout old_layout` - Describes the previous memory layout.
///
/// - `NSTDAllocLayout new_layout` - Describes the new memory layout to allocate for.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_layout`'s size is zero.
///
/// - `ptr` must point to memory previously allocated with `old_layout`.
NSTDAPI NSTDWindowsAllocError nstd_os_windows_alloc_reallocate(
    NSTDAnyMut *ptr, NSTDAllocLayout old_layout, NSTDAllocLayout new_layout
);

/// Deallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the allocated memory.
///
/// # Safety
///
/// Behavior is undefined if `ptr` does not point to memory allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
NSTDAPI void nstd_os_windows_alloc_deallocate(NSTDAnyMut ptr);

#endif
