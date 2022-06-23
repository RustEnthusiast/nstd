//! A dynamically sized, null terminated, C string.
use crate::{
    core::{
        cstr::{nstd_core_cstr_const_new, nstd_core_cstr_new, NSTDCStr, NSTDCStrConst},
        def::NSTDChar,
    },
    vec::{
        nstd_vec_clone, nstd_vec_free, nstd_vec_get, nstd_vec_new_with_cap, nstd_vec_pop,
        nstd_vec_push, NSTDVec,
    },
    NSTDUSize,
};
use core::ptr::addr_of;

/// A dynamically sized, null terminated, C string.
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
/// This function will panic if either `cap` is zero or allocating for the null byte fails.
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
/// This function will panic if either `cap` is zero or allocating for the null byte fails.
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
/// - `NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
///
/// # Safety
///
/// `cstring`'s data must remain valid while the returned C string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_cstring_as_cstr(cstring: &mut NSTDCString) -> NSTDCStr {
    nstd_core_cstr_new(
        cstring.bytes.buffer.ptr.raw.cast(),
        cstring.bytes.len.saturating_sub(1),
    )
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
pub unsafe extern "C" fn nstd_cstring_as_cstr_const(cstring: &NSTDCString) -> NSTDCStrConst {
    nstd_core_cstr_const_new(
        cstring.bytes.buffer.ptr.raw.cast(),
        cstring.bytes.len.saturating_sub(1),
    )
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
        let mut nul = nstd_vec_get(&mut cstring.bytes, nulpos).cast::<NSTDChar>();
        assert!(nstd_vec_push(&mut cstring.bytes, nul.cast()) == 0);
        // Write `chr` over the old null byte.
        nul = nstd_vec_get(&mut cstring.bytes, nulpos).cast();
        *nul = chr;
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
            let last = nstd_vec_get(&mut cstring.bytes, lastpos).cast::<NSTDChar>();
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
/// - `NSTDCString *cstring` - The C string to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_free(cstring: &mut NSTDCString) {
    nstd_vec_free(&mut cstring.bytes);
}
