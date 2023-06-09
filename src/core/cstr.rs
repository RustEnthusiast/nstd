//! Unowned C string slices.
pub mod raw;
use self::raw::{nstd_core_cstr_raw_len, nstd_core_cstr_raw_len_with_null};
use super::NSTD_INT_MAX;
use crate::{
    core::{
        mem::nstd_core_mem_search,
        optional::{gen_optional, NSTDOptional},
        slice::{nstd_core_slice_new_unchecked, NSTDSlice},
    },
    NSTDBool, NSTDChar, NSTDUInt,
};
use nstdapi::nstdapi;

/// An immutable slice of a C string.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDCStr {
    /// A pointer to the first character in the C string.
    ptr: *const NSTDChar,
    /// The length of the C string slice.
    len: NSTDUInt,
}
impl NSTDCStr {
    /// Interprets a C string slice as a byte slice.
    ///
    /// # Safety
    ///
    /// This C string slice's data must remain valid and unmodified while the returned byte slice
    /// is in use.
    #[inline]
    pub(crate) const unsafe fn as_bytes(&self) -> &[u8] {
        core::slice::from_raw_parts(self.ptr.cast(), self.len)
    }
}
gen_optional!(NSTDOptionalCStr, NSTDCStr);

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
/// `NSTDOptionalCStr cstr` - The new C string slice referencing `raw`'s data on success, or an
/// uninitialized "none" variant if `raw` is null or `len` is greater than `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_is_null_terminated, nstd_core_cstr_new};
///
/// let str = "This is a null-terminated C string slice.\0";
/// let cstr = nstd_core_cstr_new(str.as_ptr().cast(), str.len()).unwrap();
/// assert!(unsafe { nstd_core_cstr_is_null_terminated(&cstr) });
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cstr_new(raw: *const NSTDChar, len: NSTDUInt) -> NSTDOptionalCStr {
    match !raw.is_null() && len <= NSTD_INT_MAX as _ {
        true => NSTDOptional::Some(NSTDCStr { ptr: raw, len }),
        false => NSTDOptional::None,
    }
}

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
/// The user of this function must ensure that `raw` is non-null and `len` is not greater than
/// `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_is_null_terminated, nstd_core_cstr_new_unchecked};
///
/// let str = "This is a null-terminated C string slice.\0";
/// unsafe {
///     let cstr = nstd_core_cstr_new_unchecked(str.as_ptr().cast(), str.len());
///     assert!(nstd_core_cstr_is_null_terminated(&cstr));
/// }
/// ```
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_cstr_new_unchecked(raw: *const NSTDChar, len: NSTDUInt) -> NSTDCStr {
    NSTDCStr { ptr: raw, len }
}

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
/// - `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `raw`'s length must not be greater than `NSTDInt`'s max value.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_from_raw(raw: *const NSTDChar) -> NSTDCStr {
    let len = nstd_core_cstr_raw_len(raw);
    nstd_core_cstr_new_unchecked(raw, len)
}

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
/// - `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `raw`'s length must not be greater than `NSTDInt`'s max value.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_from_raw_with_null(raw: *const NSTDChar) -> NSTDCStr {
    let len = nstd_core_cstr_raw_len_with_null(raw);
    nstd_core_cstr_new_unchecked(raw, len)
}

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
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_as_bytes(cstr: &NSTDCStr) -> NSTDSlice {
    // SAFETY: `cstr.ptr` is never null.
    unsafe { nstd_core_slice_new_unchecked(cstr.ptr.cast(), 1, cstr.len) }
}

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_as_ptr, nstd_core_cstr_new};
///
/// let str = "assert!(Rust + C >= God)";
/// let str_ptr = str.as_ptr().cast();
/// let cstr = nstd_core_cstr_new(str_ptr, str.len()).unwrap();
/// assert!(str_ptr == nstd_core_cstr_as_ptr(&cstr));
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_as_ptr(cstr: &NSTDCStr) -> *const NSTDChar {
    cstr.ptr
}

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_from_raw, nstd_core_cstr_len};
///
/// let str = "Sunflower seeds yum\0";
/// let cstr = unsafe { nstd_core_cstr_from_raw(str.as_ptr().cast()) };
/// assert!(nstd_core_cstr_len(&cstr) == 19);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_len(cstr: &NSTDCStr) -> NSTDUInt {
    cstr.len
}

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
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
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
/// let nn_cstr = nstd_core_cstr_new(nn_bytes.as_ptr().cast(), nn_bytes.len()).unwrap();
///
/// let nt_bytes = "Hello, world!\0";
/// let nt_cstr = nstd_core_cstr_new(nt_bytes.as_ptr().cast(), nt_bytes.len()).unwrap();
///
/// let mn_bytes = "Hello, \0world!";
/// let mn_cstr = nstd_core_cstr_new(mn_bytes.as_ptr().cast(), mn_bytes.len()).unwrap();
///
/// unsafe {
///     assert!(nstd_core_cstr_is_null_terminated(&nn_cstr) == NSTD_FALSE);
///     assert!(nstd_core_cstr_is_null_terminated(&nt_cstr) == NSTD_TRUE);
///     assert!(nstd_core_cstr_is_null_terminated(&mn_cstr) == NSTD_FALSE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_is_null_terminated(cstr: &NSTDCStr) -> NSTDBool {
    nstd_core_mem_search(cstr.ptr.cast(), cstr.len, 0) == nstd_core_cstr_last(cstr).cast()
}

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
/// The caller must ensure that `cstr` is valid for reads.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_get_null(cstr: &NSTDCStr) -> *const NSTDChar {
    nstd_core_mem_search(cstr.ptr.cast(), cstr.len, 0).cast()
}

/// Return a pointer to the character at index `pos` in `cstr`.
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
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_get(cstr: &NSTDCStr, pos: NSTDUInt) -> *const NSTDChar {
    if pos < cstr.len {
        // SAFETY: We've checked `pos`.
        return unsafe { cstr.ptr.add(pos) };
    }
    core::ptr::null()
}

/// Returns a pointer to the first character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *first` - If present, a pointer to the first character in the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_first, nstd_core_cstr_from_raw},
///     NSTDChar,
/// };
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw("Tea\0".as_ptr().cast());
///     assert!(*nstd_core_cstr_first(&cstr) == b'T' as NSTDChar);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_first(cstr: &NSTDCStr) -> *const NSTDChar {
    match cstr.len > 0 {
        true => cstr.ptr,
        false => core::ptr::null(),
    }
}

/// Returns a pointer to the last character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *last` - If present, a pointer to the last character in the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_from_raw_with_null, nstd_core_cstr_last};
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw_with_null("Tea\0".as_ptr().cast());
///     assert!(*nstd_core_cstr_last(&cstr) == 0);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_last(cstr: &NSTDCStr) -> *const NSTDChar {
    match cstr.len > 0 {
        true => nstd_core_cstr_get(cstr, cstr.len - 1),
        false => core::ptr::null(),
    }
}

/// A mutable slice of a C string.
#[nstdapi]
pub struct NSTDCStrMut {
    /// A pointer to the first character in the C string.
    ptr: *mut NSTDChar,
    /// The length of the C string slice.
    len: NSTDUInt,
}
impl NSTDCStrMut {
    /// Interprets a C string slice as a byte slice.
    ///
    /// # Safety
    ///
    /// This C string slice's data must remain valid and unmodified while the returned byte slice
    /// is in use.
    #[inline]
    pub(crate) const unsafe fn as_bytes(&self) -> &[u8] {
        core::slice::from_raw_parts(self.ptr.cast(), self.len)
    }
}
gen_optional!(NSTDOptionalCStrMut, NSTDCStrMut);

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
/// `NSTDOptionalCStrMut cstr` - The new C string slice referencing `raw`'s data on success, or an
/// uninitialized "none" variant if `raw` is null or `len` is greater than `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_is_null_terminated, nstd_core_cstr_mut_new};
///
/// let mut str = String::from("This is a null-terminated C string slice.\0");
/// let cstr = nstd_core_cstr_mut_new(str.as_mut_ptr().cast(), str.len()).unwrap();
/// assert!(unsafe { nstd_core_cstr_mut_is_null_terminated(&cstr) });
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cstr_mut_new(raw: *mut NSTDChar, len: NSTDUInt) -> NSTDOptionalCStrMut {
    match !raw.is_null() && len <= NSTD_INT_MAX as _ {
        true => NSTDOptional::Some(NSTDCStrMut { ptr: raw, len }),
        false => NSTDOptional::None,
    }
}

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
/// The user of this function must ensure that `raw` is non-null and `len` is not greater than
/// `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{
///     nstd_core_cstr_mut_is_null_terminated, nstd_core_cstr_mut_new_unchecked,
/// };
///
/// let mut str = String::from("This is a null-terminated C string slice.\0");
/// unsafe {
///     let cstr = nstd_core_cstr_mut_new_unchecked(str.as_mut_ptr().cast(), str.len());
///     assert!(nstd_core_cstr_mut_is_null_terminated(&cstr));
/// }
/// ```
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_cstr_mut_new_unchecked(
    raw: *mut NSTDChar,
    len: NSTDUInt,
) -> NSTDCStrMut {
    NSTDCStrMut { ptr: raw, len }
}

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
/// - `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `raw`'s length must not be greater than `NSTDInt`'s max value.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_mut_from_raw(raw: *mut NSTDChar) -> NSTDCStrMut {
    let len = nstd_core_cstr_raw_len(raw);
    nstd_core_cstr_mut_new_unchecked(raw, len)
}

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
/// - `raw` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `raw`'s length must not be greater than `NSTDInt`'s max value.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_mut_from_raw_with_null(raw: *mut NSTDChar) -> NSTDCStrMut {
    let len = nstd_core_cstr_raw_len_with_null(raw);
    nstd_core_cstr_mut_new_unchecked(raw, len)
}

/// Creates an immutable version of a mutable C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The mutable C string slice.
///
/// # Returns
///
/// `NSTDCStr cstr_const` - The immutable copy of `cstr`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{
///     nstd_core_cstr_len, nstd_core_cstr_mut_as_const, nstd_core_cstr_mut_new,
/// };
///
/// let mut str = String::from("Faded than a ho");
/// let cstr_mut = nstd_core_cstr_mut_new(str.as_mut_ptr().cast(), str.len()).unwrap();
/// let cstr = nstd_core_cstr_mut_as_const(&cstr_mut);
/// assert!(nstd_core_cstr_len(&cstr) == str.len());
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_as_const(cstr: &NSTDCStrMut) -> NSTDCStr {
    // SAFETY: `cstr.ptr` is never null, C string slices are never longer than `NSTDInt`'s max
    // value.
    unsafe { nstd_core_cstr_new_unchecked(cstr.ptr, cstr.len) }
}

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
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_as_bytes(cstr: &NSTDCStrMut) -> NSTDSlice {
    // SAFETY: `cstr.ptr` is never null.
    unsafe { nstd_core_slice_new_unchecked(cstr.ptr.cast(), 1, cstr.len) }
}

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *ptr` - A pointer to the first character in the C string.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_as_ptr, nstd_core_cstr_mut_new};
///
/// let mut str = String::from("assert!(Rust + C >= God)");
/// let str_ptr = str.as_mut_ptr().cast();
/// let mut cstr = nstd_core_cstr_mut_new(str_ptr, str.len()).unwrap();
/// assert!(str_ptr == nstd_core_cstr_mut_as_ptr(&mut cstr));
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cstr_mut_as_ptr(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
    cstr.ptr
}

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_as_ptr_const, nstd_core_cstr_mut_new};
///
/// let mut str = String::from("assert!(Rust + C >= God)");
/// let cstr = nstd_core_cstr_mut_new(str.as_mut_ptr().cast(), str.len()).unwrap();
/// assert!(str.as_ptr().cast() == nstd_core_cstr_mut_as_ptr_const(&cstr));
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_as_ptr_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    cstr.ptr
}

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{nstd_core_cstr_mut_from_raw, nstd_core_cstr_mut_len};
///
/// let mut str = String::from("Sunflower seeds yum\0");
/// let cstr = unsafe { nstd_core_cstr_mut_from_raw(str.as_mut_ptr().cast()) };
/// assert!(nstd_core_cstr_mut_len(&cstr) == 19);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_len(cstr: &NSTDCStrMut) -> NSTDUInt {
    cstr.len
}

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
/// # Safety
///
/// The caller must ensure that `cstr` is valid for reads.
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
/// let nn_cstr = nstd_core_cstr_mut_new(nn_bytes.as_mut_ptr().cast(), nn_bytes.len()).unwrap();
///
/// let mut nt_bytes = String::from("Hello, world!\0");
/// let nt_cstr = nstd_core_cstr_mut_new(nt_bytes.as_mut_ptr().cast(), nt_bytes.len()).unwrap();
///
/// let mut mn_bytes = String::from("Hello, \0world!");
/// let mn_cstr = nstd_core_cstr_mut_new(mn_bytes.as_mut_ptr().cast(), mn_bytes.len()).unwrap();
///
/// unsafe {
///     assert!(nstd_core_cstr_mut_is_null_terminated(&nn_cstr) == NSTD_FALSE);
///     assert!(nstd_core_cstr_mut_is_null_terminated(&nt_cstr) == NSTD_TRUE);
///     assert!(nstd_core_cstr_mut_is_null_terminated(&mn_cstr) == NSTD_FALSE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_mut_is_null_terminated(cstr: &NSTDCStrMut) -> NSTDBool {
    let cstr_const = nstd_core_cstr_mut_as_const(cstr);
    nstd_core_cstr_is_null_terminated(&cstr_const)
}

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
/// The caller must ensure that `cstr` is valid for reads.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_mut_get_null(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
    nstd_core_cstr_mut_get_null_const(cstr) as *mut NSTDChar
}

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
/// The caller must ensure that `cstr` is valid for reads.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_mut_get_null_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    let cstr_const = nstd_core_cstr_mut_as_const(cstr);
    nstd_core_cstr_get_null(&cstr_const)
}

/// Return a pointer to the character at index `pos` in `cstr`.
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
#[inline]
#[nstdapi]
pub fn nstd_core_cstr_mut_get(cstr: &mut NSTDCStrMut, pos: NSTDUInt) -> *mut NSTDChar {
    nstd_core_cstr_mut_get_const(cstr, pos) as *mut NSTDChar
}

/// Return an immutable pointer to the character at index `pos` in `cstr`.
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
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_get_const(cstr: &NSTDCStrMut, pos: NSTDUInt) -> *const NSTDChar {
    if pos < cstr.len {
        // SAFETY: We've checked `pos`.
        return unsafe { cstr.ptr.add(pos) };
    }
    core::ptr::null_mut()
}

/// Returns a pointer to the first character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *first` - If present, a pointer to the first character in the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_mut_first, nstd_core_cstr_mut_from_raw},
///     NSTDChar,
/// };
///
/// let mut s_str = String::from("Bea\0");
///
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     *nstd_core_cstr_mut_first(&mut cstr) = b'T' as NSTDChar;
///     assert!(s_str == "Tea\0");
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cstr_mut_first(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
    match cstr.len > 0 {
        true => cstr.ptr,
        false => core::ptr::null_mut(),
    }
}

/// Returns an immutable pointer to the first character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *first` - If present, a pointer to the first character in the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_mut_first_const, nstd_core_cstr_mut_from_raw},
///     NSTDChar,
/// };
///
/// let mut s_str = String::from("Tea\0");
///
/// unsafe {
///     let cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     assert!(*nstd_core_cstr_mut_first_const(&cstr) == b'T' as NSTDChar);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_first_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    match cstr.len > 0 {
        true => cstr.ptr,
        false => core::ptr::null(),
    }
}

/// Returns a pointer to the last character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *last` - If present, a pointer to the last character in the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::{nstd_core_cstr_mut_from_raw, nstd_core_cstr_mut_last},
///     NSTDChar,
/// };
///
/// let mut s_str = String::from("Ted\0");
///
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     *nstd_core_cstr_mut_last(&mut cstr) = b'a' as NSTDChar;
///     assert!(s_str == "Tea\0");
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_cstr_mut_last(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
    match cstr.len > 0 {
        true => nstd_core_cstr_mut_get(cstr, cstr.len - 1),
        false => core::ptr::null_mut(),
    }
}

/// Returns an immutable pointer to the last character in a C string slice, or null if it is empty.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *last` - If present, a pointer to the last character in the C string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::{
///     nstd_core_cstr_mut_from_raw_with_null, nstd_core_cstr_mut_last_const,
/// };
///
/// let mut s_str = String::from("Tea\0");
///
/// unsafe {
///     let cstr = nstd_core_cstr_mut_from_raw_with_null(s_str.as_mut_ptr().cast());
///     assert!(*nstd_core_cstr_mut_last_const(&cstr) == 0);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_cstr_mut_last_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    match cstr.len > 0 {
        true => nstd_core_cstr_mut_get_const(cstr, cstr.len - 1),
        false => core::ptr::null(),
    }
}
