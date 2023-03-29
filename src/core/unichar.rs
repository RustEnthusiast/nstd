//! A Unicode scalar value.
//!
//! This is a structure that wraps an `NSTDChar32` (Rust's `char` primitive is not FFI safe). This
//! is done so that an `NSTDUnichar` can be created once and used a number of times without
//! worrying about Unicode validity.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDBool, NSTDChar32, NSTDUInt32,
};
use nstdapi::nstdapi;

/// Represents a unicode scalar value.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDUnichar {
    /// The 32-bit value.
    value: NSTDChar32,
}
impl From<char> for NSTDUnichar {
    /// Converts a Rust `char` into an `NSTDUnichar`.
    #[inline]
    fn from(value: char) -> Self {
        Self { value: value as _ }
    }
}
impl From<NSTDUnichar> for char {
    /// Converts an `NSTDUnichar` into a Rust `char`.
    #[inline]
    fn from(value: NSTDUnichar) -> Self {
        // SAFETY: `value` is always a valid Unicode scalar value.
        unsafe { char::from_u32_unchecked(value.value) }
    }
}
gen_optional!(NSTDOptionalUnichar, NSTDUnichar);

/// Creates a new `NSTDUnichar` from a 32-bit character value.
///
/// # Parameters:
///
/// - `NSTDChar32 value` - The 32-bit character to be converted into an `NSTDUnichar`.
///
/// # Returns
///
/// `NSTDOptionalUnichar unichar` - The new Unicode scalar value on success.
#[inline]
#[nstdapi]
pub const fn nstd_core_unichar_new(value: NSTDChar32) -> NSTDOptionalUnichar {
    match char::from_u32(value) {
        Some(_) => NSTDOptional::Some(NSTDUnichar { value }),
        _ => NSTDOptional::None,
    }
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
/// use nstd_sys::core::unichar::nstd_core_unichar_replacement;
///
/// assert!(nstd_core_unichar_replacement() == char::REPLACEMENT_CHARACTER.into());
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_unichar_replacement() -> NSTDUnichar {
    NSTDUnichar {
        value: char::REPLACEMENT_CHARACTER as _,
    }
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
    /// Determines whether or not `chr` is an ASCII character.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUnichar chr` - The character to check.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_ascii` - `NSTD_TRUE` if `chr` is an ASCII character.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_ascii, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_ascii('='.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_ascii('💯'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_ascii,
    is_ascii
);
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_alphabetic, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_alphabetic('G'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_alphabetic('0'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_alphabetic,
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_numeric, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_numeric('9'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_numeric('a'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_numeric,
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_alphanumeric, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_alphanumeric('Z'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_alphanumeric('5'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_alphanumeric(';'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_alphanumeric,
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_lowercase, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_lowercase('v'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_lowercase('M'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_lowercase,
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_uppercase, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_uppercase('P'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_uppercase('s'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_uppercase,
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_whitespace, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_whitespace('\n'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_whitespace('.'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_whitespace,
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
    /// use nstd_sys::{core::unichar::nstd_core_unichar_is_control, NSTD_FALSE};
    ///
    /// assert!(nstd_core_unichar_is_control('\0'.into()) != NSTD_FALSE);
    /// assert!(nstd_core_unichar_is_control('\\'.into()) == NSTD_FALSE);
    /// ```
    nstd_core_unichar_is_control,
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
/// # Example
///
/// ```
/// use nstd_sys::{core::unichar::nstd_core_unichar_is_digit, NSTD_FALSE};
///
/// assert!(nstd_core_unichar_is_digit('5'.into(), 16) != NSTD_FALSE);
/// assert!(nstd_core_unichar_is_digit('E'.into(), 16) != NSTD_FALSE);
/// assert!(nstd_core_unichar_is_digit('F'.into(), 10) == NSTD_FALSE);
/// assert!(nstd_core_unichar_is_digit('0'.into(), 37) == NSTD_FALSE);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_unichar_is_digit(chr: NSTDUnichar, radix: NSTDUInt32) -> NSTDBool {
    match radix <= 36 {
        true => char::from(chr).is_digit(radix),
        false => false,
    }
}
