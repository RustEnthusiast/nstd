#ifndef NSTD_STRING_H_INCLUDED
#define NSTD_STRING_H_INCLUDED
#include "core/def.h"
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

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString *string` - A pointer to the string to free.
NSTDAPI void nstd_string_free(NSTDString *string);

NSTDCPPEND
#endif
