#ifndef NSTD_STRING_H_INCLUDED
#define NSTD_STRING_H_INCLUDED
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
/// - `NSTDUSize cap` - The number of bytes to preallocate.
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
/// - `NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Safety
///
/// `string`'s data must remain valid while the returned string slice is in use.
NSTDAPI NSTDStr nstd_string_as_str(NSTDString *string);

/// Creates a string slice containing the contents of `string`.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Safety
///
/// `string`'s data must remain valid while the returned string slice is in use.
NSTDAPI NSTDStrConst nstd_string_as_str_const(const NSTDString *string);

/// Returns an immutable byte slice of the string's active data.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - The string's active data.
///
/// # Safety
///
/// `string`'s data must remain valid while the returned slice is in use.
NSTDAPI NSTDSliceConst nstd_string_as_bytes(const NSTDString *string);

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

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString *string` - A pointer to the string to free.
NSTDAPI void nstd_string_free(NSTDString *string);

NSTDCPPEND
#endif
