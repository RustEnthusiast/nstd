#ifndef NSTD_CSTRING_H
#define NSTD_CSTRING_H
#include "alloc.h"
#include "core/cstr/cstr.h"
#include "core/optional.h"
#include "core/slice.h"
#include "nstd.h"
#include "vec.h"

/// A dynamically sized, null terminated, C string.
///
/// Managed C strings (`NSTDCString`) will always contain a null byte until freed.
typedef struct {
    /// The underlying vector of `NSTDChar`s.
    NSTDVec bytes;
} NSTDCString;

/// Represents an optional value of type `NSTDCString`.
NSTDOptional(NSTDCString) NSTDOptionalCString;

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if allocating for the null byte fails.
NSTDAPI NSTDCString nstd_cstring_new(void);

/// Creates a new `NSTDCString` initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUInt cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if either `cap` is zero or allocating fails.
NSTDAPI NSTDCString nstd_cstring_new_with_cap(NSTDUInt cap);

/// Creates an owned version of an unowned C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The unowned C string slice.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new owned version of `cstr` on success, or an uninitialized
/// "none" variant if `cstr` contains a null byte or allocating fails.
///
/// # Panics
///
/// This operation will panic if `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller of this function must ensure that `cstr`'s data is valid for reads.
NSTDAPI NSTDOptionalCString nstd_cstring_from_cstr(const NSTDCStr *cstr);

/// Creates an owned version of an unowned C string slice without checking if the slice contains
/// any null bytes.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The unowned C string slice.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new owned version of `cstr` on success, or an uninitialized
/// "none" variant if allocating fails.
///
/// # Panics
///
/// This operation will panic if `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller of this function must ensure the following preconditions:
///
/// - `cstr`'s data is valid for reads.
///
/// - `cstr` does not contain any null (`'\0'`) bytes.
NSTDAPI NSTDOptionalCString nstd_cstring_from_cstr_unchecked(const NSTDCStr *cstr);

/// Creates a new C string from owned data.
///
/// # Parameters:
///
/// - `NSTDVec bytes` - The bytes to take ownership of.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new C string with ownership of `bytes` on success, or an
/// uninitialized "none" variant if `bytes` does not end with a null (`\0`) byte.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `bytes`'s stride is not 1.
///
/// - `bytes`'s length is greater than `NSTDInt`'s max value.
NSTDAPI NSTDOptionalCString nstd_cstring_from_bytes(NSTDVec bytes);

/// Creates a deep copy of an `NSTDCString`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string to create a deep copy of.
///
/// # Returns
///
/// `NSTDOptionalCString cloned` - A new deep copy of `cstring` on success, or an uninitialized
/// "none" variant if allocating fails.
NSTDAPI NSTDOptionalCString nstd_cstring_clone(const NSTDCString *cstring);

/// Creates a C string slice containing the contents of `cstring`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
NSTDAPI NSTDCStr nstd_cstring_as_cstr(const NSTDCString *cstring);

/// Returns an immutable byte slice of the C string's active data, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDSlice bytes` - The C string's active data.
NSTDAPI NSTDSlice nstd_cstring_as_bytes(const NSTDCString *cstring);

/// Returns a raw pointer to a C string's memory.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A raw pointer to a C string's memory.
NSTDAPI const NSTDChar *nstd_cstring_as_ptr(const NSTDCString *cstring);

/// Returns ownership of an `NSTDCString`'s raw data, taking ownership of said C string.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string.
///
/// # Returns
///
/// `NSTDVec bytes` - The C string's raw data.
NSTDAPI NSTDVec nstd_cstring_into_bytes(NSTDCString cstring);

/// Returns the number of `char`s in a C string, excluding the null terminator.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string without it's null byte.
NSTDAPI NSTDUInt nstd_cstring_len(const NSTDCString *cstring);

/// Returns the number of `char`s in a C string, including the null terminator.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string including it's null byte.
NSTDAPI NSTDUInt nstd_cstring_len_with_null(const NSTDCString *cstring);

/// Returns a C string's capacity.
///
/// This is the max number of *bytes* the C string can contain without reallocating.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDUInt cap` - The C string's capacity.
NSTDAPI NSTDUInt nstd_cstring_cap(const NSTDCString *cstring);

/// Appends an `NSTDChar` to the end of an `NSTDCString`.
///
/// This will have no effect if `chr` is a null byte (0).
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
/// - `const NSTDCStr *cstr` - The C string slice to append to the end of `cstring`.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `cstr` contains a null byte.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// - Appending the new null byte to the end of the C string fails.
///
/// - The new length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation can cause undefined behavior in the case that `cstr`'s data is invalid.
NSTDAPI NSTDAllocError nstd_cstring_push_cstr(NSTDCString *cstring, const NSTDCStr *cstr);

/// Removes the last character from a C string and returns it.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDChar chr` - The removed character, or null if the C string is empty.
///
/// # Panics
///
/// This function will panic if getting a pointer to the C string's last character fails.
NSTDAPI NSTDChar nstd_cstring_pop(NSTDCString *cstring);

/// Sets a C string's length to zero.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string to clear.
NSTDAPI void nstd_cstring_clear(NSTDCString *cstring);

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string to free.
NSTDAPI void nstd_cstring_free(NSTDCString cstring);

#endif
