//! Provides functions for examining and operating on character types.
//!
//! # Note
//!
//! Functions in this module that return a boolean will always return false on error.
use crate::{NSTDBool, NSTDUInt32, NSTDUnichar, NSTD_FALSE};

/// Returns the Unicode replacement character (�).
///
/// # Returns
///
/// `NSTDUnichar replacement_char` - The Unicode replacement character (�).
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_replacement_char() -> NSTDUnichar {
    char::REPLACEMENT_CHARACTER.into()
}

/// Generates deterministic functions such as `is_alphabetic` or `is_numeric`.
macro_rules! gen_deterministic {
    (
        $(#[$meta:meta])*
        $name: ident,
        $method: ident
    ) => {
        $(#[$meta])*
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $name(chr: NSTDUnichar) -> NSTDBool {
            match char::from_u32(chr) {
                Some(chr) => chr.$method().into(),
                _ => NSTD_FALSE,
            }
        }
    };
}
gen_deterministic!(
    /// Determines whether or not `chr` is alphabetic according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_alphabetic` - `NSTD_TRUE` if `chr` is alphabetic.
    nstd_core_cty_is_alphabetic,
    is_alphabetic
);
gen_deterministic!(
    /// Determines whether or not `chr` is numeric according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_numeric` - `NSTD_TRUE` if `chr` is numeric.
    nstd_core_cty_is_numeric,
    is_numeric
);
gen_deterministic!(
    /// Determines whether or not `chr` is alphanumeric according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_alphanumeric` - `NSTD_TRUE` if `chr` is alphanumeric.
    nstd_core_cty_is_alphanumeric,
    is_alphanumeric
);
gen_deterministic!(
    /// Determines whether or not `chr` is lowercase according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_lowercase` - `NSTD_TRUE` if `chr` is lowercase.
    nstd_core_cty_is_lowercase,
    is_lowercase
);
gen_deterministic!(
    /// Determines whether or not `chr` is uppercase according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_uppercase` - `NSTD_TRUE` if `chr` is uppercase.
    nstd_core_cty_is_uppercase,
    is_uppercase
);
gen_deterministic!(
    /// Determines whether or not `chr` is white space according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_whitespace` - `NSTD_TRUE` if `chr` is white space.
    nstd_core_cty_is_whitespace,
    is_whitespace
);
gen_deterministic!(
    /// Determines whether or not `chr` is a control character according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_control` - `NSTD_TRUE` if `chr` is a control character.
    nstd_core_cty_is_control,
    is_control
);

/// Determines whether or not `chr` is a digit, depending on `radix`.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// - `NSTDUInt32 radix` - The base.
///
/// # Returns
///
/// `NSTDBool is_digit` - `NSTD_TRUE` if `chr` is a digit.
///
/// # Panics
///
/// This operation will panic if `radix` is larger than 36.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_is_digit(chr: NSTDUnichar, radix: NSTDUInt32) -> NSTDBool {
    match char::from_u32(chr) {
        Some(chr) => chr.is_digit(radix).into(),
        _ => NSTD_FALSE,
    }
}

/// Returns the lowercase version of `chr` or `chr` if there is no lowercase version.
///
/// # Note
///
/// This only works on ASCII characters.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to convert to lowercase.
///
/// # Returns
///
/// `NSTDUnichar lowercase` - The lowercase version of `chr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_to_lowercase(chr: NSTDUnichar) -> NSTDUnichar {
    match char::from_u32(chr) {
        Some(chr) => chr.to_ascii_lowercase().into(),
        _ => chr,
    }
}
/// Returns the uppercase version of `chr` or `chr` if there is no uppercase version.
///
/// # Note
///
/// This only works on ASCII characters.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to convert to uppercase.
///
/// # Returns
///
/// `NSTDUnichar uppercase` - The uppercase version of `chr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_to_uppercase(chr: NSTDUnichar) -> NSTDUnichar {
    match char::from_u32(chr) {
        Some(chr) => chr.to_ascii_uppercase().into(),
        _ => chr,
    }
}
