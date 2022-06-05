//! An unowned view into a UTF-8 encoded byte string.
use crate::{
    core::{
        cstr::nstd_core_cstr_len,
        def::NSTDChar,
        range::NSTDURange,
        slice::{nstd_core_slice_new, NSTDSlice},
    },
    NSTDUSize, NSTDUnichar,
};

/// An unowned view into a UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDStr {
    /// A view into the UTF-8 encoded buffer.
    pub bytes: NSTDSlice,
}

/// Creates a new instance of `NSTDStr` from a C string.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The C string to wrap.
///
/// # Returns
///
/// `NSTDStr str` - The new `NSTDStr` instance, excluding the C string's null terminator.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_cstr_unchecked(cstr: *mut NSTDChar) -> NSTDStr {
    let len = nstd_core_cstr_len(cstr);
    NSTDStr {
        bytes: nstd_core_slice_new(cstr.cast(), 1, len),
    }
}

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1, or `bytes` is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_from_bytes(bytes: &NSTDSlice) -> NSTDStr {
    assert!(bytes.ptr.size == 1);
    core::str::from_utf8(bytes.as_slice()).expect("Invalid UTF-8 bytes");
    NSTDStr { bytes: *bytes }
}

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes.ptr.size` is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` is valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_bytes_unchecked(bytes: &NSTDSlice) -> NSTDStr {
    assert!(bytes.ptr.size == 1);
    NSTDStr { bytes: *bytes }
}

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice to index.
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
/// This function is unsafe because the string slice's data may be invalid at the time of access.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_get_char(str: &NSTDStr, pos: NSTDUSize) -> NSTDUnichar {
    // SAFETY: String slices are always valid UTF-8.
    let str = core::str::from_utf8_unchecked(str.bytes.as_slice());
    match str.chars().nth(pos) {
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
/// - `NSTDStr *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStr substr` - The new substring.
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
pub extern "C" fn nstd_core_str_substr(str: &mut NSTDStr, range: NSTDURange) -> NSTDStr {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.end <= str.bytes.len);
    assert!(range.start <= range.end);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = unsafe { str.bytes.ptr.raw.add(range.start) };
    let bytes = nstd_core_slice_new(start, 1, range.end - range.start);
    nstd_core_str_from_bytes(&bytes)
}
