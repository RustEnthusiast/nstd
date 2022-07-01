//! C string processing.
pub mod raw;
use crate::{
    core::{
        def::NSTDChar,
        slice::{nstd_core_slice_const_new, NSTDSliceConst},
    },
    NSTDUSize,
};

/// A mutable slice of a C string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCStr {
    /// A pointer to the first character in the C string.
    pub ptr: *mut NSTDChar,
    /// The length of the C string, excluding the null byte.
    pub len: NSTDUSize,
}

/// Creates a new C string slice from a raw pointer and a size.
///
/// # Parameters:
///
/// - `NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUSize len` - The length of the C string, excluding the null byte.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_new(raw: *mut NSTDChar, len: NSTDUSize) -> NSTDCStr {
    NSTDCStr { ptr: raw, len }
}

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice of the C string slice's data.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned byte slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_as_bytes(cstr: &NSTDCStr) -> NSTDSliceConst {
    nstd_core_slice_const_new(cstr.ptr.cast(), 1, cstr.len)
}

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `NSTDCStr *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_get(cstr: &mut NSTDCStr, pos: NSTDUSize) -> *mut NSTDChar {
    nstd_core_cstr_get_const(cstr, pos) as *mut NSTDChar
}

/// Return an immutable pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_get_const(
    cstr: &NSTDCStr,
    pos: NSTDUSize,
) -> *const NSTDChar {
    match pos < cstr.len {
        true => cstr.ptr.add(pos),
        false => core::ptr::null(),
    }
}

/// An immutable slice of a C string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCStrConst {
    /// A pointer to the first character in the C string.
    pub ptr: *const NSTDChar,
    /// The length of the C string, excluding the null byte.
    pub len: NSTDUSize,
}

/// Creates a new C string slice from a raw pointer and a size.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUSize len` - The length of the C string, excluding the null byte.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_const_new(
    raw: *const NSTDChar,
    len: NSTDUSize,
) -> NSTDCStrConst {
    NSTDCStrConst { ptr: raw, len }
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
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned byte slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_const_as_bytes(cstr: &NSTDCStrConst) -> NSTDSliceConst {
    nstd_core_slice_const_new(cstr.ptr.cast(), 1, cstr.len)
}

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_const_get(
    cstr: &NSTDCStrConst,
    pos: NSTDUSize,
) -> *const NSTDChar {
    match pos < cstr.len {
        true => cstr.ptr.add(pos),
        false => core::ptr::null(),
    }
}
