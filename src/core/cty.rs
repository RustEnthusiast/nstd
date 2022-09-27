//! Provides functions for examining and operating on character types.
use crate::{NSTDBool, NSTDChar, NSTDUInt32, NSTDUnichar, NSTD_FALSE};

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
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::cty::nstd_core_cty_is_alphabetic;
    ///
    /// assert!(nstd_core_cty_is_alphabetic('G'.into()) != 0);
    /// assert!(nstd_core_cty_is_alphabetic('0'.into()) == 0);
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
    /// use nstd_sys::core::cty::nstd_core_cty_is_numeric;
    ///
    /// assert!(nstd_core_cty_is_numeric('9'.into()) != 0);
    /// assert!(nstd_core_cty_is_numeric('a'.into()) == 0);
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
    /// use nstd_sys::core::cty::nstd_core_cty_is_alphanumeric;
    ///
    /// assert!(nstd_core_cty_is_alphanumeric('Z'.into()) != 0);
    /// assert!(nstd_core_cty_is_alphanumeric('5'.into()) != 0);
    /// assert!(nstd_core_cty_is_alphanumeric(';'.into()) == 0);
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
    /// use nstd_sys::core::cty::nstd_core_cty_is_lowercase;
    ///
    /// assert!(nstd_core_cty_is_lowercase('v'.into()) != 0);
    /// assert!(nstd_core_cty_is_lowercase('M'.into()) == 0);
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
    /// use nstd_sys::core::cty::nstd_core_cty_is_uppercase;
    ///
    /// assert!(nstd_core_cty_is_uppercase('P'.into()) != 0);
    /// assert!(nstd_core_cty_is_uppercase('s'.into()) == 0);
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
    /// use nstd_sys::core::cty::nstd_core_cty_is_whitespace;
    ///
    /// assert!(nstd_core_cty_is_whitespace('\n'.into()) != 0);
    /// assert!(nstd_core_cty_is_whitespace('.'.into()) == 0);
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
    /// use nstd_sys::core::cty::nstd_core_cty_is_control;
    ///
    /// assert!(nstd_core_cty_is_control('\0'.into()) != 0);
    /// assert!(nstd_core_cty_is_control('\\'.into()) == 0);
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
/// # Example
///
/// ```
/// use nstd_sys::core::cty::nstd_core_cty_is_digit;
///
/// assert!(nstd_core_cty_is_digit('5'.into(), 16) != 0);
/// assert!(nstd_core_cty_is_digit('E'.into(), 16) != 0);
/// assert!(nstd_core_cty_is_digit('F'.into(), 10) == 0);
/// assert!(nstd_core_cty_is_digit('9'.into(), 37) == 0);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_is_digit(chr: NSTDUnichar, radix: NSTDUInt32) -> NSTDBool {
    if radix <= 36 {
        if let Some(chr) = char::from_u32(chr) {
            return chr.is_digit(radix).into();
        }
    }
    NSTD_FALSE
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
/// use nstd_sys::{core::cty::nstd_core_cty_is_ascii_punctuation, NSTDChar};
///
/// assert!(nstd_core_cty_is_ascii_punctuation(b'.' as NSTDChar) != 0);
/// assert!(nstd_core_cty_is_ascii_punctuation(b'y' as NSTDChar) == 0);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_is_ascii_punctuation(chr: NSTDChar) -> NSTDBool {
    ((0x21..=0x2F).contains(&chr)
        || (0x3A..=0x40).contains(&chr)
        || (0x5B..=0x60).contains(&chr)
        || (0x7B..=0x7E).contains(&chr))
    .into()
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
/// use nstd_sys::{core::cty::nstd_core_cty_is_ascii_graphic, NSTDChar};
///
/// assert!(nstd_core_cty_is_ascii_graphic(b'.' as NSTDChar) != 0);
/// assert!(nstd_core_cty_is_ascii_graphic(b'\t' as NSTDChar) == 0);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_is_ascii_graphic(chr: NSTDChar) -> NSTDBool {
    (0x21..=0x7E).contains(&chr).into()
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
/// use nstd_sys::{core::cty::nstd_core_cty_to_ascii_lowercase, NSTDChar};
///
/// let a = char::from_u32(nstd_core_cty_to_ascii_lowercase('A'.into())).unwrap();
/// let z = char::from_u32(nstd_core_cty_to_ascii_lowercase('Z'.into())).unwrap();
/// assert!(a == 'a');
/// assert!(z == 'z');
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_to_ascii_lowercase(chr: NSTDUnichar) -> NSTDUnichar {
    match char::from_u32(chr) {
        Some(chr) => chr.to_ascii_lowercase().into(),
        _ => chr,
    }
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
/// use nstd_sys::{core::cty::nstd_core_cty_to_ascii_uppercase, NSTDChar};
///
/// let a = char::from_u32(nstd_core_cty_to_ascii_uppercase('a'.into())).unwrap();
/// let z = char::from_u32(nstd_core_cty_to_ascii_uppercase('z'.into())).unwrap();
/// assert!(a == 'A');
/// assert!(z == 'Z');
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cty_to_ascii_uppercase(chr: NSTDUnichar) -> NSTDUnichar {
    match char::from_u32(chr) {
        Some(chr) => chr.to_ascii_uppercase().into(),
        _ => chr,
    }
}
