//! A dynamically sized, null terminated, C string.
use crate::{
    core::cstr::{nstd_core_cstr_const_new, nstd_core_cstr_new, NSTDCStr, NSTDCStrConst},
    vec::{nstd_vec_clone, nstd_vec_free, nstd_vec_new, nstd_vec_new_with_cap, NSTDVec},
    NSTDUSize,
};

/// A dynamically sized, null terminated, C string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCString {
    /// The underlying vector of `NSTDChar`s.
    pub bytes: NSTDVec,
}

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_new() -> NSTDCString {
    NSTDCString {
        bytes: nstd_vec_new(1),
    }
}

/// Creates a new `NSTDCString` initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUSize cap` - The number of bytes to preallocate.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if `cap` is zero.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_new_with_cap(cap: NSTDUSize) -> NSTDCString {
    NSTDCString {
        bytes: nstd_vec_new_with_cap(1, cap),
    }
}

/// Creates a deep copy of an `NSTDCString`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string to create a deep copy of.
///
/// # Returns
///
/// `NSTDCString cloned` - A new deep copy of `cstring`.
///
/// # Panics
///
/// This function will panic if allocating for the new C string fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_clone(cstring: &NSTDCString) -> NSTDCString {
    NSTDCString {
        bytes: nstd_vec_clone(&cstring.bytes),
    }
}

/// Creates a C string slice containing the contents of `cstring` (excluding the null byte).
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
///
/// # Safety
///
/// `cstring`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_cstring_as_cstr(cstring: &mut NSTDCString) -> NSTDCStr {
    nstd_core_cstr_new(
        cstring.bytes.buffer.ptr.raw.cast(),
        cstring.bytes.len.saturating_sub(1),
    )
}

/// Creates a C string slice containing the contents of `cstring` (excluding the null byte).
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice.
///
/// # Safety
///
/// `cstring`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_cstring_as_cstr_const(cstring: &NSTDCString) -> NSTDCStrConst {
    nstd_core_cstr_const_new(
        cstring.bytes.buffer.ptr.raw.cast(),
        cstring.bytes.len.saturating_sub(1),
    )
}

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_free(cstring: &mut NSTDCString) {
    nstd_vec_free(&mut cstring.bytes);
}
