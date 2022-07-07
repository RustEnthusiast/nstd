//! C string processing.
pub mod raw;
use crate::{
    core::{
        def::NSTDChar,
        slice::{nstd_core_slice_const_new, NSTDSliceConst},
    },
    NSTDUSize,
};

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

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_const_as_ptr(cstr: &NSTDCStrConst) -> *const NSTDChar {
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
/// `NSTDUSize len` - The length of the C string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_const_len(cstr: &NSTDCStrConst) -> NSTDUSize {
    cstr.len
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

/// A mutable slice of a C string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCStrMut {
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
/// `NSTDCStrMut cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_new(raw: *mut NSTDChar, len: NSTDUSize) -> NSTDCStrMut {
    NSTDCStrMut { ptr: raw, len }
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
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned byte slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_as_bytes(cstr: &NSTDCStrMut) -> NSTDSliceConst {
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
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_as_ptr(cstr: &mut NSTDCStrMut) -> *mut NSTDChar {
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
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_mut_as_ptr_const(cstr: &NSTDCStrMut) -> *const NSTDChar {
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
/// `NSTDUSize len` - The length of the C string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_mut_len(cstr: &NSTDCStrMut) -> NSTDUSize {
    cstr.len
}

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string.
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
pub unsafe extern "C" fn nstd_core_cstr_mut_get(
    cstr: &mut NSTDCStrMut,
    pos: NSTDUSize,
) -> *mut NSTDChar {
    nstd_core_cstr_mut_get_const(cstr, pos) as *mut NSTDChar
}

/// Return an immutable pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string.
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
pub unsafe extern "C" fn nstd_core_cstr_mut_get_const(
    cstr: &NSTDCStrMut,
    pos: NSTDUSize,
) -> *const NSTDChar {
    match pos < cstr.len {
        true => cstr.ptr.add(pos),
        false => core::ptr::null(),
    }
}
