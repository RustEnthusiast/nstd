#ifndef NSTD_CORE_CSTR_CSTR_H
#define NSTD_CORE_CSTR_CSTR_H
#include "../../nstd.h"
#include "../optional.h"
#include "../slice.h"

/// An immutable slice of a C string.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid and unmodified
/// while an instance of this structure is in use.
typedef struct {
    /// A pointer to the first character in the C string.
    const NSTDChar *ptr;
    /// The length of the C string slice.
    NSTDUInt len;
} NSTDCStr;

/// Represents an optional value of type `NSTDCStr`.
NSTDOptional(NSTDCStr) NSTDOptionalCStr;

/// Creates a new C string slice from a raw pointer and a size.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUInt len` - The length of the C string slice.
///
/// # Returns
///
/// `NSTDOptionalCStr cstr` - The new C string slice referencing `raw`'s data on success, or a
/// "none" variant if `raw` is null.
NSTDAPI NSTDOptionalCStr nstd_core_cstr_new(const NSTDChar *raw, NSTDUInt len);

/// Creates a new C string slice from a raw pointer and a size without checking if `raw` is null.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUInt len` - The length of the C string slice.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// The user of this function must ensure that `raw` is not null.
NSTDAPI NSTDCStr nstd_core_cstr_new_unchecked(const NSTDChar *raw, NSTDUInt len);

/// Creates a new instance of `NSTDCStr` from a raw C string, excluding the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A raw pointer to the first character in the C string.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDCStr nstd_core_cstr_from_raw(const NSTDChar *raw);

/// Creates a new instance of `NSTDCStr` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A raw pointer to the first character in the C string.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDCStr nstd_core_cstr_from_raw_with_null(const NSTDChar *raw);

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSlice bytes` - An immutable byte slice of the C string slice's data.
NSTDAPI NSTDSlice nstd_core_cstr_as_bytes(const NSTDCStr *cstr);

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
NSTDAPI const NSTDChar *nstd_core_cstr_as_ptr(const NSTDCStr *cstr);

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string slice.
NSTDAPI NSTDUInt nstd_core_cstr_len(const NSTDCStr *cstr);

/// Determines whether or not a C string slice is null terminated. This will return false if the C
/// string slice contains any null bytes before the last byte.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDBool is_null_terminated` - Returns true if the C string slice contains a single null byte
/// at the end.
///
/// # Panics
///
/// This function may panic if `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
NSTDAPI NSTDBool nstd_core_cstr_is_null_terminated(const NSTDCStr *cstr);

/// Returns a pointer to the first null byte in a C string slice if one is present.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null if the C string
/// slice doesn't contain a null byte.
///
/// # Panics
///
/// This operation may panic if `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
NSTDAPI const NSTDChar *nstd_core_cstr_get_null(const NSTDCStr *cstr);

/// Return a pointer to the character at index `pos` in `cstr`.
///
/// # Note
///
/// This will return a null pointer if `pos` is greater than `NSTDInt`'s max value.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null if `pos` is out of the C
/// string slice's boundaries.
NSTDAPI const NSTDChar *nstd_core_cstr_get(const NSTDCStr *cstr, NSTDUInt pos);

/// Returns a pointer to the first character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *first` - If present, a pointer to the first character in the C string slice.
NSTDAPI const NSTDChar *nstd_core_cstr_first(const NSTDCStr *cstr);

/// Returns a pointer to the last character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *last` - If present, a pointer to the last character in the C string slice.
NSTDAPI const NSTDChar *nstd_core_cstr_last(const NSTDCStr *cstr);

/// A mutable slice of a C string.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid, unmodified, and
/// unreferenced in any other code while an instance of this structure is in use, else data races
/// may occur.
typedef struct {
    /// A pointer to the first character in the C string.
    NSTDChar *ptr;
    /// The length of the C string slice.
    NSTDUInt len;
} NSTDCStrMut;

/// Represents an optional value of type `NSTDCStrMut`.
NSTDOptional(NSTDCStrMut) NSTDOptionalCStrMut;

/// Creates a new C string slice from a raw pointer and a size.
///
/// # Parameters:
///
/// - `NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUInt len` - The length of the C string slice.
///
/// # Returns
///
/// `NSTDOptionalCStrMut cstr` - The new C string slice referencing `raw`'s data on success, or a
/// "none" variant if `raw` is null.
NSTDAPI NSTDOptionalCStrMut nstd_core_cstr_mut_new(NSTDChar *raw, NSTDUInt len);

/// Creates a new C string slice from a raw pointer and a size without checking if `raw` is null.
///
/// # Parameters:
///
/// - `NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUInt len` - The length of the C string slice.
///
/// # Returns
///
/// `NSTDCStrMut cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// The user of this function must ensure that `raw` is not null.
NSTDAPI NSTDCStrMut nstd_core_cstr_mut_new_unchecked(NSTDChar *raw, NSTDUInt len);

/// Creates a new instance of `NSTDCStrMut` from a raw C string, excluding the null byte.
///
/// # Parameters:
///
/// - `NSTDChar *raw` - A raw pointer to the first character in the C string.
///
/// # Returns
///
/// `NSTDCStrMut cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDCStrMut nstd_core_cstr_mut_from_raw(NSTDChar *raw);

/// Creates a new instance of `NSTDCStrMut` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `NSTDChar *raw` - A raw pointer to the first character in the C string.
///
/// # Returns
///
/// `NSTDCStrMut cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
NSTDAPI NSTDCStrMut nstd_core_cstr_mut_from_raw_with_null(NSTDChar *raw);

/// Creates an immutable version of a mutable C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The mutable C string slice.
///
/// # Returns
///
/// `NSTDCStr cstr_const` - The immutable copy of `cstr`.
NSTDAPI NSTDCStr nstd_core_cstr_mut_as_const(const NSTDCStrMut *cstr);

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSlice bytes` - An immutable byte slice of the C string slice's data.
NSTDAPI NSTDSlice nstd_core_cstr_mut_as_bytes(const NSTDCStrMut *cstr);

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *ptr` - A pointer to the first character in the C string.
NSTDAPI NSTDChar *nstd_core_cstr_mut_as_ptr(NSTDCStrMut *cstr);

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_as_ptr_const(const NSTDCStrMut *cstr);

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string slice.
NSTDAPI NSTDUInt nstd_core_cstr_mut_len(const NSTDCStrMut *cstr);

/// Determines whether or not a C string slice is null terminated. This will return false if the C
/// string slice contains any null bytes before the last byte.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDBool is_null_terminated` - Returns true if the C string slice contains a single null byte
/// at the end.
///
/// # Panics
///
/// This function will panic if `cstr`'s length is greater than `NSTDInt`'s maximum value.
///
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
NSTDAPI NSTDBool nstd_core_cstr_mut_is_null_terminated(const NSTDCStrMut *cstr);

/// Returns a pointer to the first null byte in a C string slice if one is present.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null if the C string
/// slice doesn't contain a null byte.
///
/// # Panics
///
/// This operation may panic if `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
NSTDAPI NSTDChar *nstd_core_cstr_mut_get_null(NSTDCStrMut *cstr);

/// Returns an immutable pointer to the first null byte in a C string slice if one is present.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null if the C string
/// slice doesn't contain a null byte.
///
/// # Panics
///
/// This operation may panic if `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_null_const(const NSTDCStrMut *cstr);

/// Return a pointer to the character at index `pos` in `cstr`.
///
/// # Note
///
/// This will return a null pointer if `pos` is greater than `NSTDInt`'s max value.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `NSTDChar *chr` - A pointer to the character at `pos`, or null if `pos` is out of the C
/// string slice's boundaries.
NSTDAPI NSTDChar *nstd_core_cstr_mut_get(NSTDCStrMut *cstr, NSTDUInt pos);

/// Return an immutable pointer to the character at index `pos` in `cstr`.
///
/// # Note
///
/// This will return a null pointer if `pos` is greater than `NSTDInt`'s max value.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null if `pos` is out of the C
/// string slice's boundaries.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_const(const NSTDCStrMut *cstr, NSTDUInt pos);

/// Returns a pointer to the first character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *first` - If present, a pointer to the first character in the C string slice.
NSTDAPI NSTDChar *nstd_core_cstr_mut_first(NSTDCStrMut *cstr);

/// Returns an immutable pointer to the first character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *first` - If present, a pointer to the first character in the C string slice.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_first_const(const NSTDCStrMut *cstr);

/// Returns a pointer to the last character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *last` - If present, a pointer to the last character in the C string slice.
NSTDAPI NSTDChar *nstd_core_cstr_mut_last(NSTDCStrMut *cstr);

/// Returns an immutable pointer to the last character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *last` - If present, a pointer to the last character in the C string slice.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_last_const(const NSTDCStrMut *cstr);

#endif
