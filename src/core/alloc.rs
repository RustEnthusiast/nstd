//! Provides useful types for memory allocation support.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDUInt, NSTD_INT_MAX,
};
use nstdapi::nstdapi;

/// Describes a valid layout for a block of memory.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NSTDAllocLayout {
    /// The size of the memory block.
    size: NSTDUInt,
    /// The alignment of the memory block.
    align: NSTDUInt,
}
gen_optional!(NSTDOptionalAllocLayout, NSTDAllocLayout);

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
#[inline]
#[nstdapi]
pub const fn nstd_core_alloc_layout_new(
    size: NSTDUInt,
    align: NSTDUInt,
) -> NSTDOptionalAllocLayout {
    match size <= NSTD_INT_MAX && crate::core::mem::is_power_of_two(align) {
        true => NSTDOptional::Some(NSTDAllocLayout { size, align }),
        false => NSTDOptional::None,
    }
}

/// Returns the size of an `NSTDAllocLayout`.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - The memory layout.
///
/// # Returns
///
/// `NSTDUInt size` - The layout's size.
#[inline]
#[nstdapi]
pub const fn nstd_core_alloc_layout_size(layout: NSTDAllocLayout) -> NSTDUInt {
    layout.size
}

/// Returns the alignment of an `NSTDAllocLayout`.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - The memory layout.
///
/// # Returns
///
/// `NSTDUInt align` - The layout's alignment.
#[inline]
#[nstdapi]
pub const fn nstd_core_alloc_layout_align(layout: NSTDAllocLayout) -> NSTDUInt {
    layout.align
}
