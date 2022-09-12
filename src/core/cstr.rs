//! Unowned C string slices.
pub mod raw;
use self::raw::{nstd_core_cstr_raw_len, nstd_core_cstr_raw_len_with_null};
use crate::{
    core::{
        def::NSTDChar,
        slice::{nstd_core_slice_new, NSTDSlice},
    },
    NSTDBool, NSTDUInt,
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
    /// # Safety
    ///
    /// This C string slice's data must remain valid while the returned byte slice is in use.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_bytes(&self) -> &[u8] {
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
    #[cfg(not(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        use crate::NSTD_FALSE;
        let mut i = 0;
        while i < cstr.len {
            if *cstr.ptr.add(i) == 0 {
                return (i == cstr.len - 1) as NSTDBool;
            }
            i += 1;
        }
        NSTD_FALSE
    }
    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        use core::arch::asm;
        let NSTDCStr { ptr, len } = *cstr;
        let is_nt;
        asm!(
            include_str!("cstr/is_null_terminated.asm"),
            ptr = in(reg) ptr,
            len = in(reg) len,
            is_nt = out(reg_byte) is_nt,
            i = out(reg) _
        );
        is_nt
    }
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_get_null(cstr: &NSTDCStr) -> *const NSTDChar {
    let mut i = 0;
    while i < cstr.len {
        if *cstr.ptr.add(i) == 0 {
            return cstr.ptr.add(i);
        }
        i += 1;
    }
    core::ptr::null()
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_get(cstr: &NSTDCStr, pos: NSTDUInt) -> *const NSTDChar {
    match pos < cstr.len {
        // SAFETY: We've checked `pos`, and the returned pointer is already unsafe to access.
        true => unsafe { cstr.ptr.add(pos) },
        false => core::ptr::null(),
    }
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
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_get_null_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    let cstr_const = nstd_core_cstr_mut_as_const(cstr);
    nstd_core_cstr_get_null(&cstr_const)
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_get(cstr: &mut NSTDCStrMut, pos: NSTDUInt) -> *mut NSTDChar {
    nstd_core_cstr_mut_get_const(cstr, pos) as *mut NSTDChar
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_get_const(
    cstr: &NSTDCStrMut,
    pos: NSTDUInt,
) -> *const NSTDChar {
    match pos < cstr.len {
        // SAFETY: We've checked `pos`, and the returned pointer is already unsafe to access.
        true => unsafe { cstr.ptr.add(pos) },
        false => core::ptr::null(),
    }
}
