//! Provides useful types for memory allocation support.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_INT_MAX,
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

/// Describes an error returned from allocation functions.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDAllocError {
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
    NSTD_ALLOC_ERROR_INVALID_LAYOUT,
}

/// A structure of function pointers making up an allocator's virtual function table.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDAllocator {
    /// An opaque pointer to the allocator's state.
    pub state: NSTDAny,
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
    pub allocate: unsafe extern "C" fn(NSTDAny, NSTDAllocLayout) -> NSTDAnyMut,
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
    pub allocate_zeroed: unsafe extern "C" fn(NSTDAny, NSTDAllocLayout) -> NSTDAnyMut,
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
    pub reallocate: unsafe extern "C" fn(
        NSTDAny,
        &mut NSTDAnyMut,
        NSTDAllocLayout,
        NSTDAllocLayout,
    ) -> NSTDAllocError,
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
    pub deallocate: unsafe extern "C" fn(NSTDAny, NSTDAnyMut, NSTDAllocLayout) -> NSTDAllocError,
}
/// # Safety
///
/// The allocator's state must be able to be safely *shared* between threads.
// SAFETY: The user guarantees that the state is thread-safe.
unsafe impl Send for NSTDAllocator {}
/// # Safety
///
/// The allocator's state must be able to be safely shared between threads.
// SAFETY: The user guarantees that the state is thread-safe.
unsafe impl Sync for NSTDAllocator {}

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
    NSTDAllocLayout {
        size: size.wrapping_mul(len),
        align,
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
