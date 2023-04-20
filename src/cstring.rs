//! A dynamically sized, null terminated, C string.
use crate::{
    alloc::NSTDAllocError,
    core::{
        cstr::{
            nstd_core_cstr_as_bytes, nstd_core_cstr_get_null, nstd_core_cstr_is_null_terminated,
            nstd_core_cstr_new_unchecked, NSTDCStr,
        },
        optional::{gen_optional, NSTDOptional},
        slice::NSTDSlice,
    },
    vec::{
        nstd_vec_as_ptr, nstd_vec_as_slice, nstd_vec_cap, nstd_vec_clear, nstd_vec_clone,
        nstd_vec_extend, nstd_vec_from_slice, nstd_vec_get_mut, nstd_vec_len,
        nstd_vec_new_with_cap, nstd_vec_pop, nstd_vec_push, nstd_vec_stride, NSTDVec,
    },
    NSTDChar, NSTDUInt,
};
use core::ptr::addr_of;
use nstdapi::nstdapi;

/// A dynamically sized, null terminated, C string.
///
/// Managed C strings (`NSTDCString`) will always contain a null byte until freed.
#[nstdapi]
#[derive(Debug)]
pub struct NSTDCString {
    /// The underlying vector of `NSTDChar`s.
    bytes: NSTDVec,
}
gen_optional!(NSTDOptionalCString, NSTDCString);

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new C string on success, or an uninitialized "none" variant
/// if allocating for the C string's null terminator fails.
///
/// # Example
///
/// ```
/// use nstd_sys::cstring::nstd_cstring_new;
///
/// let cstring = nstd_cstring_new();
/// ```
#[inline]
#[nstdapi]
pub fn nstd_cstring_new() -> NSTDOptionalCString {
    nstd_cstring_new_with_cap(1)
}

/// Creates a new `NSTDCString` initialized with the given capacity.
///
/// # Parameters:
///
/// - `NSTDUInt cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new C string on success, or an uninitialized "none" variant
/// if allocating fails.
///
/// # Panics
///
/// This function will panic if `cap` is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::cstring::nstd_cstring_new_with_cap;
///
/// let cstring = nstd_cstring_new_with_cap(10);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_cstring_new_with_cap(cap: NSTDUInt) -> NSTDOptionalCString {
    let mut bytes = nstd_vec_new_with_cap(1, cap);
    let nul: NSTDChar = 0;
    // SAFETY: `nul` is stored on the stack.
    match unsafe { nstd_vec_push(&mut bytes, addr_of!(nul) as _) } {
        NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDOptional::Some(NSTDCString { bytes }),
        _ => NSTDOptional::None,
    }
}

/// Creates an owned version of an unowned C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The unowned C string slice.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new owned version of `cstr` on success, or an uninitialized
/// "none" variant if `cstr` contains a null byte or allocating fails.
///
/// # Panics
///
/// This operation will panic if allocating for the C string's null byte fails.
///
/// # Safety
///
/// The caller of this function must ensure that `cstr`'s data is valid for reads.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::cstr::nstd_core_cstr_from_raw, cstring::nstd_cstring_from_cstr};
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw("C string\0".as_ptr().cast());
///     let cstring = nstd_cstring_from_cstr(&cstr);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_cstring_from_cstr(cstr: &NSTDCStr) -> NSTDOptionalCString {
    match nstd_core_cstr_get_null(cstr).is_null() {
        true => nstd_cstring_from_cstr_unchecked(cstr),
        false => NSTDOptional::None,
    }
}

/// Creates an owned version of an unowned C string slice without checking if the slice contains
/// any null bytes.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The unowned C string slice.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new owned version of `cstr` on success, or an uninitialized
/// "none" variant if allocating fails.
///
/// # Panics
///
/// This operation will panic if allocating for the C string's null byte fails.
///
/// # Safety
///
/// The caller of this function must ensure the following preconditions:
///
/// - `cstr`'s data is valid for reads.
///
/// - `cstr` does not contain any null (`'\0'`) bytes.
#[nstdapi]
pub unsafe fn nstd_cstring_from_cstr_unchecked(cstr: &NSTDCStr) -> NSTDOptionalCString {
    let bytes = nstd_core_cstr_as_bytes(cstr);
    if let NSTDOptional::Some(mut bytes) = nstd_vec_from_slice(&bytes) {
        let null: NSTDChar = 0;
        let null = addr_of!(null).cast();
        assert!(nstd_vec_push(&mut bytes, null) == NSTDAllocError::NSTD_ALLOC_ERROR_NONE);
        return NSTDOptional::Some(NSTDCString { bytes });
    }
    NSTDOptional::None
}

/// Creates a new C string from owned data.
///
/// # Parameters:
///
/// - `NSTDVec bytes` - The bytes to take ownership of.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new C string with ownership of `bytes` on success, or an
/// uninitialized "none" variant if `bytes` does not end with a null (`\0`) byte.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
#[nstdapi]
pub fn nstd_cstring_from_bytes(bytes: NSTDVec) -> NSTDOptionalCString {
    let ptr = nstd_vec_as_ptr(&bytes) as *const NSTDChar;
    assert!(!ptr.is_null() && nstd_vec_stride(&bytes) == 1);
    // SAFETY: `ptr` is non-null, vector length's can never be greater than `NSTDInt`'s max value.
    let cstr = unsafe { nstd_core_cstr_new_unchecked(ptr, nstd_vec_len(&bytes)) };
    // SAFETY: `cstr`'s data is owned by `bytes`.
    match unsafe { nstd_core_cstr_is_null_terminated(&cstr) } {
        true => NSTDOptional::Some(NSTDCString { bytes }),
        false => NSTDOptional::None,
    }
}

/// Creates a deep copy of an `NSTDCString`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string to create a deep copy of.
///
/// # Returns
///
/// `NSTDOptionalCString cloned` - A new deep copy of `cstring` on success, or an uninitialized
/// "none" variant if allocating fails.
#[inline]
#[nstdapi]
pub fn nstd_cstring_clone(cstring: &NSTDCString) -> NSTDOptionalCString {
    match nstd_vec_clone(&cstring.bytes) {
        NSTDOptional::Some(bytes) => NSTDOptional::Some(NSTDCString { bytes }),
        _ => NSTDOptional::None,
    }
}

/// Creates a C string slice containing the contents of `cstring`.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
#[nstdapi]
pub fn nstd_cstring_as_cstr(cstring: &NSTDCString) -> NSTDCStr {
    let ptr = nstd_vec_as_ptr(&cstring.bytes);
    let len = nstd_vec_len(&cstring.bytes);
    // SAFETY: `ptr` is never null, owned C strings can never be longer than `NSTDInt`'s max value.
    unsafe { nstd_core_cstr_new_unchecked(ptr as _, len) }
}

/// Returns an immutable byte slice of the C string's active data, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDSlice bytes` - The C string's active data.
#[inline]
#[nstdapi]
pub fn nstd_cstring_as_bytes(cstring: &NSTDCString) -> NSTDSlice {
    nstd_vec_as_slice(&cstring.bytes)
}

/// Returns a raw pointer to a C string's memory.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A raw pointer to a C string's memory.
#[inline]
#[nstdapi]
pub fn nstd_cstring_as_ptr(cstring: &NSTDCString) -> *const NSTDChar {
    nstd_vec_as_ptr(&cstring.bytes).cast()
}

/// Returns ownership of an `NSTDCString`'s raw data, taking ownership of said C string.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string.
///
/// # Returns
///
/// `NSTDVec bytes` - The C string's raw data.
#[inline]
#[nstdapi]
pub fn nstd_cstring_into_bytes(cstring: NSTDCString) -> NSTDVec {
    cstring.bytes
}

/// Returns the number of `char`s in a C string, excluding the null terminator.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string without it's null byte.
#[inline]
#[nstdapi]
pub fn nstd_cstring_len(cstring: &NSTDCString) -> NSTDUInt {
    nstd_vec_len(&cstring.bytes) - 1
}

/// Returns the number of `char`s in a C string, including the null terminator.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string including it's null byte.
#[inline]
#[nstdapi]
pub fn nstd_cstring_len_with_null(cstring: &NSTDCString) -> NSTDUInt {
    nstd_vec_len(&cstring.bytes)
}

/// Returns a C string's capacity.
///
/// This is the max number of *bytes* the C string can contain without reallocating.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `NSTDUInt cap` - The C string's capacity.
#[inline]
#[nstdapi]
pub fn nstd_cstring_cap(cstring: &NSTDCString) -> NSTDUInt {
    nstd_vec_cap(&cstring.bytes)
}

/// Appends an `NSTDChar` to the end of an `NSTDCString`.
///
/// This will have no effect if `chr` is a null byte (0).
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
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     cstring::{nstd_cstring_new, nstd_cstring_push},
///     NSTDChar,
/// };
///
/// let mut cstring = nstd_cstring_new();
/// nstd_cstring_push(&mut cstring, b'!' as NSTDChar);
/// ```
#[nstdapi]
pub fn nstd_cstring_push(cstring: &mut NSTDCString, chr: NSTDChar) {
    // SAFETY: C strings always contain an exclusive null byte at the end.
    unsafe {
        if chr != 0 {
            // Push a new null byte onto the end of the C string.
            let nulpos = nstd_vec_len(&cstring.bytes) - 1;
            let mut nul = nstd_vec_get_mut(&mut cstring.bytes, nulpos).cast::<NSTDChar>();
            let errc = nstd_vec_push(&mut cstring.bytes, nul.cast());
            assert!(errc == NSTDAllocError::NSTD_ALLOC_ERROR_NONE);
            // Write `chr` over the old null byte.
            nul = nstd_vec_get_mut(&mut cstring.bytes, nulpos).cast();
            *nul = chr;
        }
    }
}

/// Appends a C string slice to the end of a C string.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string.
///
/// - `const NSTDCStr *cstr` - The C string slice to append to the end of `cstring`.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `cstr` contains a null byte.
///
/// - Appending the new null byte to the end of the C string fails.
///
/// # Safety
///
/// This operation can cause undefined behavior in the case that `cstr`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     core::cstr::nstd_core_cstr_from_raw,
///     cstring::{nstd_cstring_new, nstd_cstring_push_cstr},
///     NSTDChar,
/// };
///
/// let mut cstring = nstd_cstring_new();
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw("baNaNa\0".as_ptr().cast());
///     assert!(nstd_cstring_push_cstr(&mut cstring, &cstr) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_cstring_push_cstr(cstring: &mut NSTDCString, cstr: &NSTDCStr) -> NSTDAllocError {
    // Make sure the C string slice doesn't contain a null byte.
    assert!(nstd_core_cstr_get_null(cstr).is_null());
    // Pop the old null byte.
    let nul = *nstd_vec_pop(&mut cstring.bytes).cast::<NSTDChar>();
    // Append the C string slice.
    let bytes = nstd_core_cstr_as_bytes(cstr);
    let errc = nstd_vec_extend(&mut cstring.bytes, &bytes);
    // Push a new null byte.
    let pusherrc = nstd_vec_push(&mut cstring.bytes, addr_of!(nul).cast());
    assert!(pusherrc == NSTDAllocError::NSTD_ALLOC_ERROR_NONE);
    errc
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
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::cstr::nstd_core_cstr_from_raw,
///     cstring::{nstd_cstring_from_cstr, nstd_cstring_pop},
///     NSTDChar,
/// };
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw("123\0".as_ptr().cast());
///     let mut cstring = nstd_cstring_from_cstr(&cstr).unwrap();
///     assert!(nstd_cstring_pop(&mut cstring) == b'3' as NSTDChar);
/// }
/// ```
#[nstdapi]
pub fn nstd_cstring_pop(cstring: &mut NSTDCString) -> NSTDChar {
    let mut ret = 0;
    let len = nstd_cstring_len(cstring);
    if len > 0 {
        // SAFETY: The C string's length is at least 1.
        unsafe {
            // Write the last character in the C string to the return value.
            let last = nstd_vec_get_mut(&mut cstring.bytes, len - 1).cast::<NSTDChar>();
            ret = *last;
            // Set the last byte to null.
            *last = 0;
            // Pop the old null byte.
            nstd_vec_pop(&mut cstring.bytes);
        }
    }
    ret
}

/// Sets a C string's length to zero.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string to clear.
#[inline]
#[nstdapi]
pub fn nstd_cstring_clear(cstring: &mut NSTDCString) {
    nstd_vec_clear(&mut cstring.bytes);
}

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_cstring_free(cstring: NSTDCString) {}
