#ifndef NSTD_CORE_CTY_H
#define NSTD_CORE_CTY_H
#include "../nstd.h"
NSTDCPPSTART

/// Returns the Unicode replacement character (�).
///
/// # Returns
///
/// `NSTDUnichar replacement_char` - The Unicode replacement character (�).
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
NSTDAPI NSTDBool nstd_core_cty_is_numeric(NSTDUnichar chr);
/// Determines whether or not `chr` is alphanumeric according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_alphanumeric` - `NSTD_TRUE` if `chr` is alphanumeric.
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
NSTDAPI NSTDBool nstd_core_cty_is_control(NSTDUnichar chr);

/// Determines whether or not `chr` is punctuation.
///
/// # Note
///
/// This only works with ASCII characters.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_punctuation` - `NSTD_TRUE` if `chr` is punctuation.
NSTDAPI NSTDBool nstd_core_cty_is_punctuation(NSTDUnichar chr);

/// Determines whether or not `chr` is a graphic character.
///
/// # Note
///
/// This only works with ASCII characters.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_graphic` - `NSTD_TRUE` if `chr` is a graphic character.
NSTDAPI NSTDBool nstd_core_cty_is_graphic(NSTDUnichar chr);

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
NSTDAPI NSTDBool nstd_core_cty_is_digit(NSTDUnichar chr, NSTDUInt32 radix);

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
NSTDAPI NSTDUnichar nstd_core_cty_to_lowercase(NSTDUnichar chr);
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
NSTDAPI NSTDUnichar nstd_core_cty_to_uppercase(NSTDUnichar chr);

NSTDCPPEND
#endif
