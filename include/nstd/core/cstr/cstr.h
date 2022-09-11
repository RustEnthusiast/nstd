#ifndef NSTD_CORE_CSTR_CSTR_H
#define NSTD_CORE_CSTR_CSTR_H
#include "../../nstd.h"
#include "../def.h"
#include "../slice.h"

/// An immutable slice of a C string.
typedef struct {
    /// A pointer to the first character in the C string.
    const NSTDChar *ptr;
    /// The length of the C string slice.
    NSTDUInt len;
} NSTDCStr;

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
/// `NSTDCStr cstr` - The new C string slice, referencing `raw`'s data.
NSTDAPI NSTDCStr nstd_core_cstr_new(const NSTDChar *raw, NSTDUInt len);

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
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
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
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
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
/// string slice contains any null bytes in the middle.
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
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_is_null_terminated, nstd_core_cstr_new},
///     NSTD_FALSE, NSTD_TRUE,
/// };
///
/// let nn_bytes = "Hello, world!";
/// let nn_cstr = nstd_core_cstr_new(nn_bytes.as_ptr().cast(), nn_bytes.len());
///
/// let nt_bytes = "Hello, world!\0";
/// let nt_cstr = nstd_core_cstr_new(nt_bytes.as_ptr().cast(), nt_bytes.len());
///
/// let mn_bytes = "Hello, \0world!";
/// let mn_cstr = nstd_core_cstr_new(mn_bytes.as_ptr().cast(), mn_bytes.len());
///
/// unsafe {
///     assert!(nstd_core_cstr_is_null_terminated(&nn_cstr) == NSTD_FALSE);
///     assert!(nstd_core_cstr_is_null_terminated(&nt_cstr) == NSTD_TRUE);
///     assert!(nstd_core_cstr_is_null_terminated(&mn_cstr) == NSTD_FALSE);
/// }
/// ```
NSTDAPI NSTDBool nstd_core_cstr_is_null_terminated(const NSTDCStr *cstr);

/// Returns a pointer to the first null byte in a C string slice if present.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null ([NSTD_NULL]) if
/// the C string slice doesn't contain a null byte.
///
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
NSTDAPI const NSTDChar *nstd_core_cstr_get_null(const NSTDCStr *cstr);

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
NSTDAPI const NSTDChar *nstd_core_cstr_get(const NSTDCStr *cstr, NSTDUInt pos);

/// A mutable slice of a C string.
typedef struct {
    /// A pointer to the first character in the C string.
    NSTDChar *ptr;
    /// The length of the C string slice.
    NSTDUInt len;
} NSTDCStrMut;

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
/// `NSTDCStrMut cstr` - The new C string slice, referencing `raw`'s data.
NSTDAPI NSTDCStrMut nstd_core_cstr_mut_new(NSTDChar *raw, NSTDUInt len);

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
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
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
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
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
/// string slice contains any null bytes in the middle.
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
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
NSTDAPI NSTDBool nstd_core_cstr_mut_is_null_terminated(const NSTDCStrMut *cstr);

/// Returns a pointer to the first null byte in a C string slice if present.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null ([NSTD_NULL]) if the C
/// string slice doesn't contain a null byte.
///
/// # Safety
///
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
NSTDAPI NSTDChar *nstd_core_cstr_mut_get_null(NSTDCStrMut *cstr);

/// Returns a pointer to the first null byte in a C string slice if present.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null ([NSTD_NULL]) if
/// the C string slice doesn't contain a null byte.
///
/// # Safety
///
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_null_const(const NSTDCStr *cstr);

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
NSTDAPI NSTDChar *nstd_core_cstr_mut_get(NSTDCStrMut *cstr, NSTDUInt pos);

/// Return an immutable pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_const(const NSTDCStrMut *cstr, NSTDUInt pos);

#endif
