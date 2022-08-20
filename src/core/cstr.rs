//! Unowned C string slices.
pub mod raw;
use self::raw::{nstd_core_cstr_raw_len, nstd_core_cstr_raw_len_with_null};
use crate::{
    core::{
        def::NSTDChar,
        slice::{nstd_core_slice_const_new, NSTDSliceConst},
    },
    NSTDBool, NSTDUInt, NSTD_FALSE, NSTD_NULL,
};

/// An immutable slice of a C string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDCStrConst {
    /// A pointer to the first character in the C string.
    ptr: *const NSTDChar,
    /// The length of the C string slice.
    len: NSTDUInt,
}
impl NSTDCStrConst {
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
/// `NSTDCStrConst cstr` - The new C string slice, referencing `raw`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_new(raw: *const NSTDChar, len: NSTDUInt) -> NSTDCStrConst {
    NSTDCStrConst { ptr: raw, len }
}

/// Creates a new instance of `NSTDCStrConst` from a raw C string, excluding the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A raw pointer to the first character in the C string.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice, referencing `raw`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nstd_core_cstr_const_from_raw(raw: *const NSTDChar) -> NSTDCStrConst {
    // SAFETY: `NSTDCStrConst` is already unsafe to access, so there's no need for any kind of
    // validation here.
    let len = unsafe { nstd_core_cstr_raw_len(raw) };
    nstd_core_cstr_const_new(raw, len)
}

/// Creates a new instance of `NSTDCStrConst` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A raw pointer to the first character in the C string.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice, referencing `raw`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nstd_core_cstr_const_from_raw_with_null(raw: *const NSTDChar) -> NSTDCStrConst {
    // SAFETY: `NSTDCStrConst` is already unsafe to access, so there's no need for any kind of
    // validation here.
    let len = unsafe { nstd_core_cstr_raw_len_with_null(raw) };
    nstd_core_cstr_const_new(raw, len)
}

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice of the C string slice's data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_as_bytes(cstr: &NSTDCStrConst) -> NSTDSliceConst {
    nstd_core_slice_const_new(cstr.ptr.cast(), 1, cstr.len)
}

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_as_ptr(cstr: &NSTDCStrConst) -> *const NSTDChar {
    cstr.ptr
}

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_len(cstr: &NSTDCStrConst) -> NSTDUInt {
    cstr.len
}

/// Determines whether or not a C string slice is null terminated. This will return false if the C
/// string slice contains any null bytes in the middle.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDBool is_null_terminated` - Returns true if the C string slice contains a single null byte
/// at the end.
///
/// # Safety
///
/// Undefined behavior may occur if `cstr`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_const_is_null_terminated(cstr: &NSTDCStrConst) -> NSTDBool {
    let mut i = 0;
    while i < cstr.len {
        if *cstr.ptr.add(i) == 0 {
            return (i == cstr.len - 1) as NSTDBool;
        }
        i += 1;
    }
    NSTD_FALSE
}

/// Returns a pointer to the first null byte in a C string slice if present.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *nul` - A pointer to the first null byte in `cstr`, or null ([NSTD_NULL]) if
/// the C string slice doesn't contain a null byte.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_get_null(cstr: &NSTDCStrConst) -> *const NSTDChar {
    let mut i = 0;
    while i < cstr.len {
        // SAFETY: The returned pointer is unsafe to access, no need for validation here.
        unsafe {
            if *cstr.ptr.add(i) == 0 {
                return cstr.ptr.add(i);
            }
        }
        i += 1;
    }
    NSTD_NULL as *const NSTDChar
}

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string.
///
/// - `NSTDUInt pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_get(cstr: &NSTDCStrConst, pos: NSTDUInt) -> *const NSTDChar {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nstd_core_cstr_mut_from_raw(raw: *mut NSTDChar) -> NSTDCStrMut {
    // SAFETY: `NSTDCStrMut` is already unsafe to access, so there's no need for any kind of
    // validation here.
    let len = unsafe { nstd_core_cstr_raw_len(raw) };
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn nstd_core_cstr_mut_from_raw_with_null(raw: *mut NSTDChar) -> NSTDCStrMut {
    // SAFETY: `NSTDCStrMut` is already unsafe to access, so there's no need for any kind of
    // validation here.
    let len = unsafe { nstd_core_cstr_raw_len_with_null(raw) };
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
/// `NSTDCStrConst cstr_const` - The immutable copy of `cstr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_as_const(cstr: &NSTDCStrMut) -> NSTDCStrConst {
    nstd_core_cstr_const_new(cstr.ptr, cstr.len)
}

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice of the C string slice's data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_as_bytes(cstr: &NSTDCStrMut) -> NSTDSliceConst {
    nstd_core_slice_const_new(cstr.ptr.cast(), 1, cstr.len)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_is_null_terminated(cstr: &NSTDCStrMut) -> NSTDBool {
    let cstr_const = nstd_core_cstr_mut_as_const(cstr);
    nstd_core_cstr_const_is_null_terminated(&cstr_const)
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_get_null(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
    nstd_core_cstr_mut_get_null_const(cstr) as *mut NSTDChar
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_get_null_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
    let cstr_const = nstd_core_cstr_mut_as_const(cstr);
    nstd_core_cstr_const_get_null(&cstr_const)
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
