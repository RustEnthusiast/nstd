#ifndef NSTD_STRING_H
#define NSTD_STRING_H
#include "core/alloc.h"
#include "core/def.h"
#include "core/optional.h"
#include "core/slice.h"
#include "core/str.h"
#include "core/unichar.h"
#include "nstd.h"
#include "vec.h"

/// Dynamically sized UTF-8 encoded byte string.
typedef struct {
    /// The underlying UTF-8 encoded byte buffer.
    NSTDVec bytes;
} NSTDString;

/// Represents an optional value of type `NSTDString`.
NSTDOptional(NSTDString) NSTDOptionalString;

/// Creates a new instance of `NSTDString`.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// # Returns
///
/// `NSTDString string` - The new string.
NSTDAPI NSTDString nstd_string_new(const NSTDAllocator *allocator);

/// Creates a new string initialized with the given capacity.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDOptionalString string` - The new string on success, or an uninitialized "none" variant if
/// allocating fails.
NSTDAPI NSTDOptionalString nstd_string_new_with_cap(const NSTDAllocator *allocator, NSTDUInt cap);

/// Creates an owned version of an unowned string slice.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `const NSTDStr *str` - The unowned string slice.
///
/// # Returns
///
/// `NSTDOptionalString string` - The new owned version of `str` on success, or an uninitialized
/// "none" variant if allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure that `str`'s data is valid for reads.
NSTDAPI NSTDOptionalString nstd_string_from_str(const NSTDAllocator *allocator, const NSTDStr *str);

/// Creates a new string from owned UTF-8 data.
///
/// # Parameters:
///
/// - `NSTDVec bytes` - The owned UTF-8 encoded buffer to take ownership of.
///
/// # Returns
///
/// `NSTDOptionalString string` - The new UTF-8 encoded string with ownership of `bytes` on success
/// or an uninitialized "none" variant if `bytes` contains invalid UTF-8.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
NSTDAPI NSTDOptionalString nstd_string_from_bytes(NSTDVec bytes);

/// Creates a deep copy of a string.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string to create a deep copy of.
///
/// # Returns
///
/// `NSTDOptionalString cloned` - A new deep copy of `string` on success, or an uninitialized
/// "none" variant if allocating fails.
NSTDAPI NSTDOptionalString nstd_string_clone(const NSTDString *string);

/// Returns an immutable reference to a string's allocator.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `const NSTDAllocator *allocator` - The string's allocator.
NSTDAPI const NSTDAllocator *nstd_string_allocator(const NSTDString *allocator);

/// Creates a string slice containing the contents of `string`.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
NSTDAPI NSTDStr nstd_string_as_str(const NSTDString *string);

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
/// `NSTDSlice bytes` - The string's active data.
NSTDAPI NSTDSlice nstd_string_as_bytes(const NSTDString *string);

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
NSTDAPI NSTDVec nstd_string_into_bytes(NSTDString string);

/// Returns the number of Unicode characters in a string.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the string.
NSTDAPI NSTDUInt nstd_string_len(const NSTDString *string);

/// Returns the number of bytes a string contains.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUInt byte_len` - The number of bytes in the string.
NSTDAPI NSTDUInt nstd_string_byte_len(const NSTDString *string);

/// Returns a string's capacity.
///
/// This is the max number of *bytes* the string can contain without reallocating.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUInt cap` - The string's capacity.
NSTDAPI NSTDUInt nstd_string_cap(const NSTDString *string);

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
/// `NSTDAllocError errc` - The allocation operation error code.
NSTDAPI NSTDAllocError nstd_string_push(NSTDString *string, NSTDUnichar chr);

/// Appends a string slice to the end of a string.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string.
///
/// - `const NSTDStr *str` - The string slice to append to the end of `string`.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// This function will cause undefined behavior in the case where `str`'s data is no longer valid.
NSTDAPI NSTDAllocError nstd_string_push_str(NSTDString *string, const NSTDStr *str);

/// Removes the last character from a string and returns it.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to pop.
///
/// # Returns
///
/// `NSTDOptionalUnichar chr` - The removed character on success.
NSTDAPI NSTDOptionalUnichar nstd_string_pop(NSTDString *string);

/// Sets a string's length to zero.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to clear.
NSTDAPI void nstd_string_clear(NSTDString *string);

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

/// Creates a new `NSTDString` from an `NSTDInt`.
///
/// # Parameters:
///
/// - `NSTDInt v` - The arch-bit signed integer value.
///
/// # Returns
///
/// `NSTDString string` - The arch-bit signed integer value as a string.
NSTDAPI NSTDString nstd_string_from_int(NSTDInt v);

/// Creates a new `NSTDString` from an `NSTDUInt`.
///
/// # Parameters:
///
/// - `NSTDUInt v` - The arch-bit unsigned integer value.
///
/// # Returns
///
/// `NSTDString string` - The arch-bit unsigned integer value as a string.
NSTDAPI NSTDString nstd_string_from_uint(NSTDUInt v);

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

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString string` - The string to free.
NSTDAPI void nstd_string_free(NSTDString string);

#endif
