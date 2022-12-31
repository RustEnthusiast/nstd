#ifndef NSTD_CORE_CSTR_RAW_H
#define NSTD_CORE_CSTR_RAW_H
#include "../../nstd.h"

/// Gets the length of a raw null terminated C string, excluding the null-terminator.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string, excluding the null-terminator.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDUInt nstd_core_cstr_raw_len(const NSTDChar *cstr);

/// Gets the length of a raw null terminated C string, including the null-terminator.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string, including the null-terminator.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDUInt nstd_core_cstr_raw_len_with_null(const NSTDChar *cstr);

/// Compares two raw null-terminated C strings, returning `NSTD_TRUE` if they are lexicographically
/// equal.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr1` - The first C string.
///
/// - `const NSTDChar *cstr2` - The second C string.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two C strings are lexicographically equal.
///
/// # Safety
///
/// Both `cstr1` and `cstr2` must point to character arrays that are valid for reads up until and
/// including their null-terminating bytes.
NSTDAPI NSTDBool nstd_core_cstr_raw_compare(const NSTDChar *cstr1, const NSTDChar *cstr2);

/// Copies the contents of one raw C string to another, excluding the source's null-terminator.
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
/// - `src` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `dest` must point to a character array that is valid for writes.
///
/// - `dest`'s buffer must be large enough to contain the contents of `src`.
NSTDAPI void nstd_core_cstr_raw_copy(NSTDChar *dest, const NSTDChar *src);

/// Copies the contents of one raw C string to another, including the source's null-terminator.
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
/// - `src` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `dest` must point to a character array that is valid for writes.
///
/// - `dest`'s buffer must be large enough to contain the contents of `src`, including it's
/// null-terminating byte.
NSTDAPI void nstd_core_cstr_raw_copy_with_null(NSTDChar *dest, const NSTDChar *src);

#endif
