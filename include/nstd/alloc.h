#ifndef NSTD_ALLOC_H
#define NSTD_ALLOC_H
#include "nstd.h"

/// Describes an error returned from allocation functions.
typedef enum {
    /// No error occurred.
    NSTD_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
    /// Deallocating memory failed.
    NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND,
    /// Getting a handle to a heap failed.
    NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
} NSTDAllocError;

/// Allocates a block of memory on the heap.
/// The number of bytes to be allocated is specified by `size`.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Panics
///
/// This function may panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// - Behavior is undefined if `size` is zero.
///
/// - The new memory buffer should be considered uninitialized.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDAnyMut nstd_alloc_allocate(NSTDUInt size);

/// Allocates a block of zero-initialized memory on the heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Panics
///
/// This function may panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// - Behavior is undefined if `size` is zero.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDAnyMut nstd_alloc_allocate_zeroed(NSTDUInt size);

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
/// - `NSTDUInt size` - The number of bytes currently allocated.
///
/// - `NSTDUInt new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This function may panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// - Behavior is undefined if `new_size` is zero.
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDAllocError nstd_alloc_reallocate(NSTDAnyMut *ptr, NSTDUInt size, NSTDUInt new_size);

/// Deallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to null.
///
/// - `NSTDUInt size` - The number of bytes to free.
///
/// # Panics
///
/// This function may panic if getting a handle to the default heap fails.
///
/// # Safety
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI void nstd_alloc_deallocate(NSTDAnyMut *ptr, NSTDUInt size);

#endif
