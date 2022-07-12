//! Dynamically sized UTF-8 encoded byte string.
use crate::{
    core::{
        def::NSTDErrorCode,
        slice::{nstd_core_slice_const_new, NSTDSliceConst},
        str::{
            nstd_core_str_const_as_bytes, nstd_core_str_const_from_bytes_unchecked,
            nstd_core_str_mut_from_bytes_unchecked, NSTDStrConst, NSTDStrMut,
        },
    },
    vec::{
        nstd_vec_as_slice, nstd_vec_as_slice_mut, nstd_vec_clone, nstd_vec_extend, nstd_vec_len,
        nstd_vec_new, nstd_vec_new_with_cap, nstd_vec_truncate, NSTDVec,
    },
    NSTDUSize, NSTDUnichar,
};

/// Dynamically sized UTF-8 encoded byte string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDString {
    /// The underlying UTF-8 encoded byte buffer.
    bytes: NSTDVec,
}

/// Creates a new instance of `NSTDString`.
///
/// # Returns
///
/// `NSTDString string` - The new string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_new() -> NSTDString {
    NSTDString {
        bytes: nstd_vec_new(1),
    }
}

/// Creates a new string initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUSize cap` - The number of bytes to preallocate.
///
/// # Returns
///
/// `NSTDString string` - The new string.
///
/// # Panics
///
/// This function will panic if `cap` is zero.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_new_with_cap(cap: NSTDUSize) -> NSTDString {
    NSTDString {
        bytes: nstd_vec_new_with_cap(1, cap),
    }
}

/// Creates a deep copy of a string.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string to create a deep copy of.
///
/// # Returns
///
/// `NSTDString cloned` - A new deep copy of `string`.
///
/// # Panics
///
/// This function will panic if allocating for the new string fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_clone(string: &NSTDString) -> NSTDString {
    NSTDString {
        bytes: nstd_vec_clone(&string.bytes),
    }
}

/// Creates a string slice containing the contents of `string`.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_as_str(string: &NSTDString) -> NSTDStrConst {
    let bytes = nstd_vec_as_slice(&string.bytes);
    // SAFETY: The string's bytes are always be UTF-8 encoded.
    unsafe { nstd_core_str_const_from_bytes_unchecked(&bytes) }
}

/// Creates a string slice containing the contents of `string`.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_as_str_mut(string: &mut NSTDString) -> NSTDStrMut {
    let mut bytes = nstd_vec_as_slice_mut(&mut string.bytes);
    // SAFETY: The string's bytes are always be UTF-8 encoded.
    unsafe { nstd_core_str_mut_from_bytes_unchecked(&mut bytes) }
}

/// Returns an immutable byte slice of the string's active data.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - The string's active data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_as_bytes(string: &NSTDString) -> NSTDSliceConst {
    nstd_vec_as_slice(&string.bytes)
}

/// Pushes an `NSTDUnichar` onto the end of a string.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to append the character to.
///
/// - `NSTDUnichar chr` - The Unicode character to append to the string.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_push(string: &mut NSTDString, chr: NSTDUnichar) -> NSTDErrorCode {
    if let Some(chr) = char::from_u32(chr) {
        let mut buf = [0; 4];
        chr.encode_utf8(&mut buf);
        let buf = nstd_core_slice_const_new(buf.as_ptr().cast(), 1, chr.len_utf8());
        return unsafe { nstd_vec_extend(&mut string.bytes, &buf) };
    }
    1
}

/// Appends a string slice to the end of a string.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string.
///
/// - `const NSTDStrConst *str` - The string slice to append to the end of `string`.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// This function will cause undefined behavior in the case where `str`'s data is no longer valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_push_str(
    string: &mut NSTDString,
    str: &NSTDStrConst,
) -> NSTDErrorCode {
    let str_bytes = nstd_core_str_const_as_bytes(str);
    nstd_vec_extend(&mut string.bytes, &str_bytes)
}

/// Removes the last character from a string and returns it.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to pop.
///
/// # Returns
///
/// `NSTDUnichar chr` - The removed character, or the Unicode replacement character on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_pop(string: &mut NSTDString) -> NSTDUnichar {
    // SAFETY: `NSTDString` is always UTF-8 encoded.
    let str = unsafe { core::str::from_utf8_unchecked(string.bytes.as_slice()) };
    if let Some(chr) = str.chars().last() {
        let len = nstd_vec_len(&string.bytes) - chr.len_utf8();
        nstd_vec_truncate(&mut string.bytes, len);
        return chr as NSTDUnichar;
    }
    char::REPLACEMENT_CHARACTER as NSTDUnichar
}

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString string` - A pointer to the string to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_string_free(string: NSTDString) {}
