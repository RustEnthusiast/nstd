#ifndef NSTD_CORE_CSTR_CSTR_H
#define NSTD_CORE_CSTR_CSTR_H
#include "../../nstd.h"
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_from_raw, nstd_core_cstr_len};
///
/// let s_str = "Yo yo dog\0";
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw(s_str.as_ptr().cast());
///     assert!(nstd_core_cstr_len(&cstr) == s_str.len() - 1);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_from_raw_with_null, nstd_core_cstr_len};
///
/// let s_str = "Yo yo cat\0";
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw_with_null(s_str.as_ptr().cast());
///     assert!(nstd_core_cstr_len(&cstr) == s_str.len());
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     cstr::{nstd_core_cstr_as_bytes, nstd_core_cstr_from_raw, nstd_core_cstr_len},
///     slice::nstd_core_slice_len,
/// };
///
/// let s_str = "Rusty 🦀\0";
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw(s_str.as_ptr().cast());
///     let bytes = nstd_core_cstr_as_bytes(&cstr);
///     assert!(nstd_core_cstr_len(&cstr) == nstd_core_slice_len(&bytes));
/// }
/// ```
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
/// This function will panic if `cstr`'s length is greater than `NSTDInt`'s maximum value.
///
/// # Safety
///
/// - Undefined behavior may occur if `cstr`'s data is invalid.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
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
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{
///     nstd_core_cstr_from_raw, nstd_core_cstr_from_raw_with_null, nstd_core_cstr_get,
///     nstd_core_cstr_get_null, nstd_core_cstr_len,
/// };
///
/// let s_str = "Where is the null byte?\0";
/// let str_ptr = s_str.as_ptr().cast();
///
/// unsafe {
///     let mut cstr = nstd_core_cstr_from_raw_with_null(str_ptr);
///     let ptr = nstd_core_cstr_get_null(&cstr);
///     let last_pos = nstd_core_cstr_len(&cstr) - 1;
///     assert!(ptr == nstd_core_cstr_get(&cstr, last_pos));
///
///     cstr = nstd_core_cstr_from_raw(str_ptr);
///     assert!(nstd_core_cstr_get_null(&cstr).is_null());
/// }
/// ```
NSTDAPI const NSTDChar *nstd_core_cstr_get_null(const NSTDCStr *cstr);

/// Return a pointer the character at `pos` in `cstr`.
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_from_raw_with_null, nstd_core_cstr_get};
///
/// let s_str = "AMP\0";
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw_with_null(s_str.as_ptr().cast());
///     let nb = nstd_core_cstr_get(&cstr, s_str.len() - 1);
///     assert!(!nb.is_null());
///     assert!(*nb == 0);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_from_raw, nstd_core_cstr_mut_len};
///
/// let mut s_str = String::from("Yo yo dog\0");
///
/// unsafe {
///     let cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     assert!(nstd_core_cstr_mut_len(&cstr) == s_str.len() - 1);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_from_raw_with_null, nstd_core_cstr_mut_len};
///
/// let mut s_str = String::from("Yo yo cat\0");
///
/// unsafe {
///     let cstr = nstd_core_cstr_mut_from_raw_with_null(s_str.as_mut_ptr().cast());
///     assert!(nstd_core_cstr_mut_len(&cstr) == s_str.len());
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     cstr::{nstd_core_cstr_mut_as_bytes, nstd_core_cstr_mut_from_raw, nstd_core_cstr_mut_len},
///     slice::nstd_core_slice_len,
/// };
///
/// let mut s_str = String::from("Rusty 🦀\0");
///
/// unsafe {
///     let cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     let bytes = nstd_core_cstr_mut_as_bytes(&cstr);
///     assert!(nstd_core_cstr_mut_len(&cstr) == nstd_core_slice_len(&bytes));
/// }
/// ```
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
/// - Undefined behavior may occur if `cstr`'s data is invalid.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_mut_is_null_terminated, nstd_core_cstr_mut_new},
///     NSTD_FALSE, NSTD_TRUE,
/// };
///
/// let mut nn_bytes = String::from("Hello, world!");
/// let nn_cstr = nstd_core_cstr_mut_new(nn_bytes.as_mut_ptr().cast(), nn_bytes.len());
///
/// let mut nt_bytes = String::from("Hello, world!\0");
/// let nt_cstr = nstd_core_cstr_mut_new(nt_bytes.as_mut_ptr().cast(), nt_bytes.len());
///
/// let mut mn_bytes = String::from("Hello, \0world!");
/// let mn_cstr = nstd_core_cstr_mut_new(mn_bytes.as_mut_ptr().cast(), mn_bytes.len());
///
/// unsafe {
///     assert!(nstd_core_cstr_mut_is_null_terminated(&nn_cstr) == NSTD_FALSE);
///     assert!(nstd_core_cstr_mut_is_null_terminated(&nt_cstr) == NSTD_TRUE);
///     assert!(nstd_core_cstr_mut_is_null_terminated(&mn_cstr) == NSTD_FALSE);
/// }
/// ```
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
/// # Safety
///
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{
///         nstd_core_cstr_mut_from_raw_with_null, nstd_core_cstr_mut_get_null,
///         nstd_core_cstr_mut_is_null_terminated,
///     },
///     NSTDChar, NSTD_FALSE,
/// };
///
/// let mut s_str = String::from("BMP\0");
///
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw_with_null(s_str.as_mut_ptr().cast());
///     let n = nstd_core_cstr_mut_get_null(&mut cstr);
///     assert!(!n.is_null());
///     *n = b'!' as NSTDChar;
///     assert!(nstd_core_cstr_mut_is_null_terminated(&cstr) == NSTD_FALSE);
/// }
/// ```
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
/// # Safety
///
/// This operation may attempt to access data that is unowned by the raw C string, which can lead
/// to undefined behavior.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{
///     nstd_core_cstr_mut_from_raw, nstd_core_cstr_mut_from_raw_with_null,
///     nstd_core_cstr_mut_get_const, nstd_core_cstr_mut_get_null_const, nstd_core_cstr_mut_len,
/// };
///
/// let mut s_str = String::from("Where is the null byte?\0");
/// let str_ptr = s_str.as_mut_ptr().cast();
///
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw_with_null(str_ptr);
///     let ptr = nstd_core_cstr_mut_get_null_const(&cstr);
///     let last_pos = nstd_core_cstr_mut_len(&cstr) - 1;
///     assert!(ptr == nstd_core_cstr_mut_get_const(&cstr, last_pos));
///
///     cstr = nstd_core_cstr_mut_from_raw(str_ptr);
///     assert!(nstd_core_cstr_mut_get_null_const(&cstr).is_null());
/// }
/// ```
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_null_const(const NSTDCStr *cstr);

/// Return a pointer the character at `pos` in `cstr`.
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
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_mut_from_raw_with_null, nstd_core_cstr_mut_get},
///     NSTDChar,
/// };
///
/// let mut s_str = String::from("BMP\0");
///
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw_with_null(s_str.as_mut_ptr().cast());
///     let b = nstd_core_cstr_mut_get(&mut cstr, 0);
///     assert!(!b.is_null());
///     *b = b'A' as NSTDChar;
///     assert!(s_str == "AMP\0");
/// }
/// ```
NSTDAPI NSTDChar *nstd_core_cstr_mut_get(NSTDCStrMut *cstr, NSTDUInt pos);

/// Return an immutable pointer the character at `pos` in `cstr`.
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_from_raw_with_null, nstd_core_cstr_mut_get_const};
///
/// let mut s_str = String::from("AMP\0");
///
/// unsafe {
///     let cstr = nstd_core_cstr_mut_from_raw_with_null(s_str.as_mut_ptr().cast());
///     let nb = nstd_core_cstr_mut_get_const(&cstr, s_str.len() - 1);
///     assert!(!nb.is_null());
///     assert!(*nb == 0);
/// }
/// ```
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_const(const NSTDCStrMut *cstr, NSTDUInt pos);

#endif
