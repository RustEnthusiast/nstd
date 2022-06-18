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

/// Creates a new `NSTDCString` initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUSize cap` - The number of bytes to preallocate.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if `cap` is zero.
NSTDAPI NSTDCString nstd_cstring_new_with_cap(NSTDUSize cap);

/// Creates a deep copy of an `NSTDCString`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string to create a deep copy of.
///
/// # Returns
///
/// `NSTDCString cloned` - A new deep copy of `cstring`.
///
/// # Panics
///
/// This function will panic if allocating for the new C string fails.
NSTDAPI NSTDCString nstd_cstring_clone(const NSTDCString *cstring);

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string to free.
NSTDAPI void nstd_cstring_free(NSTDCString *cstring);

NSTDCPPEND
#endif
