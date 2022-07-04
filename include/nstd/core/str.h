#ifndef NSTD_CORE_STR_H_INCLUDED
#define NSTD_CORE_STR_H_INCLUDED
#include "../nstd.h"
#include "cstr.h"
#include "range.h"
#include "slice.h"
NSTDCPPSTART

/// An unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A view into the UTF-8 encoded buffer.
    NSTDSliceMut bytes;
} NSTDStrMut;

/// Creates a new instance of `NSTDStrMut` from a C string.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string to wrap.
///
/// # Returns
///
/// `NSTDStrMut str` - The new `NSTDStrMut` instance, excluding the C string's null terminator.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
NSTDAPI NSTDStrMut nstd_core_str_mut_from_cstr_unchecked(NSTDCStrMut *cstr);

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `NSTDSliceMut *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1, or `bytes` is not valid UTF-8.
///
/// # Safety
///
/// `bytes` must remain valid while the returned string slice is in use.
NSTDAPI NSTDStrMut nstd_core_str_mut_from_bytes(NSTDSliceMut *bytes);

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `NSTDSliceMut *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
NSTDAPI NSTDStrMut nstd_core_str_mut_from_bytes_unchecked(NSTDSliceMut *bytes);

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice over `str`'s data.
///
/// # Safety
///
/// `str`'s data must remain valid while the returned slice is in use.
NSTDAPI NSTDSliceConst nstd_core_str_mut_as_bytes(const NSTDStrMut *str);

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice to index.
///
/// - `NSTDUSize pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
NSTDAPI NSTDUnichar nstd_core_str_mut_get_char(const NSTDStrMut *str, NSTDUSize pos);

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `NSTDStrMut *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStrMut substr` - The new substring.
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
NSTDAPI NSTDStrMut nstd_core_str_mut_substr(NSTDStrMut *str, NSTDURange range);

/// An immutable unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A view into the UTF-8 encoded buffer.
    NSTDSliceConst bytes;
} NSTDStrConst;

/// Creates a new instance of `NSTDStrConst` from a C string.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string to wrap.
///
/// # Returns
///
/// `NSTDStrConst str` - The new `NSTDStrConst` instance, excluding the C string's null terminator.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
NSTDAPI NSTDStrConst nstd_core_str_const_from_cstr_unchecked(const NSTDCStrConst *cstr);

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1, or `bytes` is not valid UTF-8.
///
/// # Safety
///
/// `bytes` must remain valid while the returned string slice is in use.
NSTDAPI NSTDStrConst nstd_core_str_const_from_bytes(const NSTDSliceConst *bytes);

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
NSTDAPI NSTDStrConst nstd_core_str_const_from_bytes_unchecked(const NSTDSliceConst *bytes);

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice over `str`'s data.
///
/// # Safety
///
/// `str`'s data must remain valid while the returned slice is in use.
NSTDAPI NSTDSliceConst nstd_core_str_const_as_bytes(const NSTDStrConst *str);

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice to index.
///
/// - `NSTDUSize pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
NSTDAPI NSTDUnichar nstd_core_str_const_get(const NSTDStrConst *str, NSTDUSize pos);

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStrConst substr` - The new substring.
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
NSTDAPI NSTDStrConst nstd_core_str_const_substr(const NSTDStrConst *str, NSTDURange range);

NSTDCPPEND
#endif
