#ifndef NSTD_CORE_UNICHAR_H
#define NSTD_CORE_UNICHAR_H
#include "../nstd.h"
#include "optional.h"

/// Represents a unicode scalar value.
typedef struct {
    /// The 32-bit value.
    NSTDChar32 value;
} NSTDUnichar;

/// Represents an optional value of type `NSTDUnichar`.
NSTDOptional(NSTDUnichar) NSTDOptionalUnichar;

/// Creates a new `NSTDUnichar` from a 32-bit character value.
///
/// # Parameters:
///
/// - `NSTDChar32 value` - The 32-bit character to be converted into an `NSTDUnichar`.
///
/// # Returns
///
/// `NSTDOptionalUnichar unichar` - The new Unicode scalar value on success.
NSTDAPI NSTDOptionalUnichar nstd_core_unichar_new(NSTDChar32 value);

/// Returns the Unicode replacement character (�).
///
/// # Returns
///
/// `NSTDUnichar replacement_char` - The Unicode replacement character (�).
NSTDAPI NSTDUnichar nstd_core_unichar_replacement(void);

/// Determines whether or not `chr` is an ASCII character.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_ascii` - `NSTD_TRUE` if `chr` is an ASCII character.
NSTDAPI NSTDBool nstd_core_unichar_is_ascii(NSTDUnichar chr);

/// Determines whether or not `chr` is alphabetic according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_alphabetic` - `NSTD_TRUE` if `chr` is alphabetic.
NSTDAPI NSTDBool nstd_core_unichar_is_alphabetic(NSTDUnichar chr);

/// Determines whether or not `chr` is numeric according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_numeric` - `NSTD_TRUE` if `chr` is numeric.
NSTDAPI NSTDBool nstd_core_unichar_is_numeric(NSTDUnichar chr);

/// Determines whether or not `chr` is alphabetic or numeric according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_alphanumeric` - `NSTD_TRUE` if `chr` is alphabetic or numeric.
NSTDAPI NSTDBool nstd_core_unichar_is_alphanumeric(NSTDUnichar chr);

/// Determines whether or not `chr` is lowercase according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_lowercase` - `NSTD_TRUE` if `chr` is lowercase.
NSTDAPI NSTDBool nstd_core_unichar_is_lowercase(NSTDUnichar chr);

/// Determines whether or not `chr` is uppercase according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_uppercase` - `NSTD_TRUE` if `chr` is uppercase.
NSTDAPI NSTDBool nstd_core_unichar_is_uppercase(NSTDUnichar chr);

/// Determines whether or not `chr` is white space according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_whitespace` - `NSTD_TRUE` if `chr` is white space.
NSTDAPI NSTDBool nstd_core_unichar_is_whitespace(NSTDUnichar chr);

/// Determines whether or not `chr` is a control character according to the Unicode standard.
///
/// # Parameters:
///
/// - `NSTDUnichar chr` - The character to check.
///
/// # Returns
///
/// `NSTDBool is_control` - `NSTD_TRUE` if `chr` is a control character.
NSTDAPI NSTDBool nstd_core_unichar_is_control(NSTDUnichar chr);

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
NSTDAPI NSTDBool nstd_core_unichar_is_digit(NSTDUnichar chr, NSTDUInt32 radix);

#endif
