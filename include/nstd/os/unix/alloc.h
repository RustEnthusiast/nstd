#ifndef NSTD_OS_UNIX_ALLOC_H
#define NSTD_OS_UNIX_ALLOC_H
#include "../../core/def.h"
#include "../../nstd.h"

/// Allocates a block of memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate for the new block of memory.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/malloc.3.html>.
NSTDAPI NSTDAnyMut nstd_os_unix_alloc_allocate(NSTDUInt size);

/// Allocates a block of zero initialized memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate for the new block of memory.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/calloc.3p.html>.
NSTDAPI NSTDAnyMut nstd_os_unix_alloc_allocate_zeroed(NSTDUInt size);

/// Reallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the block of memory to reallocate.
///
/// - `NSTDUInt new_size` - The new size of the memory block.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if reallocating fails.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/realloc.3p.html>.
NSTDAPI NSTDErrorCode nstd_os_unix_alloc_reallocate(NSTDAnyMut *ptr, NSTDUInt new_size);

/// Deallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the block of memory to free.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/free.3p.html>.
NSTDAPI void nstd_os_unix_alloc_deallocate(NSTDAnyMut *ptr);

#endif
