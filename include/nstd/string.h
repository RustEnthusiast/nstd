#ifndef NSTD_STRING_H_INCLUDED
#define NSTD_STRING_H_INCLUDED
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

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString *string` - A pointer to the string to free.
NSTDAPI void nstd_string_free(NSTDString *string);

NSTDCPPEND
#endif
