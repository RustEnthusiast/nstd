#ifndef NSTD_ALLOC_H
#define NSTD_ALLOC_H
#include "core/alloc.h"
#include "nstd.h"

/// `nstd`'s default allocator.
NSTDAPI const NSTDAllocator NSTD_ALLOCATOR;

/// Allocates a new block of memory.
///
/// If allocation fails, a null pointer is returned.
///
/// If allocation succeeds, this returns a pointer to the new memory that is suitably aligned for
/// `layout`'s alignment and the number of bytes allocated is at least equal to `layout`'s size.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// - Behavior is undefined if `layout`'s size is zero.
///
/// - The new memory buffer should be considered uninitialized.
NSTDAPI NSTDAnyMut nstd_alloc_allocate(NSTDAllocLayout layout);

/// Allocates a new block of zero-initialized memory.
///
/// If allocation fails, a null pointer is returned.
///
/// If allocation succeeds, this returns a pointer to the new memory that is suitably aligned
/// for `layout`'s alignment and the number of bytes allocated is at least equal to `layout`'s
/// size.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// Behavior is undefined if `layout`'s size is zero.
NSTDAPI NSTDAnyMut nstd_alloc_allocate_zeroed(NSTDAllocLayout layout);

/// Reallocates memory that was previously allocated by this allocator.
///
/// On successful reallocation, `ptr` will point to the new memory location and
/// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and reallocation fails,
/// the pointer will remain untouched and the appropriate error is returned.
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
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_layout`'s size is zero.
///
/// - Behavior is undefined if `ptr` is not a pointer to memory allocated by this allocator.
///
/// - `old_layout` must be the same value that was used to allocate the memory buffer.
NSTDAPI NSTDAllocError
nstd_alloc_reallocate(NSTDAnyMut *ptr, NSTDAllocLayout old_layout, NSTDAllocLayout new_layout);

/// Deallocates memory that was previously allocated by this allocator.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the allocated memory.
///
/// - `NSTDAllocLayout layout` - Describes the layout of memory that `ptr` points to.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `ptr` is not a pointer to memory allocated by this allocator.
///
/// - `layout` must be the same value that was used to allocate the memory buffer.
NSTDAPI NSTDAllocError nstd_alloc_deallocate(NSTDAnyMut ptr, NSTDAllocLayout layout);

#endif
