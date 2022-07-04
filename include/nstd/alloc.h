#ifndef NSTD_ALLOC_H_INCLUDED
#define NSTD_ALLOC_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Allocates a block of memory on the heap.
/// The number of bytes to be allocated is specified by `size`.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `size` is zero.
NSTDAPI NSTDAnyMut nstd_alloc_allocate(NSTDUSize size);

/// Allocates a block of zero-initialized memory on the heap.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `size` is zero.
NSTDAPI NSTDAnyMut nstd_alloc_allocate_zeroed(NSTDUSize size);

/// Reallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and 0 will be
/// returned. If this is not the case and allocation fails, the pointer will remain untouched and a
/// value of nonzero is returned.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUSize size` - The number of bytes currently allocated.
///
/// - `NSTDUSize new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `ptr` is not a value returned by
/// `nstd_alloc_allocate[_zeroed]`.
NSTDAPI NSTDErrorCode nstd_alloc_reallocate(NSTDAnyMut *ptr, NSTDUSize size, NSTDUSize new_size);

/// Deallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to null.
///
/// - `NSTDUSize size` - The number of bytes to free.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `ptr` is not a value returned by
/// `nstd_alloc_allocate[_zeroed]`.
NSTDAPI void nstd_alloc_deallocate(NSTDAnyMut *ptr, NSTDUSize size);

NSTDCPPEND
#endif
