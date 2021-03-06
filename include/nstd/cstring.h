#ifndef NSTD_CSTRING_H_INCLUDED
#define NSTD_CSTRING_H_INCLUDED
#include "core/cstr.h"
#include "core/def.h"
#include "core/slice.h"
#include "nstd.h"
#include "vec.h"
NSTDCPPSTART

/// A dynamically sized, null terminated, C string.
///
/// Managed C strings (`NSTDCString`) will always contain a null byte until freed.
typedef struct {
    /// The underlying vector of `NSTDChar`s.
    NSTDVec bytes;
} NSTDCString;

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if allocating for the null byte fails.
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
/// This function will panic if either `cap` is zero or allocating fails.
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

/// Creates a C string slice containing the contents of `cstring`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice.
NSTDAPI NSTDCStrConst nstd_cstring_as_cstr(const NSTDCString *cstring);

/// Creates a C string slice containing the contents of `cstring`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStrMut cstr` - The new C string slice.
NSTDAPI NSTDCStrMut nstd_cstring_as_cstr_mut(NSTDCString *cstring);

/// Returns an immutable byte slice of the C string's active data, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - The C string's active data.
NSTDAPI NSTDSliceConst nstd_cstring_as_bytes(const NSTDCString *cstring);

/// Returns ownership of an `NSTDCString`'s raw data, taking ownership of said C string.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string.
///
/// # Returns
///
/// `NSTDVec bytes` - The C string's raw data.
NSTDAPI NSTDVec nstd_cstring_to_bytes(NSTDCString cstring);

/// Appends an `NSTDChar` to the end of an `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// - `NSTDChar chr` - The C char to append to the C string.
///
/// # Panics
///
/// This operation panics if `chr` cannot be appended to the C string.
NSTDAPI void nstd_cstring_push(NSTDCString *cstring, NSTDChar chr);

/// Appends a C string slice to the end of a C string.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// - `const NSTDCStrConst *cstr` - The C string slice to append to the end of `cstring`.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if reserving memory for the push fails.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `cstr` contains a null byte.
///
/// - Appending the new null byte to the end of the C string fails.
///
/// # Safety
///
/// This operation can cause undefined behavior in the case that `cstr's data is invalid.
NSTDAPI NSTDErrorCode nstd_cstring_push_cstr(NSTDCString *cstring, const NSTDCStrConst *cstr);

/// Removes the last character from a C string and returns it.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDChar chr` - The removed character, or null if the C string is empty.
NSTDAPI NSTDChar nstd_cstring_pop(NSTDCString *cstring);

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string to free.
NSTDAPI void nstd_cstring_free(NSTDCString cstring);

NSTDCPPEND
#endif
