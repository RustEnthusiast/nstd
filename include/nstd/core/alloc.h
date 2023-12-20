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
