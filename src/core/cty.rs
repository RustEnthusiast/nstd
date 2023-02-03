//! Provides functions for examining and operating on character types.
use crate::{core::unichar::NSTDUnichar, NSTDBool, NSTDChar, NSTDChar32, NSTDUInt32};
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

/// Returns the Unicode replacement character (�).
///
/// # Returns
///
/// `NSTDUnichar replacement_char` - The Unicode replacement character (�).
///
/// # Example
///
/// ```
/// use nstd_sys::core::cty::nstd_core_cty_replacement_char;
///
/// assert!(nstd_core_cty_replacement_char() == char::REPLACEMENT_CHARACTER.into());
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cty_replacement_char() -> NSTDUnichar {
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
        #[nstdapi]
        pub fn $name(chr: NSTDUnichar) -> NSTDBool {
            char::from(chr).$method()
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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_alphabetic, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_alphabetic('G'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_alphabetic('0'.into()) == NSTD_FALSE);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_numeric, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_numeric('9'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_numeric('a'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_numeric,
    is_numeric
);
gen_deterministic!(
    /// Determines whether or not `chr` is alphabetic or numeric according to the Unicode standard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_alphanumeric` - `NSTD_TRUE` if `chr` is alphabetic or numeric.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_alphanumeric, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_alphanumeric('Z'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_alphanumeric('5'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_alphanumeric(';'.into()) == NSTD_FALSE);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_lowercase, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_lowercase('v'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_lowercase('M'.into()) == NSTD_FALSE);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_uppercase, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_uppercase('P'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_uppercase('s'.into()) == NSTD_FALSE);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_whitespace, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_whitespace('\n'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_whitespace('.'.into()) == NSTD_FALSE);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::cty::nstd_core_cty_is_control, NSTD_FALSE};
    ///
    /// assert!(nstd_core_cty_is_control('\0'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_cty_is_control('\\'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_cty_is_control,
    is_control
);

/// Determines whether or not `chr` is a digit, depending on `radix`.
///
/// # Note
///
/// This will always return false when given a base greater than 36.
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
/// This function will panic if `radix` is greater than 36.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::cty::nstd_core_cty_is_digit, NSTD_FALSE};
///
/// assert!(nstd_core_cty_is_digit('5'.into(), 16) != NSTD_FALSE);
/// assert!(nstd_core_cty_is_digit('E'.into(), 16) != NSTD_FALSE);
/// assert!(nstd_core_cty_is_digit('F'.into(), 10) == NSTD_FALSE);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cty_is_digit(chr: NSTDUnichar, radix: NSTDUInt32) -> NSTDBool {
    char::from(chr).is_digit(radix)
}

/// Determines whether or not `chr` is punctuation.
///
/// # Note
///
/// This only works with ASCII characters.
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
/// use nstd_sys::{core::cty::nstd_core_cty_is_ascii_punctuation, NSTDChar, NSTD_FALSE};
///
/// assert!(nstd_core_cty_is_ascii_punctuation(b'.' as NSTDChar) != NSTD_FALSE);
/// assert!(nstd_core_cty_is_ascii_punctuation(b'y' as NSTDChar) == NSTD_FALSE);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cty_is_ascii_punctuation(chr: NSTDChar) -> NSTDBool {
    matches!(chr, 0x21..=0x2F | 0x3A..=0x40 | 0x5B..=0x60 | 0x7B..=0x7E)
}

/// Determines whether or not `chr` is a graphic character.
///
/// # Note
///
/// This only works with ASCII characters.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_graphic` - `NSTD_TRUE` if `chr` is a graphic character.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::cty::nstd_core_cty_is_ascii_graphic, NSTDChar, NSTD_FALSE};
///
/// assert!(nstd_core_cty_is_ascii_graphic(b'.' as NSTDChar) != NSTD_FALSE);
/// assert!(nstd_core_cty_is_ascii_graphic(b'\t' as NSTDChar) == NSTD_FALSE);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cty_is_ascii_graphic(chr: NSTDChar) -> NSTDBool {
    matches!(chr, 0x21..=0x7E)
}

/// Returns the lowercase version of `chr`, or `chr` if there is no lowercase version.
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::{cty::nstd_core_cty_to_ascii_lowercase, unichar::nstd_core_unichar_new};
///
/// let a = nstd_core_unichar_new('A'.into()).unwrap();
/// let z = nstd_core_unichar_new('Z'.into()).unwrap();
/// let a = char::from(nstd_core_cty_to_ascii_lowercase(a));
/// let z = char::from(nstd_core_cty_to_ascii_lowercase(z));
/// assert!(a == 'a');
/// assert!(z == 'z');
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cty_to_ascii_lowercase(chr: NSTDUnichar) -> NSTDUnichar {
    char::from(chr).to_ascii_lowercase().into()
}
/// Returns the uppercase version of `chr`, or `chr` if there is no uppercase version.
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::{cty::nstd_core_cty_to_ascii_uppercase, unichar::nstd_core_unichar_new};
///
/// let a = nstd_core_unichar_new('a'.into()).unwrap();
/// let z = nstd_core_unichar_new('z'.into()).unwrap();
/// let a = char::from(nstd_core_cty_to_ascii_uppercase(a));
/// let z = char::from(nstd_core_cty_to_ascii_uppercase(z));
/// assert!(a == 'A');
/// assert!(z == 'Z');
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cty_to_ascii_uppercase(chr: NSTDUnichar) -> NSTDUnichar {
    char::from(chr).to_ascii_uppercase().into()
}
