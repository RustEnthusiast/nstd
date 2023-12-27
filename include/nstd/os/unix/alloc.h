#ifndef NSTD_OS_UNIX_ALLOC_H
#define NSTD_OS_UNIX_ALLOC_H
#include "../../core/alloc.h"
#include "../../nstd.h"

/// Allocates a block of memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// - Behavior is undefined if `layout`'s size is zero.
///
/// - The new memory buffer should be considered uninitialized.
NSTDAPI NSTDAnyMut nstd_os_unix_alloc_allocate(NSTDAllocLayout layout);

/// Allocates a block of zero initialized memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// Behavior is undefined if `layout`'s size is zero.
NSTDAPI NSTDAnyMut nstd_os_unix_alloc_allocate_zeroed(NSTDAllocLayout layout);

/// Reallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the block of memory to reallocate.
///
/// - `NSTDAllocLayout old_layout` - Describes the previous memory layout.
///
/// - `NSTDAllocLayout new_layout` - Describes the new memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_layout`'s size is zero.
///
/// - `ptr` must point to memory previously allocated with `old_layout`.
NSTDAPI NSTDAllocError nstd_os_unix_alloc_reallocate(
    NSTDAnyMut *ptr, NSTDAllocLayout old_layout, NSTDAllocLayout new_layout
);

/// Deallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the block of memory to free.
///
/// # Safety
///
/// Behavior is undefined if `ptr` does not point to memory allocated by
/// `nstd_os_unix_alloc_allocate[_zeroed]`.
NSTDAPI void nstd_os_unix_alloc_deallocate(NSTDAnyMut ptr);

#endif
