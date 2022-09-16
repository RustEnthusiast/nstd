#ifndef NSTD_CORE_CTY_H
#define NSTD_CORE_CTY_H
#include "../nstd.h"
#include "def.h"

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
NSTDAPI NSTDUnichar nstd_core_cty_replacement_char();

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
NSTDAPI NSTDBool nstd_core_cty_is_alphabetic(NSTDUnichar chr);
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
NSTDAPI NSTDBool nstd_core_cty_is_numeric(NSTDUnichar chr);
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
NSTDAPI NSTDBool nstd_core_cty_is_alphanumeric(NSTDUnichar chr);
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
NSTDAPI NSTDBool nstd_core_cty_is_lowercase(NSTDUnichar chr);
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
NSTDAPI NSTDBool nstd_core_cty_is_uppercase(NSTDUnichar chr);
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
NSTDAPI NSTDBool nstd_core_cty_is_whitespace(NSTDUnichar chr);
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
NSTDAPI NSTDBool nstd_core_cty_is_control(NSTDUnichar chr);

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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cty::nstd_core_cty_is_digit;
///
/// assert!(nstd_core_cty_is_digit('5'.into(), 16) != 0);
/// assert!(nstd_core_cty_is_digit('E'.into(), 16) != 0);
/// assert!(nstd_core_cty_is_digit('F'.into(), 10) == 0);
/// ```
NSTDAPI NSTDBool nstd_core_cty_is_digit(NSTDUnichar chr, NSTDUInt32 radix);

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
/// use nstd_sys::core::{cty::nstd_core_cty_is_punctuation, def::NSTDChar};
///
/// assert!(nstd_core_cty_is_punctuation(b'.' as NSTDChar) != 0);
/// assert!(nstd_core_cty_is_punctuation(b'y' as NSTDChar) == 0);
/// ```
NSTDAPI NSTDBool nstd_core_cty_is_punctuation(NSTDChar chr);

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
/// use nstd_sys::core::{cty::nstd_core_cty_is_graphic, def::NSTDChar};
///
/// assert!(nstd_core_cty_is_graphic(b'.' as NSTDChar) != 0);
/// assert!(nstd_core_cty_is_graphic(b'\t' as NSTDChar) == 0);
/// ```
NSTDAPI NSTDBool nstd_core_cty_is_graphic(NSTDChar chr);

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
/// use nstd_sys::core::{cty::nstd_core_cty_to_lowercase, def::NSTDChar};
///
/// let a = char::from_u32(nstd_core_cty_to_lowercase('A'.into())).unwrap();
/// let z = char::from_u32(nstd_core_cty_to_lowercase('Z'.into())).unwrap();
/// assert!(a == 'a');
/// assert!(z == 'z');
/// ```
NSTDAPI NSTDUnichar nstd_core_cty_to_lowercase(NSTDUnichar chr);
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
/// use nstd_sys::core::{cty::nstd_core_cty_to_uppercase, def::NSTDChar};
///
/// let a = char::from_u32(nstd_core_cty_to_uppercase('a'.into())).unwrap();
/// let z = char::from_u32(nstd_core_cty_to_uppercase('z'.into())).unwrap();
/// assert!(a == 'A');
/// assert!(z == 'Z');
/// ```
NSTDAPI NSTDUnichar nstd_core_cty_to_uppercase(NSTDUnichar chr);

#endif
