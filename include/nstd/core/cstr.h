#ifndef NSTD_CORE_CSTR_H_INCLUDED
#define NSTD_CORE_CSTR_H_INCLUDED
#include "../nstd.h"
#include "def.h"
NSTDCPPSTART

/// Gets the length of a null terminated C string, excluding the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string, excluding the null byte.
NSTDAPI NSTDUSize nstd_core_cstr_len(const NSTDChar *cstr);

/// Gets the length of a null terminated C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string, including the null byte.
NSTDAPI NSTDUSize nstd_core_cstr_len_with_null(const NSTDChar *cstr);

/// Compares two C strings, returning `NSTD_BOOL_TRUE` if they are lexicographically equal.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr1` - The first C string.
///
/// - `const NSTDChar *cstr2` - The second C string.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the C strings are lexicographically equal.
NSTDAPI NSTDBool nstd_core_cstr_compare(const NSTDChar *cstr1, const NSTDChar *cstr2);

NSTDCPPEND
#endif
