#ifndef NSTD_CORE_ALLOC_H
#define NSTD_CORE_ALLOC_H
#include "../nstd.h"
#include "optional.h"

/// Describes a valid layout for a block of memory.
typedef struct {
    /// The size of the memory block.
    NSTDUInt size;
    /// The alignment of the memory block.
    NSTDUInt align;
} NSTDAllocLayout;

/// Represents an optional value of type `NSTDAllocLayout`.
NSTDOptional(NSTDAllocLayout) NSTDOptionalAllocLayout;

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

/// A structure of function pointers making up an allocator's virtual function table.
typedef struct {
    /// An opaque pointer to the allocator's state.
    NSTDAny state;
    /// Allocates a new block of memory.
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
    /// - Behavior is undefined if `layout`'s size is zero.
    ///
    /// - The new memory buffer should be considered uninitialized.
    NSTDAnyMut (*allocate)(NSTDAny, NSTDAllocLayout);
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
    NSTDAnyMut (*allocate_zeroed)(NSTDAny, NSTDAllocLayout);
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
    NSTDAllocError (*reallocate)(NSTDAny, NSTDAnyMut *, NSTDAllocLayout, NSTDAllocLayout);
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
    NSTDAllocError (*deallocate)(NSTDAny, NSTDAnyMut, NSTDAllocLayout);
} NSTDAllocator;

/// Creates a new memory layout from a size and alignment.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The size of the memory block.
///
/// - `NSTDUInt align` - The alignment of the memory block.
///
/// # Returns
///
/// `NSTDOptionalAllocLayout layout` - The memory layout on success, or an uninitialized "none"
/// variant if either `size` is greater than `NSTDInt`'s max value or `align` is not a power of two.
NSTDAPI NSTDOptionalAllocLayout nstd_core_alloc_layout_new(NSTDUInt size, NSTDUInt align);

/// Creates a new memory layout from a size and alignment without performing safety checks.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The size of the memory block.
///
/// - `NSTDUInt align` - The alignment of the memory block.
///
/// # Returns
///
/// `NSTDAllocLayout layout` - The memory layout.
///
/// # Safety
///
/// - `size` must not be greater than `NSTDInt`'s max value.
///
/// - `align` must be a nonzero power of two.
NSTDAPI NSTDAllocLayout nstd_core_alloc_layout_new_unchecked(NSTDUInt size, NSTDUInt align);

/// Creates a new memory layout for an array of elements.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The size of each element in the array.
///
/// - `NSTDUInt align` - The alignment of each element in the array.
///
/// - `NSTDUInt len` - The length of the array to create a memory layout for.
///
/// # Returns
///
/// `NSTDOptionalAllocLayout layout` - The memory layout on success, or an uninitialized "none"
/// variant if the calculated size is greater than `NSTDInt`'s max value, `size` is not a multiple
/// of `align`, or `align` is not a power of two.
NSTDAPI NSTDOptionalAllocLayout
nstd_core_alloc_layout_array(NSTDUInt size, NSTDUInt align, NSTDUInt len);

/// Creates a new memory layout for an array of elements without performing safety checks.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The size of each element in the array.
///
/// - `NSTDUInt align` - The alignment of each element in the array.
///
/// - `NSTDUInt len` - The length of the array to create a memory layout for.
///
/// # Returns
///
/// `NSTDAllocLayout layout` - The new memory layout.
///
/// # Panics
///
/// This operation will panic if `align` is 0.
///
/// # Safety
///
/// - `align` must be a power of two.
///
/// - `size` must be a multiple of `align`.
///
/// - The calculated size must not be greater than `NSTDInt`'s max value.
NSTDAPI NSTDAllocLayout
nstd_core_alloc_layout_array_unchecked(NSTDUInt size, NSTDUInt align, NSTDUInt len);

/// Returns the size of an `NSTDAllocLayout`.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - The memory layout.
///
/// # Returns
///
/// `NSTDUInt size` - The layout's size.
NSTDAPI NSTDUInt nstd_core_alloc_layout_size(NSTDAllocLayout layout);

/// Returns the alignment of an `NSTDAllocLayout`.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - The memory layout.
///
/// # Returns
///
/// `NSTDUInt align` - The layout's alignment.
NSTDAPI NSTDUInt nstd_core_alloc_layout_align(NSTDAllocLayout layout);

#endif
