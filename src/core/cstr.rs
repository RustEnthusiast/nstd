//! C string processing.
//!
//! # Note
//!
//! The functions in this module must be provided valid C strings, as they do not accept null
//! pointers.
use crate::core::{
    def::{NSTDBool, NSTDChar, NSTDUSize},
    ptr::nstd_core_ptr_new,
    slice::NSTDSlice,
};

/// Creates a byte slice over a C string, excluding the null terminator.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The C string to create a slice for.
///
/// # Returns
///
/// `NSTDSlice slice` - The new byte slice over the C string (without the null byte at the end).
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_as_slice(cstr: *mut NSTDChar) -> NSTDSlice {
    let len = nstd_core_cstr_len(cstr);
    NSTDSlice {
        ptr: nstd_core_ptr_new(cstr.cast(), 1),
        len,
    }
}

/// Creates a byte slice over a C string, including the null terminator.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The C string to create a slice for.
///
/// # Returns
///
/// `NSTDSlice slice` - The new byte slice over the C string (including the null byte at the end).
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_as_slice_with_null(cstr: *mut NSTDChar) -> NSTDSlice {
    let len = nstd_core_cstr_len_with_null(cstr);
    NSTDSlice {
        ptr: nstd_core_ptr_new(cstr.cast(), 1),
        len,
    }
}

/// Gets the length of a null terminated C string, excluding the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string, excluding the null byte.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_len(cstr: *const NSTDChar) -> NSTDUSize {
    let mut i = 0;
    while unsafe { *cstr.add(i) } != 0 {
        i += 1;
    }
    i
}

/// Gets the length of a null terminated C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string, including the null byte.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_len_with_null(cstr: *const NSTDChar) -> NSTDUSize {
    nstd_core_cstr_len(cstr) + 1
}

/// Compares two C strings, returning `NSTD_BOOL_TRUE` if they are lexicographically equal.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr1` - The first C string.
///
/// - `const NSTDChar *cstr2` - The second C string.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the C strings are lexicographically equal.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_cstr_compare(
    mut cstr1: *const NSTDChar,
    mut cstr2: *const NSTDChar,
) -> NSTDBool {
    // If the C strings point to the same data return true.
    if cstr1 == cstr2 {
        return NSTDBool::NSTD_BOOL_TRUE;
    }
    // Otherwise compare them lexicographically.
    unsafe {
        loop {
            if *cstr1 != *cstr2 {
                return NSTDBool::NSTD_BOOL_FALSE;
            } else if *cstr1 == 0 && *cstr2 == 0 {
                return NSTDBool::NSTD_BOOL_TRUE;
            }
            cstr1 = cstr1.add(1);
            cstr2 = cstr2.add(1);
        }
    }
}

/// Copies the contents of `src` to `dest`, excluding the null terminator.
///
/// # Note
///
/// If you already know how many bytes should be copied, `nstd_core_mem_copy[_overlapped]` should
/// be used instead.
///
/// # Parameters:
///
/// - `NSTDChar *dest` - The C string buffer to copy data to.
///
/// - `const NSTDChar *src` - The C string to copy data from.
///
/// # Safety
///
/// This operation is highly unsafe because it cannot guarantee that it won't write past the end of
/// `dest`'s memory buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_copy(mut dest: *mut NSTDChar, mut src: *const NSTDChar) {
    loop {
        if *src == 0 {
            break;
        }
        *dest = *src;
        dest = dest.add(1);
        src = src.add(1);
    }
}

/// Copies the contents of `src` to `dest`, including the null terminator.
///
/// # Note
///
/// If you already know how many bytes should be copied, `nstd_core_mem_copy[_overlapped]` should
/// be used instead.
///
/// # Parameters:
///
/// - `NSTDChar *dest` - The C string buffer to copy data to.
///
/// - `const NSTDChar *src` - The C string to copy data from.
///
/// # Safety
///
/// This operation is highly unsafe because it cannot guarantee that it won't write past the end of
/// `dest`'s memory buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_copy_with_null(
    mut dest: *mut NSTDChar,
    mut src: *const NSTDChar,
) {
    loop {
        *dest = *src;
        if *src == 0 {
            break;
        }
        dest = dest.add(1);
        src = src.add(1);
    }
}