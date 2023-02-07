#ifndef NSTD_CORE_CTY_H
#define NSTD_CORE_CTY_H
#include "../nstd.h"

/// Determines whether or not a 32-bit character value is a valid Unicode scalar value.
///
/// # Parameters:
///
/// - `NSTDChar32 chr` - The 32-bit character value to check.
///
/// # Returns
///
/// `NSTDBool is_unicode` - True if `chr` is a valid Unicode character.
NSTDAPI NSTDBool nstd_core_cty_is_unicode(NSTDChar32 chr);

/// Determines whether or not `chr` is a valid ASCII value.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_ascii` - `NSTD_TRUE` if `chr` is a valid ASCII value.
NSTDAPI NSTDBool nstd_core_cty_is_ascii(NSTDChar chr);

/// Determines whether or not `chr` is alphabetic.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_alphabetic` - `NSTD_TRUE` if `chr` is alphabetic.
NSTDAPI NSTDBool nstd_core_cty_is_alphabetic(NSTDChar chr);

/// Determines whether or not `chr` is numeric.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_numeric` - `NSTD_TRUE` if `chr` is numeric.
NSTDAPI NSTDBool nstd_core_cty_is_numeric(NSTDChar chr);

/// Determines whether or not `chr` is alphabetic or numeric.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_alphanumeric` - `NSTD_TRUE` if `chr` is alphabetic or numeric.
NSTDAPI NSTDBool nstd_core_cty_is_alphanumeric(NSTDChar chr);

/// Determines whether or not `chr` is a hexadecimal digit.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_hexdigit` - `NSTD_TRUE` if `chr` is a hexadecimal digit.
NSTDAPI NSTDBool nstd_core_cty_is_hexdigit(NSTDChar chr);

/// Determines whether or not `chr` is lowercase.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_lowercase` - `NSTD_TRUE` if `chr` is lowercase.
NSTDAPI NSTDBool nstd_core_cty_is_lowercase(NSTDChar chr);

/// Determines whether or not `chr` is uppercase.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_uppercase` - `NSTD_TRUE` if `chr` is uppercase.
NSTDAPI NSTDBool nstd_core_cty_is_uppercase(NSTDChar chr);

/// Determines whether or not `chr` is white space.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_whitespace` - `NSTD_TRUE` if `chr` is white space.
NSTDAPI NSTDBool nstd_core_cty_is_whitespace(NSTDChar chr);

/// Determines whether or not `chr` is a control character.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_control` - `NSTD_TRUE` if `chr` is a control character.
NSTDAPI NSTDBool nstd_core_cty_is_control(NSTDChar chr);

/// Determines whether or not `chr` is punctuation.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_punctuation` - `NSTD_TRUE` if `chr` is punctuation.
NSTDAPI NSTDBool nstd_core_cty_is_punctuation(NSTDChar chr);

/// Determines whether or not `chr` is a graphical character.
///
/// # Parameters:
///
/// - `NSTDChar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_graphic` - `NSTD_TRUE` if `chr` is a graphical character.
NSTDAPI NSTDBool nstd_core_cty_is_graphic(NSTDChar chr);

#endif
