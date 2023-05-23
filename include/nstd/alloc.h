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
    /// A heap is invalid.
    NSTD_ALLOC_ERROR_INVALID_HEAP,
    /// An allocation function received input parameters that resulted in an invalid memory layout.
    NSTD_ALLOC_ERROR_INVALID_LAYOUT
} NSTDAllocError;

/// A structure of function pointers making up an allocator VTable.
typedef struct {
    /// An opaque pointer to the allocator's state.
    NSTDAny state;
    /// Allocates a contiguous sequence of `size` bytes in memory.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// Allocation will fail if `size` is greater than `NSTDInt`'s max value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `size` is zero.
    ///
    /// - The new memory buffer should be considered uninitialized.
    NSTDAnyMut (*allocate)(NSTDAny, NSTDUInt);
    /// Allocates a contiguous sequence of `size` bytes in memory.
    ///
    /// The initialized memory is zero-initialized.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// Allocation will fail if `size` is greater than `NSTDInt`'s max value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `size` is zero.
    ///
    /// - The new memory buffer should be considered uninitialized.
    NSTDAnyMut (*allocate_zeroed)(NSTDAny, NSTDUInt);
    /// Reallocates memory that was previously allocated by this allocator.
    ///
    /// Reallocation will fail if `new_size` is greater than `NSTDInt`'s max value.
    ///
    /// On successful reallocation, `ptr` will point to the new memory location and
    /// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and reallocation fails,
    /// the pointer will remain untouched and the appropriate error is returned.
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
    /// # Safety
    ///
    /// - Behavior is undefined if `new_size` is zero.
    ///
    /// - Behavior is undefined if `ptr` is not a value returned by this allocator.
    ///
    /// - `size` must be the same value that was used to allocate the memory buffer.
    NSTDAllocError (*reallocate)(NSTDAny, NSTDAnyMut *, NSTDUInt, NSTDUInt);
    /// Deallocates memory that was previously allocated by this allocator.
    ///
    /// On successful deallocation, `ptr` will be set to null and `NSTD_ALLOC_ERROR_NONE` will be
    /// returned. If this is not the case and deallocation fails, the pointer will remain untouched
    /// and the appropriate error is returned.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to
    /// null.
    ///
    /// - `NSTDUInt size` - The number of bytes currently allocated.
    ///
    /// # Returns
    ///
    /// `NSTDAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `ptr` is not a value returned by this allocator.
    ///
    /// - `size` must be the same value that was used to allocate the memory buffer.
    NSTDAllocError (*deallocate)(NSTDAny, NSTDAnyMut *, NSTDUInt);
} NSTDAllocator;

/// `nstd`'s default allocator.
NSTDAPI NSTDAllocator NSTD_ALLOCATOR;

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
/// # Safety
///
/// - Behavior is undefined if `size` is zero.
///
/// - The new memory buffer should be considered uninitialized.
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
/// # Safety
///
/// Behavior is undefined if `size` is zero.
NSTDAPI NSTDAnyMut nstd_alloc_allocate_zeroed(NSTDUInt size);

/// Reallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and
/// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and allocation fails, the
/// pointer will remain untouched and the appropriate error is returned.
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
/// # Safety
///
/// - Behavior is undefined if `new_size` is zero.
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
NSTDAPI NSTDAllocError nstd_alloc_reallocate(NSTDAnyMut *ptr, NSTDUInt size, NSTDUInt new_size);

/// Deallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to null.
///
/// - `NSTDUInt size` - The number of bytes to free.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
NSTDAPI NSTDAllocError nstd_alloc_deallocate(NSTDAnyMut *ptr, NSTDUInt size);

#endif
