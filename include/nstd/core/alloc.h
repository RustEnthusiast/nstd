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
