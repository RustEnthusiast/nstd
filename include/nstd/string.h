#ifndef NSTD_STRING_H
#define NSTD_STRING_H
#include "core/def.h"
#include "core/slice.h"
#include "core/str.h"
#include "nstd.h"
#include "vec.h"
NSTDCPPSTART

/// Dynamically sized UTF-8 encoded byte string.
typedef struct {
    /// The underlying UTF-8 encoded byte buffer.
    NSTDVec bytes;
} NSTDString;

/// Creates a new instance of `NSTDString`.
///
/// # Returns
///
/// `NSTDString string` - The new string.
NSTDAPI NSTDString nstd_string_new();

/// Creates a new string initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUSize cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDString string` - The new string.
///
/// # Panics
///
/// This function will panic if `cap` is zero.
NSTDAPI NSTDString nstd_string_new_with_cap(NSTDUSize cap);

/// Creates a deep copy of a string.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string to create a deep copy of.
///
/// # Returns
///
/// `NSTDString cloned` - A new deep copy of `string`.
///
/// # Panics
///
/// This function will panic if allocating for the new string fails.
NSTDAPI NSTDString nstd_string_clone(const NSTDString *string);

/// Creates a string slice containing the contents of `string`.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
NSTDAPI NSTDStrConst nstd_string_as_str(const NSTDString *string);

/// Creates a string slice containing the contents of `string`.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
NSTDAPI NSTDStrMut nstd_string_as_str_mut(NSTDString *string);

/// Returns an immutable byte slice of the string's active data.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - The string's active data.
NSTDAPI NSTDSliceConst nstd_string_as_bytes(const NSTDString *string);

/// Returns a raw pointer to a string's memory.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string's memory.
NSTDAPI const NSTDByte *nstd_string_as_ptr(const NSTDString *string);

/// Returns ownership of an `NSTDString`'s raw data, taking ownership of said string.
///
/// # Parameters:
///
/// - `NSTDString string` - The string.
///
/// # Returns
///
/// `NSTDVec bytes` - The string's raw data.
NSTDAPI NSTDVec nstd_string_to_bytes(NSTDString string);

/// Returns the number of Unicode characters in a string.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the string.
NSTDAPI NSTDUSize nstd_string_len(const NSTDString *string);

/// Pushes an `NSTDUnichar` onto the end of a string.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to append the character to.
///
/// - `NSTDUnichar chr` - The Unicode character to append to the string.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_string_push(NSTDString *string, NSTDUnichar chr);

/// Appends a string slice to the end of a string.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string.
///
/// - `const NSTDStrConst *str` - The string slice to append to the end of `string`.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// This function will cause undefined behavior in the case where `str`'s data is no longer valid.
NSTDAPI NSTDErrorCode nstd_string_push_str(NSTDString *string, const NSTDStrConst *str);

/// Removes the last character from a string and returns it.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to pop.
///
/// # Returns
///
/// `NSTDUnichar chr` - The removed character, or the Unicode replacement character on error.
NSTDAPI NSTDUnichar nstd_string_pop(NSTDString *string);

/// Creates a new `NSTDString` from an `NSTDFloat32`.
///
/// # Parameters:
///
/// - `NSTDFloat32 v` - The 32-bit floating-point value.
///
/// # Returns
///
/// `NSTDString string` - The 32-bit floating-point value as a string.
NSTDAPI NSTDString nstd_string_from_f32(NSTDFloat32 v);

/// Creates a new `NSTDString` from an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `NSTDFloat64 v` - The 64-bit floating-point value.
///
/// # Returns
///
/// `NSTDString string` - The 64-bit floating-point value as a string.
NSTDAPI NSTDString nstd_string_from_f64(NSTDFloat64 v);

/// Creates a new `NSTDString` from an `NSTDUInt8`.
///
/// # Parameters:
///
/// - `NSTDUInt8 v` - The 8-bit unsigned integer value.
///
/// # Returns
///
/// `NSTDString string` - The 8-bit unsigned integer value as a string.
NSTDAPI NSTDString nstd_string_from_u8(NSTDUInt8 v);

/// Creates a new `NSTDString` from an `NSTDInt8`.
///
/// # Parameters:
///
/// - `NSTDInt8 v` - The 8-bit signed integer value.
///
/// # Returns
///
/// `NSTDString string` - The 8-bit signed integer value as a string.
NSTDAPI NSTDString nstd_string_from_i8(NSTDInt8 v);

/// Creates a new `NSTDString` from an `NSTDUInt16`.
///
/// # Parameters:
///
/// - `NSTDUInt16 v` - The 16-bit unsigned integer value.
///
/// # Returns
///
/// `NSTDString string` - The 16-bit unsigned integer value as a string.
NSTDAPI NSTDString nstd_string_from_u16(NSTDUInt16 v);

/// Creates a new `NSTDString` from an `NSTDInt16`.
///
/// # Parameters:
///
/// - `NSTDInt16 v` - The 16-bit signed integer value.
///
/// # Returns
///
/// `NSTDString string` - The 16-bit signed integer value as a string.
NSTDAPI NSTDString nstd_string_from_i16(NSTDInt16 v);

/// Creates a new `NSTDString` from an `NSTDUInt32`.
///
/// # Parameters:
///
/// - `NSTDUInt32 v` - The 32-bit unsigned integer value.
///
/// # Returns
///
/// `NSTDString string` - The 32-bit unsigned integer value as a string.
NSTDAPI NSTDString nstd_string_from_u32(NSTDUInt32 v);

/// Creates a new `NSTDString` from an `NSTDInt32`.
///
/// # Parameters:
///
/// - `NSTDInt32 v` - The 32-bit signed integer value.
///
/// # Returns
///
/// `NSTDString string` - The 32-bit signed integer value as a string.
NSTDAPI NSTDString nstd_string_from_i32(NSTDInt32 v);

/// Creates a new `NSTDString` from an `NSTDUInt64`.
///
/// # Parameters:
///
/// - `NSTDUInt64 v` - The 64-bit unsigned integer value.
///
/// # Returns
///
/// `NSTDString string` - The 64-bit unsigned integer value as a string.
NSTDAPI NSTDString nstd_string_from_u64(NSTDUInt64 v);

/// Creates a new `NSTDString` from an `NSTDInt64`.
///
/// # Parameters:
///
/// - `NSTDInt64 v` - The 64-bit signed integer value.
///
/// # Returns
///
/// `NSTDString string` - The 64-bit signed integer value as a string.
NSTDAPI NSTDString nstd_string_from_i64(NSTDInt64 v);

/// Creates a new `NSTDString` from an `NSTDUSize`.
///
/// # Parameters:
///
/// - `NSTDUSize v` - The arch-bit unsigned integer value.
///
/// # Returns
///
/// `NSTDString string` - The arch-bit unsigned integer value as a string.
NSTDAPI NSTDString nstd_string_from_usize(NSTDUSize v);

/// Creates a new `NSTDString` from an `NSTDISize`.
///
/// # Parameters:
///
/// - `NSTDISize v` - The arch-bit signed integer value.
///
/// # Returns
///
/// `NSTDString string` - The arch-bit signed integer value as a string.
NSTDAPI NSTDString nstd_string_from_isize(NSTDISize v);

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString string` - A pointer to the string to free.
NSTDAPI void nstd_string_free(NSTDString string);

NSTDCPPEND
#endif
