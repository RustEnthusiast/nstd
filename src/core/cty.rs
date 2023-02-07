//! Provides functions for examining and operating on character types.
use crate::{NSTDBool, NSTDChar, NSTDChar32};
use nstdapi::nstdapi;

/// Determines whether or not a 32-bit character value is a valid Unicode scalar value.
///
/// # Parameters:
///
/// - `NSTDChar32 chr` - The 32-bit character value to check.
///
/// # Returns
///
/// `NSTDBool is_unicode` - True if `chr` is a valid Unicode character.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::cty::nstd_core_cty_is_unicode, NSTDChar32};
///
/// assert!(nstd_core_cty_is_unicode('🦀' as NSTDChar32));
/// assert!(!nstd_core_cty_is_unicode(NSTDChar32::MAX));
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cty_is_unicode(chr: NSTDChar32) -> NSTDBool {
    char::from_u32(chr).is_some()
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
        #[nstdapi]
        pub fn $name(chr: NSTDChar) -> NSTDBool {
            (chr as u8).$method()
        }
    };
}
gen_deterministic!(
    /// Determines whether or not `chr` is a valid ASCII value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_ascii` - `NSTD_TRUE` if `chr` is a valid ASCII value.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_ascii, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_ascii(b'-' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_ascii(u8::MAX as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_ascii,
    is_ascii
);
gen_deterministic!(
    /// Determines whether or not `chr` is alphabetic.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_alphabetic` - `NSTD_TRUE` if `chr` is alphabetic.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_alphabetic, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_alphabetic(b'G' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_alphabetic(b'0' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_alphabetic,
    is_ascii_alphabetic
);
gen_deterministic!(
    /// Determines whether or not `chr` is numeric.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_numeric` - `NSTD_TRUE` if `chr` is numeric.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_numeric, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_numeric(b'9' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_numeric(b'a' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_numeric,
    is_ascii_digit
);
gen_deterministic!(
    /// Determines whether or not `chr` is alphabetic or numeric.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_alphanumeric` - `NSTD_TRUE` if `chr` is alphabetic or numeric.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_alphanumeric, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_alphanumeric(b'Z' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_alphanumeric(b'5' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_alphanumeric(b';' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_alphanumeric,
    is_ascii_alphanumeric
);
gen_deterministic!(
    /// Determines whether or not `chr` is a hexadecimal digit.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_hexdigit` - `NSTD_TRUE` if `chr` is a hexadecimal digit.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_hexdigit, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_hexdigit(b'0' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_hexdigit(b'F' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_hexdigit(b';' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_hexdigit,
    is_ascii_hexdigit
);
gen_deterministic!(
    /// Determines whether or not `chr` is lowercase.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_lowercase` - `NSTD_TRUE` if `chr` is lowercase.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_lowercase, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_lowercase(b'v' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_lowercase(b'M' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_lowercase,
    is_ascii_lowercase
);
gen_deterministic!(
    /// Determines whether or not `chr` is uppercase.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_uppercase` - `NSTD_TRUE` if `chr` is uppercase.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_uppercase, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_uppercase(b'P' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_uppercase(b's' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_uppercase,
    is_ascii_uppercase
);
gen_deterministic!(
    /// Determines whether or not `chr` is white space.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_whitespace` - `NSTD_TRUE` if `chr` is white space.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_whitespace, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_whitespace(b'\n' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_whitespace(b'.' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_whitespace,
    is_ascii_whitespace
);
gen_deterministic!(
    /// Determines whether or not `chr` is a control character.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_control` - `NSTD_TRUE` if `chr` is a control character.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_control, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_control(b'\0' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_control(b'\\' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_control,
    is_ascii_control
);
gen_deterministic!(
    /// Determines whether or not `chr` is punctuation.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_punctuation` - `NSTD_TRUE` if `chr` is punctuation.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_punctuation, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_punctuation(b'.' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_punctuation(b'y' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_punctuation,
    is_ascii_punctuation
);
gen_deterministic!(
    /// Determines whether or not `chr` is a graphical character.
    ///
    /// # Parameters:
    ///
    /// - `NSTDChar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_graphic` - `NSTD_TRUE` if `chr` is a graphical character.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_graphic, NSTDChar, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_graphic(b'.' as NSTDChar) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_graphic(b'\t' as NSTDChar) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_graphic,
    is_ascii_graphic
);
