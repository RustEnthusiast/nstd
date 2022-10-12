//! Unowned C string slices.
pub mod raw;
use self::raw::{nstd_core_cstr_raw_len, nstd_core_cstr_raw_len_with_null};
use crate::{
    core::{
        mem::nstd_core_mem_search,
        slice::{nstd_core_slice_new, NSTDSlice},
    },
    NSTDBool, NSTDChar, NSTDUInt, NSTD_FALSE,
};

/// An immutable slice of a C string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDCStr {
    /// A pointer to the first character in the C string.
    ptr: *const NSTDChar,
    /// The length of the C string slice.
    len: NSTDUInt,
}
impl NSTDCStr {
    /// Interprets a C string slice as a byte slice.
    ///
    /// # Panics
    ///
    /// This operation will panic if the C string slice's length is greater than `isize::MAX`.
    ///
    /// # Safety
    ///
    /// This C string slice's data must remain valid and unmodified while the returned byte slice
    /// is in use.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_bytes(&self) -> &[u8] {
        assert!(self.len <= isize::MAX as usize);
        core::slice::from_raw_parts(self.ptr.cast(), self.len)
    }
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_new(raw: *const NSTDChar, len: NSTDUInt) -> NSTDCStr {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_from_raw(raw: *const NSTDChar) -> NSTDCStr {
    let len = nstd_core_cstr_raw_len(raw);
    nstd_core_cstr_new(raw, len)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_from_raw_with_null(raw: *const NSTDChar) -> NSTDCStr {
    let len = nstd_core_cstr_raw_len_with_null(raw);
    nstd_core_cstr_new(raw, len)
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_as_bytes(cstr: &NSTDCStr) -> NSTDSlice {
    nstd_core_slice_new(cstr.ptr.cast(), 1, cstr.len)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_as_ptr(cstr: &NSTDCStr) -> *const NSTDChar {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_len(cstr: &NSTDCStr) -> NSTDUInt {
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
/// # Panics
///
/// This function will panic if `cstr`'s length is greater than `NSTDInt`'s maximum value.
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_is_null_terminated(cstr: &NSTDCStr) -> NSTDBool {
    assert!(cstr.len <= isize::MAX as usize);
    let mut i = 0;
    while i < cstr.len {
        if *cstr.ptr.add(i) == 0 {
            return i == cstr.len - 1;
        }
        i += 1;
    }
    NSTD_FALSE
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_get_null(cstr: &NSTDCStr) -> *const NSTDChar {
    nstd_core_mem_search(cstr.ptr.cast(), cstr.len, 0).cast()
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_get(cstr: &NSTDCStr, pos: NSTDUInt) -> *const NSTDChar {
    if pos < cstr.len && pos <= isize::MAX as usize {
        // SAFETY: We've checked `pos`.
        return unsafe { cstr.ptr.add(pos) };
    }
    core::ptr::null()
}

/// A mutable slice of a C string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDCStrMut {
    /// A pointer to the first character in the C string.
    ptr: *mut NSTDChar,
    /// The length of the C string slice.
    len: NSTDUInt,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_new(raw: *mut NSTDChar, len: NSTDUInt) -> NSTDCStrMut {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_from_raw(raw: *mut NSTDChar) -> NSTDCStrMut {
    let len = nstd_core_cstr_raw_len(raw);
    nstd_core_cstr_mut_new(raw, len)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_from_raw_with_null(raw: *mut NSTDChar) -> NSTDCStrMut {
    let len = nstd_core_cstr_raw_len_with_null(raw);
    nstd_core_cstr_mut_new(raw, len)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_as_const(cstr: &NSTDCStrMut) -> NSTDCStr {
    nstd_core_cstr_new(cstr.ptr, cstr.len)
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_as_bytes(cstr: &NSTDCStrMut) -> NSTDSlice {
    nstd_core_slice_new(cstr.ptr.cast(), 1, cstr.len)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_as_ptr(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_as_ptr_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_len(cstr: &NSTDCStrMut) -> NSTDUInt {
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
/// # Panics
///
/// This function will panic if `cstr`'s length is greater than `NSTDInt`'s maximum value.
///
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_is_null_terminated(cstr: &NSTDCStrMut) -> NSTDBool {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_get_null(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_get_null_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    let cstr_const = nstd_core_cstr_mut_as_const(cstr);
    nstd_core_cstr_get_null(&cstr_const)
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_get(cstr: &mut NSTDCStrMut, pos: NSTDUInt) -> *mut NSTDChar {
    nstd_core_cstr_mut_get_const(cstr, pos) as *mut NSTDChar
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_get_const(
    cstr: &NSTDCStrMut,
    pos: NSTDUInt,
) -> *const NSTDChar {
    if pos < cstr.len && pos <= isize::MAX as usize {
        // SAFETY: We've checked `pos`.
        return unsafe { cstr.ptr.add(pos) };
    }
    core::ptr::null_mut()
}
