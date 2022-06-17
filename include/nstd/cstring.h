#ifndef NSTD_CSTRING_H_INCLUDED
#define NSTD_CSTRING_H_INCLUDED
#include "nstd.h"
#include "vec.h"
NSTDCPPSTART

/// A dynamically sized, null terminated, C string.
typedef struct {
    /// The underlying vector of `NSTDChar`s.
    NSTDVec bytes;
} NSTDCString;

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
NSTDAPI NSTDCString nstd_cstring_new();

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string to free.
NSTDAPI void nstd_cstring_free(NSTDCString *cstring);

NSTDCPPEND
#endif
