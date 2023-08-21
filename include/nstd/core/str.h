#ifndef NSTD_CORE_STR_H
#define NSTD_CORE_STR_H
#include "../nstd.h"
#include "cstr/cstr.h"
#include "def.h"
#include "optional.h"
#include "range.h"
#include "slice.h"
#include "unichar.h"

/// An immutable unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A raw pointer to the string's data.
    const NSTDByte *ptr;
    /// The number of bytes in the string.
    NSTDUInt len;
} NSTDStr;

/// Represents an optional value of type `NSTDStr`.
NSTDOptional(NSTDStr) NSTDOptionalStr;

/// Creates a new instance of an `NSTDStr` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new `NSTDStr` instance on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Safety
///
/// `cstr`'s data must be valid for reads of at least `cstr.len` consecutive bytes.
NSTDAPI NSTDOptionalStr nstd_core_str_from_cstr(const NSTDCStr *cstr);

/// Creates a new instance of an `NSTDStr` from a C string slice.
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

/// Creates a new `NSTDStr` from a raw C string.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new string slice on success or an uninitialized "none" variant if
/// `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDOptionalStr nstd_core_str_from_raw_cstr(const NSTDChar *cstr);

/// Creates a new `NSTDStr` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new string slice on success or an uninitialized "none" variant if
/// `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDOptionalStr nstd_core_str_from_raw_cstr_with_null(const NSTDChar *cstr);

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `const NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new string slice on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Safety
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
NSTDAPI NSTDOptionalStr nstd_core_str_from_bytes(const NSTDSlice *bytes);

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
/// - This function does not check to ensure that `bytes` are valid UTF-8.
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
NSTDAPI NSTDStr nstd_core_str_from_bytes_unchecked(const NSTDSlice *bytes);

/// Returns a C string slice variant of this UTF-8 encoded string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
NSTDAPI NSTDCStr nstd_core_str_as_cstr(const NSTDStr *str);

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
/// `NSTDOptionalUnichar chr` - The character at index `pos`, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUnichar nstd_core_str_get(const NSTDStr *str, NSTDUInt pos);

/// Creates a substring of an existing string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDOptionalStr substr` - The new substring on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.start` is greater than `range.end`.
///
/// - `range.end` is greater than `str.len`.
///
/// # Safety
///
/// `str`'s data must be valid for reads of at least `str.len` consecutive bytes.
NSTDAPI NSTDOptionalStr nstd_core_str_substr(const NSTDStr *str, NSTDURange range);

/// Attempts to parse a string slice as an `NSTDFloat32`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalFloat32 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalFloat32 nstd_core_str_to_f32(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalFloat64 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalFloat64 nstd_core_str_to_f64(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDInt`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt nstd_core_str_to_int(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDUInt`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt nstd_core_str_to_uint(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDInt8`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt8 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt8 nstd_core_str_to_i8(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDUInt8`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt8 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt8 nstd_core_str_to_u8(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDInt16`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt16 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt16 nstd_core_str_to_i16(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDUInt16`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt16 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt16 nstd_core_str_to_u16(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDInt32`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt32 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt32 nstd_core_str_to_i32(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDUInt32`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt32 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt32 nstd_core_str_to_u32(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDInt64`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt64 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt64 nstd_core_str_to_i64(const NSTDStr *str);
/// Attempts to parse a string slice as an `NSTDUInt64`.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt64 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt64 nstd_core_str_to_u64(const NSTDStr *str);

/// An unowned view into a UTF-8 encoded byte string.
typedef struct {
    /// A raw pointer to the string's data.
    NSTDByte *ptr;
    /// The number of bytes in the string.
    NSTDUInt len;
} NSTDStrMut;

/// Represents an optional value of type `NSTDStrMut`.
NSTDOptional(NSTDStrMut) NSTDOptionalStrMut;

/// Creates a new instance of an `NSTDStrMut` from a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new `NSTDStrMut` instance on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Safety
///
/// `cstr`'s data must be valid for reads of at least `cstr.len` consecutive bytes.
NSTDAPI NSTDOptionalStrMut nstd_core_str_mut_from_cstr(NSTDCStrMut *cstr);

/// Creates a new instance of an `NSTDStrMut` from a C string slice.
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

/// Creates a new `NSTDStrMut` from a raw C string.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new string slice on success or an uninitialized "none" variant
/// if `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDOptionalStrMut nstd_core_str_mut_from_raw_cstr(NSTDChar *cstr);

/// Creates a new `NSTDStrMut` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new string slice on success or an uninitialized "none" variant
/// if `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDOptionalStrMut nstd_core_str_mut_from_raw_cstr_with_null(NSTDChar *cstr);

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `NSTDSliceMut *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new string slice on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Safety
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
NSTDAPI NSTDOptionalStrMut nstd_core_str_mut_from_bytes(NSTDSliceMut *bytes);

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
/// - This function does not check to ensure that `bytes` are valid UTF-8.
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
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

/// Returns a C string slice variant of this UTF-8 encoded string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
NSTDAPI NSTDCStr nstd_core_str_mut_as_cstr(const NSTDStrMut *str);

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
/// `NSTDOptionalUnichar chr` - The character at index `pos`, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUnichar nstd_core_str_mut_get(const NSTDStrMut *str, NSTDUInt pos);

/// Creates a substring of an existing string slice.
///
/// # Parameters:
///
/// - `NSTDStrMut *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDOptionalStrMut substr` - The new substring on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.start` is greater than `range.end`.
///
/// - `range.end` is greater than `str.len`.
///
/// # Safety
///
/// `str`'s data must be valid for reads of at least `str.len` consecutive bytes.
NSTDAPI NSTDOptionalStrMut nstd_core_str_mut_substr(NSTDStrMut *str, NSTDURange range);

/// Attempts to parse a string slice as an `NSTDFloat32`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalFloat32 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalFloat32 nstd_core_str_mut_to_f32(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalFloat64 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalFloat64 nstd_core_str_mut_to_f64(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDInt`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt nstd_core_str_mut_to_int(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDUInt`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt nstd_core_str_mut_to_uint(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDInt8`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt8 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt8 nstd_core_str_mut_to_i8(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDUInt8`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt8 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt8 nstd_core_str_mut_to_u8(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDInt16`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt16 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt16 nstd_core_str_mut_to_i16(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDUInt16`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt16 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt16 nstd_core_str_mut_to_u16(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDInt32`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt32 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt32 nstd_core_str_mut_to_i32(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDUInt32`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt32 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt32 nstd_core_str_mut_to_u32(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDInt64`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalInt64 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalInt64 nstd_core_str_mut_to_i64(const NSTDStrMut *str);
/// Attempts to parse a string slice as an `NSTDUInt64`.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDOptionalUInt64 v` - The parsed value, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
NSTDAPI NSTDOptionalUInt64 nstd_core_str_mut_to_u64(const NSTDStrMut *str);

#endif
