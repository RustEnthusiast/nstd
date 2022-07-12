//! An unowned view into a UTF-8 encoded byte string.
use crate::{
    core::{
        cstr::{
            nstd_core_cstr_const_as_bytes, nstd_core_cstr_mut_as_ptr, nstd_core_cstr_mut_len,
            NSTDCStrConst, NSTDCStrMut,
        },
        range::NSTDURange,
        slice::{nstd_core_slice_const_new, nstd_core_slice_mut_new, NSTDSliceConst, NSTDSliceMut},
    },
    NSTDUSize, NSTDUnichar,
};

/// An immutable unowned view into a UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDStrConst {
    /// A view into the UTF-8 encoded buffer.
    bytes: NSTDSliceConst,
}
impl NSTDStrConst {
    /// Creates a Rust string slice from this [NSTDStrConst].
    #[inline]
    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.bytes.as_slice()) }
    }
}

/// Creates a new instance of `NSTDStrConst` from a C string.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string to wrap.
///
/// # Returns
///
/// `NSTDStrConst str` - The new `NSTDStrConst` instance, excluding the C string's null terminator.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_const_from_cstr_unchecked(
    cstr: &NSTDCStrConst,
) -> NSTDStrConst {
    NSTDStrConst {
        bytes: nstd_core_cstr_const_as_bytes(cstr),
    }
}

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1, or `bytes` is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_from_bytes(bytes: &NSTDSliceConst) -> NSTDStrConst {
    assert!(bytes.ptr.size == 1);
    core::str::from_utf8(bytes.as_slice()).expect("Invalid UTF-8 bytes");
    NSTDStrConst {
        bytes: nstd_core_slice_const_new(bytes.ptr.raw, bytes.ptr.size, bytes.len),
    }
}

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_const_from_bytes_unchecked(
    bytes: &NSTDSliceConst,
) -> NSTDStrConst {
    assert!(bytes.ptr.size == 1);
    NSTDStrConst {
        bytes: nstd_core_slice_const_new(bytes.ptr.raw, bytes.ptr.size, bytes.len),
    }
}

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice over `str`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_as_bytes(str: &NSTDStrConst) -> NSTDSliceConst {
    nstd_core_slice_const_new(str.bytes.ptr.raw.cast(), 1, str.bytes.len)
}

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_len(str: &NSTDStrConst) -> NSTDUSize {
    str.as_str().chars().count()
}

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice to index.
///
/// - `NSTDUSize pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
///
/// # Safety
///
/// This operation could cause undefined behavior if `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_const_get_char(
    str: &NSTDStrConst,
    pos: NSTDUSize,
) -> NSTDUnichar {
    match str.as_str().chars().nth(pos) {
        Some(chr) => chr as NSTDUnichar,
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
}

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStrConst substr` - The new substring.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.end` is greater than `str.bytes.len`.
///
/// - `range.start` is greater than `range.end`.
///
/// - The substring bytes are not valid UTF-8.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_substr(
    str: &NSTDStrConst,
    range: NSTDURange,
) -> NSTDStrConst {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.end <= str.bytes.len);
    assert!(range.start <= range.end);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = unsafe { str.bytes.ptr.raw.add(range.start) };
    let bytes = nstd_core_slice_const_new(start, 1, range.end - range.start);
    nstd_core_str_const_from_bytes(&bytes)
}

/// An unowned view into a UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDStrMut {
    /// A view into the UTF-8 encoded buffer.
    bytes: NSTDSliceMut,
}
impl NSTDStrMut {
    /// Creates a Rust string slice from this [NSTDStrMut].
    #[inline]
    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.bytes.as_slice()) }
    }
}

/// Creates a new instance of `NSTDStrMut` from a C string.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string to wrap.
///
/// # Returns
///
/// `NSTDStrMut str` - The new `NSTDStrMut` instance, excluding the C string's null terminator.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_cstr_unchecked(
    cstr: &mut NSTDCStrMut,
) -> NSTDStrMut {
    let ptr = nstd_core_cstr_mut_as_ptr(cstr).cast();
    let len = nstd_core_cstr_mut_len(cstr);
    NSTDStrMut {
        bytes: nstd_core_slice_mut_new(ptr, 1, len),
    }
}

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `NSTDSliceMut *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1, or `bytes` is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_from_bytes(bytes: &mut NSTDSliceMut) -> NSTDStrMut {
    assert!(bytes.ptr.size == 1);
    core::str::from_utf8(bytes.as_slice()).expect("Invalid UTF-8 bytes");
    NSTDStrMut {
        bytes: nstd_core_slice_mut_new(bytes.ptr.raw, bytes.ptr.size, bytes.len),
    }
}

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `NSTDSliceMut *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_bytes_unchecked(
    bytes: &mut NSTDSliceMut,
) -> NSTDStrMut {
    assert!(bytes.ptr.size == 1);
    NSTDStrMut {
        bytes: nstd_core_slice_mut_new(bytes.ptr.raw, bytes.ptr.size, bytes.len),
    }
}

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice over `str`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_as_bytes(str: &NSTDStrMut) -> NSTDSliceConst {
    nstd_core_slice_const_new(str.bytes.ptr.raw.cast(), 1, str.bytes.len)
}

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_len(str: &NSTDStrMut) -> NSTDUSize {
    str.as_str().chars().count()
}

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice to index.
///
/// - `NSTDUSize pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
///
/// # Safety
///
/// This operation could cause undefined behavior if `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_get_char(
    str: &NSTDStrMut,
    pos: NSTDUSize,
) -> NSTDUnichar {
    match str.as_str().chars().nth(pos) {
        Some(chr) => chr as NSTDUnichar,
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
}

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `NSTDStrMut *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStrMut substr` - The new substring.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.end` is greater than `str.bytes.len`.
///
/// - `range.start` is greater than `range.end`.
///
/// - The substring bytes are not valid UTF-8.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_substr(str: &mut NSTDStrMut, range: NSTDURange) -> NSTDStrMut {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.end <= str.bytes.len);
    assert!(range.start <= range.end);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = unsafe { str.bytes.ptr.raw.add(range.start) };
    let mut bytes = nstd_core_slice_mut_new(start, 1, range.end - range.start);
    nstd_core_str_mut_from_bytes(&mut bytes)
}
