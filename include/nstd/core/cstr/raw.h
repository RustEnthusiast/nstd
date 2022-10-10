#ifndef NSTD_CORE_CSTR_RAW_H
#define NSTD_CORE_CSTR_RAW_H
#include "../../nstd.h"

/// Gets the length of a null terminated C string, excluding the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string, excluding the null byte.
///
/// # Safety
///
/// This function makes access to raw pointer data, which can cause undefined behavior in the event
/// that `cstr`'s data is invalid.
NSTDAPI NSTDUInt nstd_core_cstr_raw_len(const NSTDChar *cstr);

/// Gets the length of a null terminated C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string, including the null byte.
///
/// # Safety
///
/// This function makes access to raw pointer data, which can cause undefined behavior in the event
/// that `cstr`'s data is invalid.
NSTDAPI NSTDUInt nstd_core_cstr_raw_len_with_null(const NSTDChar *cstr);

/// Compares two C strings, returning `NSTD_TRUE` if they are lexicographically equal.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr1` - The first C string.
///
/// - `const NSTDChar *cstr2` - The second C string.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the C strings are lexicographically equal.
///
/// # Safety
///
/// This function makes access to raw pointer data, which can cause undefined behavior in the event
/// that either `cstr1` or `cstr2`'s data is invalid.
NSTDAPI NSTDBool nstd_core_cstr_raw_compare(const NSTDChar *cstr1, const NSTDChar *cstr2);

/// Copies the contents of `src` to `dest`, excluding the null terminator.
///
/// # Note
///
/// If you already know how many bytes should be copied, `nstd_core_mem_copy[_overlapped]` should
/// be used instead as it can minimize execution times.
///
/// # Parameters:
///
/// - `NSTDChar *dest` - The C string buffer to copy data to.
///
/// - `const NSTDChar *src` - The C string to copy data from.
///
/// # Safety
///
/// This function reads from/writes to raw pointer data, which can cause undefined behavior in the
/// event that either `dest` or `src`'s data is invalid.
NSTDAPI void nstd_core_cstr_raw_copy(NSTDChar *dest, const NSTDChar *src);

/// Copies the contents of `src` to `dest`, including the null terminator.
///
/// # Note
///
/// If you already know how many bytes should be copied, `nstd_core_mem_copy[_overlapped]` should
/// be used instead as it can minimize execution times.
///
/// # Parameters:
///
/// - `NSTDChar *dest` - The C string buffer to copy data to.
///
/// - `const NSTDChar *src` - The C string to copy data from.
///
/// # Safety
///
/// This function reads from/writes to raw pointer data, which can cause undefined behavior in the
/// event that either `dest` or `src`'s data is invalid.
NSTDAPI void nstd_core_cstr_raw_copy_with_null(NSTDChar *dest, const NSTDChar *src);

#endif
