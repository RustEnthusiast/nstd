//! Provides useful types for memory allocation support.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDUInt, NSTD_INT_MAX,
};
use nstdapi::nstdapi;

/// Describes a valid layout for a block of memory.
#[repr(C)]
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
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_alloc_layout_new_unchecked(
    size: NSTDUInt,
    align: NSTDUInt,
) -> NSTDAllocLayout {
    NSTDAllocLayout { size, align }
}

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
#[inline]
#[nstdapi]
pub const fn nstd_core_alloc_layout_array(
    size: NSTDUInt,
    align: NSTDUInt,
    len: NSTDUInt,
) -> NSTDOptionalAllocLayout {
    #[allow(clippy::arithmetic_side_effects)]
    if crate::core::mem::is_power_of_two(align) && size % align == 0 {
        if let Some(size) = size.checked_mul(len) {
            if size <= NSTD_INT_MAX {
                return NSTDOptional::Some(NSTDAllocLayout { size, align });
            }
        }
    }
    NSTDOptional::None
}

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
#[inline]
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub const unsafe fn nstd_core_alloc_layout_array_unchecked(
    size: NSTDUInt,
    align: NSTDUInt,
    len: NSTDUInt,
) -> NSTDAllocLayout {
    let off = size.wrapping_rem(align);
    let size = size.wrapping_add(align - off);
    let size = size.wrapping_mul(len);
    NSTDAllocLayout { size, align }
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
