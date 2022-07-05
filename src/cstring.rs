//! A dynamically sized, null terminated, C string.
use crate::{
    core::{
        cstr::{
            nstd_core_cstr_const_as_bytes, nstd_core_cstr_const_new, nstd_core_cstr_mut_new,
            NSTDCStrConst, NSTDCStrMut,
        },
        def::{NSTDChar, NSTDErrorCode},
        slice::NSTDSliceConst,
    },
    vec::{
        nstd_vec_as_slice_const, nstd_vec_clone, nstd_vec_extend, nstd_vec_free, nstd_vec_get_mut,
        nstd_vec_new_with_cap, nstd_vec_pop, nstd_vec_push, NSTDVec,
    },
    NSTDUSize,
};
use core::ptr::addr_of;

/// A dynamically sized, null terminated, C string.
///
/// Managed C strings (`NSTDCString`) will always contain a null byte until freed.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCString {
    /// The underlying vector of `NSTDChar`s.
    pub bytes: NSTDVec,
}

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if allocating for the null byte fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_new() -> NSTDCString {
    nstd_cstring_new_with_cap(1)
}

/// Creates a new `NSTDCString` initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUSize cap` - The number of bytes to preallocate.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
///
/// # Panics
///
/// This function will panic if either `cap` is zero or allocating fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_new_with_cap(cap: NSTDUSize) -> NSTDCString {
    let mut bytes = nstd_vec_new_with_cap(1, cap);
    let nul: NSTDChar = 0;
    unsafe { assert!(nstd_vec_push(&mut bytes, addr_of!(nul).cast()) == 0) };
    NSTDCString { bytes }
}

/// Creates a deep copy of an `NSTDCString`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string to create a deep copy of.
///
/// # Returns
///
/// `NSTDCString cloned` - A new deep copy of `cstring`.
///
/// # Panics
///
/// This function will panic if allocating for the new C string fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_clone(cstring: &NSTDCString) -> NSTDCString {
    NSTDCString {
        bytes: nstd_vec_clone(&cstring.bytes),
    }
}

/// Creates a C string slice containing the contents of `cstring` (excluding the null byte).
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice.
///
/// # Safety
///
/// `cstring`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_cstring_as_cstr(cstring: &NSTDCString) -> NSTDCStrConst {
    nstd_core_cstr_const_new(cstring.bytes.buffer.ptr.raw.cast(), cstring.bytes.len - 1)
}

/// Creates a C string slice containing the contents of `cstring` (excluding the null byte).
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStrMut cstr` - The new C string slice.
///
/// # Safety
///
/// `cstring`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_cstring_as_cstr_mut(cstring: &mut NSTDCString) -> NSTDCStrMut {
    nstd_core_cstr_mut_new(cstring.bytes.buffer.ptr.raw.cast(), cstring.bytes.len - 1)
}

/// Returns an immutable byte slice of the C string's active data, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - The C string's active data.
///
/// # Safety
///
/// `cstring`'s data must remain valid while the returned slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_cstring_as_bytes(cstring: &NSTDCString) -> NSTDSliceConst {
    nstd_vec_as_slice_const(&cstring.bytes)
}

/// Appends an `NSTDChar` to the end of an `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// - `NSTDChar chr` - The C char to append to the C string.
///
/// # Panics
///
/// This operation panics if `chr` cannot be appended to the C string.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_push(cstring: &mut NSTDCString, chr: NSTDChar) {
    unsafe {
        // Push a new null byte onto the end of the C string.
        let nulpos = cstring.bytes.len - 1;
        let mut nul = nstd_vec_get_mut(&mut cstring.bytes, nulpos).cast::<NSTDChar>();
        assert!(nstd_vec_push(&mut cstring.bytes, nul.cast()) == 0);
        // Write `chr` over the old null byte.
        nul = nstd_vec_get_mut(&mut cstring.bytes, nulpos).cast();
        *nul = chr;
    }
}

/// Appends a C string slice to the end of a C string.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// - `const NSTDCStrConst *cstr` - The C string slice to append to the end of `cstring`.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if reserving memory for the push fails.
///
/// # Panics
///
/// This operation will panic if appending the new null byte to the end of the C string fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_push_cstr(
    cstring: &mut NSTDCString,
    cstr: &NSTDCStrConst,
) -> NSTDErrorCode {
    unsafe {
        // Pop the old null byte.
        let nul = *nstd_vec_pop(&mut cstring.bytes).cast::<NSTDChar>();
        // Append the C string slice.
        let bytes = nstd_core_cstr_const_as_bytes(cstr);
        let errc = nstd_vec_extend(&mut cstring.bytes, &bytes);
        // Push a new null byte.
        assert!(nstd_vec_push(&mut cstring.bytes, addr_of!(nul).cast()) == 0);
        errc
    }
}

/// Removes the last character from a C string and returns it.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDChar chr` - The removed character, or null if the C string is empty.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_pop(cstring: &mut NSTDCString) -> NSTDChar {
    let mut ret = 0;
    if cstring.bytes.len > 1 {
        unsafe {
            // Write the last character in the C string to the return value.
            let lastpos = cstring.bytes.len - 2;
            let last = nstd_vec_get_mut(&mut cstring.bytes, lastpos).cast();
            ret = *last;
            // Set the last byte to null.
            *last = 0;
            // Pop the old null byte.
            nstd_vec_pop(&mut cstring.bytes);
        }
    }
    ret
}

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_free(cstring: NSTDCString) {
    nstd_vec_free(cstring.bytes);
}
