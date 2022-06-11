#ifndef NSTD_CORE_STR_H_INCLUDED
#define NSTD_CORE_STR_H_INCLUDED
#include "../nstd.h"
#include "def.h"
#include "range.h"
#include "slice.h"
NSTDCPPSTART

/// An unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A view into the UTF-8 encoded buffer.
    NSTDSlice bytes;
} NSTDStr;

/// Creates a new instance of `NSTDStr` from a C string.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The C string to wrap.
///
/// # Returns
///
/// `NSTDStr str` - The new `NSTDStr` instance, excluding the C string's null terminator.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
NSTDAPI NSTDStr nstd_core_str_from_cstr_unchecked(NSTDChar *cstr);

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1, or `bytes` is not valid UTF-8.
///
/// # Safety
///
/// `bytes` must remain valid while the returned string slice is in use.
NSTDAPI NSTDStr nstd_core_str_from_bytes(NSTDSlice *bytes);

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
NSTDAPI NSTDStr nstd_core_str_from_bytes_unchecked(NSTDSlice *bytes);

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice to index.
///
/// - `NSTDUSize pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
NSTDAPI NSTDUnichar nstd_core_str_get_char(const NSTDStr *str, NSTDUSize pos);

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `NSTDStr *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStr substr` - The new substring.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.end` is greater than `str.bytes.len`.
///
/// - `range.start` is greater than `range.end`.
///
/// - The substring bytes are not valid UTF-8.
///
/// # Safety
///
/// `str`'s data must remain valid while the returned string slice is in use.
NSTDAPI NSTDStr nstd_core_str_substr(NSTDStr *str, NSTDURange range);

NSTDCPPEND
#endif
