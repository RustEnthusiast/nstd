//! Provides useful utilities for working with raw pointers.
use crate::{NSTDAny, NSTDAnyMut, NSTDBool, NSTDUInt};
use nstdapi::nstdapi;

/// The default alignment suitable for any scalar type.
///
/// Corresponds to `alignof(max_align_t)`.
/// The C/C++ standards specify that this value should be at least 8 or 16, I'm going with 16 for
/// safety but of course this is platform dependent so if you (the reader) know of a platform that
/// this value is smaller (or larger for that matter) on, please submit an issue/pull request.
pub(crate) const MAX_ALIGN: usize = 16;

/// Checks if `align` is a power of 2.
#[inline]
#[allow(clippy::arithmetic_side_effects)]
const fn is_power_of_two(align: NSTDUInt) -> NSTDBool {
    (align != 0) && ((align & (align - 1)) == 0)
}

/// Creates a new dangling pointer to some immutable memory. The pointer is guaranteed to have valid
/// alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAny dangling` - The new dangling raw pointer.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{ptr::raw::nstd_core_ptr_raw_dangling, slice::nstd_core_slice_new};
///
/// let slice = unsafe { nstd_core_slice_new(nstd_core_ptr_raw_dangling(), 1, 0).unwrap() };
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_raw_dangling() -> NSTDAny {
    MAX_ALIGN as NSTDAny
}

/// Creates a new dangling pointer to some mutable memory. The pointer is guaranteed to have valid
/// alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAnyMut dangling` - The new dangling raw pointer.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{ptr::raw::nstd_core_ptr_raw_dangling_mut, slice::nstd_core_slice_mut_new};
///
/// let slice = unsafe { nstd_core_slice_mut_new(nstd_core_ptr_raw_dangling_mut(), 1, 0).unwrap() };
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_raw_dangling_mut() -> NSTDAnyMut {
    MAX_ALIGN as NSTDAnyMut
}

/// Returns a pointer that is properly aligned to `align` based on the offset `ptr`.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - The pointer to align.
///
/// - `NSTDUInt align` - The alignment requirement. This must be a power of two.
///
/// # Returns
///
/// `NSTDAny aligned` - The properly aligned pointer.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two or overflow occurs.
///
/// # Safety
///
/// Both `ptr` and the resulting pointer must be either in bounds or one byte past the end of the
/// same allocated object.
///
/// # Example
///
/// ```
/// use nstd_sys::core::ptr::raw::{nstd_core_ptr_raw_align, nstd_core_ptr_raw_is_aligned};
///
/// unsafe {
///     let ptr = 2 as _;
///     let aligned = nstd_core_ptr_raw_align(ptr, 16);
///     assert!(nstd_core_ptr_raw_is_aligned(aligned, 16));
/// }
/// ```
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub unsafe fn nstd_core_ptr_raw_align(ptr: NSTDAny, align: NSTDUInt) -> NSTDAny {
    assert!(is_power_of_two(align));
    ((ptr as NSTDUInt)
        .checked_add(align - 1)
        .expect("pointer arithmetic should not overflow")
        & !(align - 1)) as NSTDAny
}

/// Returns a pointer that is properly aligned to `align` based on the offset `ptr`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - The pointer to align.
///
/// - `NSTDUInt align` - The alignment requirement. This must be a power of two.
///
/// # Returns
///
/// `NSTDAnyMut aligned` - The properly aligned pointer.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two or overflow occurs.
///
/// # Safety
///
/// Both `ptr` and the resulting pointer must be either in bounds or one byte past the end of the
/// same allocated object.
///
/// # Example
///
/// ```
/// use nstd_sys::core::ptr::raw::{nstd_core_ptr_raw_align_mut, nstd_core_ptr_raw_is_aligned};
///
/// unsafe {
///     let ptr = 2 as _;
///     let aligned = nstd_core_ptr_raw_align_mut(ptr, 16);
///     assert!(nstd_core_ptr_raw_is_aligned(aligned, 16));
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_ptr_raw_align_mut(ptr: NSTDAnyMut, align: NSTDUInt) -> NSTDAnyMut {
    nstd_core_ptr_raw_align(ptr, align).cast_mut()
}

/// Checks if `ptr` is aligned to `align`.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - The pointer to check.
///
/// - `NSTDUInt align` - The alignment to check for. This must be a power of two.
///
/// # Returns
///
/// `NSTDBool is_aligned` - `NSTD_TRUE` if the pointer is aligned to `align`.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::ptr::raw::{nstd_core_ptr_raw_align, nstd_core_ptr_raw_is_aligned},
///     NSTDAny,
/// };
///
/// unsafe {
///     let mut a = 1usize as NSTDAny;
///     a = nstd_core_ptr_raw_align(a, 8);
///     assert!(!nstd_core_ptr_raw_is_aligned(a, 16));
///     a = nstd_core_ptr_raw_align(a, 16);
///     assert!(nstd_core_ptr_raw_is_aligned(a, 16));
/// }
/// ```
#[inline]
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub fn nstd_core_ptr_raw_is_aligned(ptr: NSTDAny, align: NSTDUInt) -> NSTDBool {
    assert!(is_power_of_two(align));
    ptr as NSTDUInt & (align - 1) == 0
}
