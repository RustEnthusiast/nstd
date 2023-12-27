//! A dynamically sized, null terminated, C string.
use crate::{
    core::{
        alloc::{
            NSTDAllocError::{self, NSTD_ALLOC_ERROR_NONE},
            NSTDAllocator,
        },
        cstr::{
            nstd_core_cstr_as_bytes, nstd_core_cstr_get_null, nstd_core_cstr_is_null_terminated,
            nstd_core_cstr_new_unchecked, NSTDCStr,
        },
        optional::NSTDOptional,
        slice::NSTDSlice,
    },
    vec::{
        nstd_vec_allocator, nstd_vec_as_ptr, nstd_vec_as_slice, nstd_vec_cap, nstd_vec_clear,
        nstd_vec_clone, nstd_vec_extend, nstd_vec_from_slice, nstd_vec_get_mut, nstd_vec_len,
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
pub struct NSTDCString<'a> {
    /// The underlying vector of `NSTDChar`s.
    bytes: NSTDVec<'a>,
}

/// Represents an optional value of type `NSTDCString`.
pub type NSTDOptionalCString<'a> = NSTDOptional<NSTDCString<'a>>;

/// Creates a new empty `NSTDCString`.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new C string on success, or an uninitialized "none" variant
/// if allocating for the C string's null terminator fails.
///
/// # Example
///
/// ```
/// use nstd_sys::{alloc::NSTD_ALLOCATOR, cstring::nstd_cstring_new};
///
/// let cstring = unsafe { nstd_cstring_new(&NSTD_ALLOCATOR) };
/// ```
#[inline]
#[nstdapi]
pub fn nstd_cstring_new(allocator: &NSTDAllocator) -> NSTDOptionalCString<'_> {
    nstd_cstring_new_with_cap(allocator, 1)
}

/// Creates a new `NSTDCString` initialized with the given capacity.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new C string on success, or an uninitialized "none" variant
/// if allocating fails.
///
/// # Example
///
/// ```
/// use nstd_sys::{alloc::NSTD_ALLOCATOR, cstring::nstd_cstring_new_with_cap};
///
/// let cstring = unsafe { nstd_cstring_new_with_cap(&NSTD_ALLOCATOR, 10) };
/// ```
#[inline]
#[nstdapi]
pub fn nstd_cstring_new_with_cap(
    allocator: &NSTDAllocator,
    cap: NSTDUInt,
) -> NSTDOptionalCString<'_> {
    if let NSTDOptional::Some(mut bytes) = nstd_vec_new_with_cap(allocator, 1, 1, cap) {
        let nul: NSTDChar = 0;
        // SAFETY: `nul` is stored on the stack.
        if unsafe { nstd_vec_push(&mut bytes, addr_of!(nul).cast()) } == NSTD_ALLOC_ERROR_NONE {
            return NSTDOptional::Some(NSTDCString { bytes });
        }
    }
    NSTDOptional::None
}

/// Creates an owned version of an unowned C string slice.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `const NSTDCStr *cstr` - The unowned C string slice.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new owned version of `cstr` on success, or an uninitialized
/// "none" variant if `cstr` contains a null byte or allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure that `cstr`'s data is valid for reads.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR, core::cstr::nstd_core_cstr_from_raw, cstring::nstd_cstring_from_cstr,
/// };
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw("C string\0".as_ptr().cast());
///     let cstring = nstd_cstring_from_cstr(&NSTD_ALLOCATOR, &cstr);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_cstring_from_cstr<'a>(
    allocator: &'a NSTDAllocator,
    cstr: &NSTDCStr,
) -> NSTDOptionalCString<'a> {
    match nstd_core_cstr_get_null(cstr).is_null() {
        true => nstd_cstring_from_cstr_unchecked(allocator, cstr),
        false => NSTDOptional::None,
    }
}

/// Creates an owned version of an unowned C string slice without checking if the slice contains
/// any null bytes.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `const NSTDCStr *cstr` - The unowned C string slice.
///
/// # Returns
///
/// `NSTDOptionalCString cstring` - The new owned version of `cstr` on success, or an uninitialized
/// "none" variant if allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure the following preconditions:
///
/// - `cstr`'s data is valid for reads.
///
/// - `cstr` does not contain any null (`'\0'`) bytes.
#[nstdapi]
pub unsafe fn nstd_cstring_from_cstr_unchecked<'a>(
    allocator: &'a NSTDAllocator,
    cstr: &NSTDCStr,
) -> NSTDOptionalCString<'a> {
    let bytes = nstd_core_cstr_as_bytes(cstr);
    if let NSTDOptional::Some(mut bytes) = nstd_vec_from_slice(allocator, &bytes, 1) {
        let null: NSTDChar = 0;
        let null = addr_of!(null).cast();
        if nstd_vec_push(&mut bytes, null) == NSTD_ALLOC_ERROR_NONE {
            return NSTDOptional::Some(NSTDCString { bytes });
        }
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
pub fn nstd_cstring_from_bytes(bytes: NSTDVec<'_>) -> NSTDOptionalCString<'_> {
    assert!(nstd_vec_stride(&bytes) == 1);
    let ptr = nstd_vec_as_ptr(&bytes).cast();
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
pub fn nstd_cstring_clone<'a>(cstring: &NSTDCString<'a>) -> NSTDOptionalCString<'a> {
    match nstd_vec_clone(&cstring.bytes) {
        NSTDOptional::Some(bytes) => NSTDOptional::Some(NSTDCString { bytes }),
        NSTDOptional::None => NSTDOptional::None,
    }
}

/// Returns an immutable reference to a C string's allocator.
///
/// # Parameters:
///
/// - `const NSTDCString *cstring` - The C string.
///
/// # Returns
///
/// `const NSTDAllocator *allocator` - The C string's allocator.
#[inline]
#[nstdapi]
pub const fn nstd_cstring_allocator<'a>(cstring: &NSTDCString<'a>) -> &'a NSTDAllocator {
    nstd_vec_allocator(&cstring.bytes)
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
pub const fn nstd_cstring_as_cstr(cstring: &NSTDCString<'_>) -> NSTDCStr {
    let ptr = nstd_vec_as_ptr(&cstring.bytes);
    let len = nstd_vec_len(&cstring.bytes);
    // SAFETY: `ptr` is never null, owned C strings can never be longer than `NSTDInt`'s max value.
    unsafe { nstd_core_cstr_new_unchecked(ptr.cast(), len) }
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
pub const fn nstd_cstring_as_bytes(cstring: &NSTDCString<'_>) -> NSTDSlice {
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
pub const fn nstd_cstring_as_ptr(cstring: &NSTDCString<'_>) -> *const NSTDChar {
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
#[allow(clippy::missing_const_for_fn)]
pub fn nstd_cstring_into_bytes(cstring: NSTDCString<'_>) -> NSTDVec<'_> {
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
#[allow(clippy::arithmetic_side_effects)]
pub const fn nstd_cstring_len(cstring: &NSTDCString<'_>) -> NSTDUInt {
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
pub const fn nstd_cstring_len_with_null(cstring: &NSTDCString<'_>) -> NSTDUInt {
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
pub const fn nstd_cstring_cap(cstring: &NSTDCString<'_>) -> NSTDUInt {
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
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     cstring::{nstd_cstring_new, nstd_cstring_push},
///     NSTDChar,
/// };
///
/// unsafe {
///     let mut cstring = nstd_cstring_new(&NSTD_ALLOCATOR).unwrap();
///     nstd_cstring_push(&mut cstring, b'!' as NSTDChar);
/// }
/// ```
#[nstdapi]
pub fn nstd_cstring_push(cstring: &mut NSTDCString<'_>, chr: NSTDChar) -> NSTDAllocError {
    if chr != 0 {
        // SAFETY: C strings always contain an exclusive null byte at the end.
        unsafe {
            // Push a new null byte onto the end of the C string.
            let nul: NSTDChar = 0;
            let errc = nstd_vec_push(&mut cstring.bytes, addr_of!(nul).cast());
            if errc != NSTD_ALLOC_ERROR_NONE {
                return errc;
            }
            // Write `chr` over the old null byte.
            #[allow(clippy::arithmetic_side_effects)]
            let nulpos = nstd_vec_len(&cstring.bytes) - 2;
            let nul = nstd_vec_get_mut(&mut cstring.bytes, nulpos).cast();
            *nul = chr;
        }
    }
    NSTD_ALLOC_ERROR_NONE
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
///     alloc::NSTD_ALLOCATOR,
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, cstr::nstd_core_cstr_from_raw},
///     cstring::{nstd_cstring_new, nstd_cstring_push_cstr},
///     NSTDChar,
/// };
///
/// unsafe {
///     let mut cstring = nstd_cstring_new(&NSTD_ALLOCATOR).unwrap();
///     let cstr = nstd_core_cstr_from_raw("baNaNa\0".as_ptr().cast());
///     assert!(nstd_cstring_push_cstr(&mut cstring, &cstr) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_cstring_push_cstr(
    cstring: &mut NSTDCString<'_>,
    cstr: &NSTDCStr,
) -> NSTDAllocError {
    // Make sure the C string slice doesn't contain a null byte.
    assert!(nstd_core_cstr_get_null(cstr).is_null());
    // Pop the old null byte.
    let nul = *nstd_vec_pop(&mut cstring.bytes).cast::<NSTDChar>();
    // Append the C string slice.
    let bytes = nstd_core_cstr_as_bytes(cstr);
    let errc = nstd_vec_extend(&mut cstring.bytes, &bytes);
    // Push a new null byte.
    let pusherrc = nstd_vec_push(&mut cstring.bytes, addr_of!(nul).cast());
    assert!(pusherrc == NSTD_ALLOC_ERROR_NONE);
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
///     alloc::NSTD_ALLOCATOR,
///     core::cstr::nstd_core_cstr_from_raw,
///     cstring::{nstd_cstring_from_cstr, nstd_cstring_pop},
///     NSTDChar,
/// };
///
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw("123\0".as_ptr().cast());
///     let mut cstring = nstd_cstring_from_cstr(&NSTD_ALLOCATOR, &cstr).unwrap();
///     assert!(nstd_cstring_pop(&mut cstring) == b'3' as NSTDChar);
/// }
/// ```
#[nstdapi]
pub fn nstd_cstring_pop(cstring: &mut NSTDCString<'_>) -> NSTDChar {
    let mut ret = 0;
    let len = nstd_cstring_len(cstring);
    if len > 0 {
        // SAFETY: The C string's length is at least 1.
        unsafe {
            // Write the last character in the C string to the return value.
            #[allow(clippy::arithmetic_side_effects)]
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
pub fn nstd_cstring_clear(cstring: &mut NSTDCString<'_>) {
    nstd_vec_clear(&mut cstring.bytes);
}

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString cstring` - The C string to free.
#[inline]
#[nstdapi]
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
pub fn nstd_cstring_free(cstring: NSTDCString<'_>) {}
