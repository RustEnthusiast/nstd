#ifndef NSTD_CORE_STR_H
#define NSTD_CORE_STR_H
#include "../nstd.h"
#include "cstr.h"
#include "def.h"
#include "range.h"
#include "slice.h"

/// An immutable unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A raw pointer to the string's data.
    const NSTDByte *ptr;
    /// The number of bytes in the string.
    NSTDUInt len;
} NSTDStr;

/// Creates a new instance of `NSTDStr` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStr str` - The new `NSTDStr` instance.
///
/// # Panics
///
/// This function will panic if `cstr`'s data is not valid UTF-8.
NSTDAPI NSTDStr nstd_core_str_from_cstr(const NSTDCStr *cstr);

/// Creates a new instance of `NSTDStr` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStr str` - The new `NSTDStr` instance.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
NSTDAPI NSTDStr nstd_core_str_from_cstr_unchecked(const NSTDCStr *cstr);

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `const NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1, or `bytes` is not valid UTF-8.
NSTDAPI NSTDStr nstd_core_str_from_bytes(const NSTDSlice *bytes);

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `const NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
NSTDAPI NSTDStr nstd_core_str_from_bytes_unchecked(const NSTDSlice *bytes);

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDSlice bytes` - An immutable byte slice over `str`'s data.
NSTDAPI NSTDSlice nstd_core_str_as_bytes(const NSTDStr *str);

/// Returns a raw pointer to a string slice's memory.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string slice's memory.
NSTDAPI const NSTDByte *nstd_core_str_as_ptr(const NSTDStr *str);

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the string slice.
///
/// # Panics
///
/// This operation may panic in the event that `str`'s calculated length is greater than the
/// highest number representable by `NSTDUInt`.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDUInt nstd_core_str_len(const NSTDStr *str);

/// Returns the number of bytes a string slice contains.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt byte_len` - The number of bytes in the string slice.
NSTDAPI NSTDUInt nstd_core_str_byte_len(const NSTDStr *str);

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
/// - `NSTDUInt pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
///
/// # Safety
///
/// This operation could cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUnichar nstd_core_str_get_char(const NSTDStr *str, NSTDUInt pos);

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice to create the new substring from.
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
NSTDAPI NSTDStr nstd_core_str_substr(const NSTDStr *str, NSTDURange range);

/// Attempts to parse a string slice as an `NSTDFloat32`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDFloat32 v` - The parsed 32-bit floating-point value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDFloat32 nstd_core_str_to_f32(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDFloat64 v` - The parsed 64-bit floating-point value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDFloat64 nstd_core_str_to_f64(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt v` - The parsed arch-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt nstd_core_str_to_int(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt v` - The parsed arch-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt nstd_core_str_to_uint(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt8`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt8 v` - The parsed 8-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt8 nstd_core_str_to_i8(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt8`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt8 v` - The parsed 8-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt8 nstd_core_str_to_u8(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt16`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt16 v` - The parsed 16-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt16 nstd_core_str_to_i16(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt16`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt16 v` - The parsed 16-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt16 nstd_core_str_to_u16(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt32`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt32 v` - The parsed 32-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt32 nstd_core_str_to_i32(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt32`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt32 v` - The parsed 32-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt32 nstd_core_str_to_u32(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt64`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt64 v` - The parsed 64-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt64 nstd_core_str_to_i64(const NSTDStr *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt64`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt64 v` - The parsed 64-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt64 nstd_core_str_to_u64(const NSTDStr *str, NSTDErrorCode *errc);

/// An unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A raw pointer to the string's data.
    NSTDByte *ptr;
    /// The number of bytes in the string.
    NSTDUInt len;
} NSTDStrMut;

/// Creates a new instance of `NSTDStrMut` from a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStrMut str` - The new `NSTDStrMut` instance.
///
/// # Panics
///
/// This function will panic if `cstr`'s data is not valid UTF-8.
NSTDAPI NSTDStrMut nstd_core_str_mut_from_cstr(NSTDCStrMut *cstr);

/// Creates a new instance of `NSTDStrMut` from a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStrMut str` - The new `NSTDStrMut` instance.
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
/// This operation will panic if `bytes`'s stride is not 1, or `bytes` is not valid UTF-8.
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
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
NSTDAPI NSTDStrMut nstd_core_str_mut_from_bytes_unchecked(NSTDSliceMut *bytes);

/// Creates an immutable version of a mutable string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The mutable string slice.
///
/// # Returns
///
/// `NSTDStr str_const` - The immutable copy of `str`.
NSTDAPI NSTDStr nstd_core_str_mut_as_const(const NSTDStrMut *str);

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDSlice bytes` - An immutable byte slice over `str`'s data.
NSTDAPI NSTDSlice nstd_core_str_mut_as_bytes(const NSTDStrMut *str);

/// Returns an immutable raw pointer to a string slice's memory.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string slice's memory.
NSTDAPI const NSTDByte *nstd_core_str_mut_as_ptr(const NSTDStrMut *str);

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the string slice.
///
/// # Panics
///
/// This operation may panic in the event that `str`'s calculated length is greater than the
/// highest number representable by `NSTDUInt`.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDUInt nstd_core_str_mut_len(const NSTDStrMut *str);

/// Returns the number of bytes a string slice contains.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt byte_len` - The number of bytes in the string slice.
NSTDAPI NSTDUInt nstd_core_str_mut_byte_len(const NSTDStrMut *str);

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
/// - `NSTDUInt pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
///
/// # Safety
///
/// This operation could cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUnichar nstd_core_str_mut_get_char(const NSTDStrMut *str, NSTDUInt pos);

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
NSTDAPI NSTDStrMut nstd_core_str_mut_substr(NSTDStrMut *str, NSTDURange range);

/// Attempts to parse a string slice as an `NSTDFloat32`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDFloat32 v` - The parsed 32-bit floating-point value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDFloat32 nstd_core_str_mut_to_f32(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDFloat64 v` - The parsed 64-bit floating-point value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDFloat64 nstd_core_str_mut_to_f64(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt v` - The parsed arch-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt nstd_core_str_mut_to_int(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt v` - The parsed arch-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt nstd_core_str_mut_to_uint(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt8`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt8 v` - The parsed 8-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt8 nstd_core_str_mut_to_i8(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt8`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt8 v` - The parsed 8-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt8 nstd_core_str_mut_to_u8(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt16`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt16 v` - The parsed 16-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt16 nstd_core_str_mut_to_i16(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt16`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt16 v` - The parsed 16-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt16 nstd_core_str_mut_to_u16(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt32`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt32 v` - The parsed 32-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt32 nstd_core_str_mut_to_i32(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt32`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt32 v` - The parsed 32-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt32 nstd_core_str_mut_to_u32(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDInt64`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDInt64 v` - The parsed 64-bit signed integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDInt64 nstd_core_str_mut_to_i64(const NSTDStrMut *str, NSTDErrorCode *errc);
/// Attempts to parse a string slice as an `NSTDUInt64`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// - `NSTDErrorCode *errc` - Set to nonzero on error.
///
/// # Returns
///
/// `NSTDUInt64 v` - The parsed 64-bit unsigned integral value.
///
/// # Safety:
///
/// This operation can cause undefined behavior if `str`'s data is invalid.
NSTDAPI NSTDUInt64 nstd_core_str_mut_to_u64(const NSTDStrMut *str, NSTDErrorCode *errc);

#endif
