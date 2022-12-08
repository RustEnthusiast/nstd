//! Provides useful utilities for working with raw pointers.
use crate::{NSTDAny, NSTDAnyMut};

/// The default alignment suitable for any scalar type.
///
/// Corresponds to `alignof(max_align_t)`.
/// The C/C++ standards specify that this value should be at least 8 or 16, I'm going with 16 for
/// safety but of course this is platform dependent so if you (the reader) know of a platform that
/// this value is smaller (or larger for that matter) on, please submit an issue/pull request.
pub(crate) const MAX_ALIGN: usize = 16;

/// Creates a new dangling immutable pointer with valid alignment for any scalar type.
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
/// let slice = nstd_core_slice_new(nstd_core_ptr_raw_dangling(), 1, 0);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_raw_dangling() -> NSTDAny {
    MAX_ALIGN as NSTDAny
}

/// Creates a new dangling mutable pointer with valid alignment for any scalar type.
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
/// let slice = nstd_core_slice_mut_new(nstd_core_ptr_raw_dangling_mut(), 1, 0);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_raw_dangling_mut() -> NSTDAnyMut {
    MAX_ALIGN as NSTDAnyMut
}
