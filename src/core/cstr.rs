//! C string processing.
//!
//! # Note
//!
//! The functions in this module must be provided valid C strings, because they do not check for
//! null pointers.
use crate::core::def::{NSTDChar, NSTDUSize};

/// Gets the length of a null terminated C string, excluding the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string, excluding the null byte.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_len(cstr: *const NSTDChar) -> NSTDUSize {
    let mut i = 0;
    while unsafe { *cstr.add(i) } != 0 {
        i += 1;
    }
    i
}

/// Gets the length of a null terminated C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string, including the null byte.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_len_with_null(cstr: *const NSTDChar) -> NSTDUSize {
    nstd_core_cstr_len(cstr) + 1
}
