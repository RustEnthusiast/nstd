//! Dynamically sized UTF-8 encoded byte string.
extern crate alloc;
use crate::{
    alloc::NSTDAllocError,
    core::{
        def::{NSTDByte, NSTDErrorCode},
        slice::{nstd_core_slice_new, NSTDSlice},
        str::{
            nstd_core_str_as_bytes, nstd_core_str_from_bytes_unchecked, nstd_core_str_len,
            nstd_core_str_mut_from_bytes_unchecked, NSTDStr, NSTDStrMut,
        },
    },
    vec::{
        nstd_vec_as_ptr, nstd_vec_as_slice, nstd_vec_as_slice_mut, nstd_vec_cap, nstd_vec_clone,
        nstd_vec_extend, nstd_vec_from_slice, nstd_vec_len, nstd_vec_new, nstd_vec_new_with_cap,
        nstd_vec_truncate, NSTDVec,
    },
    NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt,
    NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8, NSTDUnichar,
};
use alloc::string::ToString;

/// Generates the `nstd_string_from_[i|u|f]*` functions.
macro_rules! gen_from_primitive {
    (
        $(#[$meta:meta])*
        $name: ident, $FromT: ty
    ) => {
        $(#[$meta])*
        ///
        /// # Panics
        ///
        /// Panics if allocating fails.
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $name(v: $FromT) -> NSTDString {
            NSTDString::from_str(&v.to_string())
        }
    };
}

/// Dynamically sized UTF-8 encoded byte string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDString {
    /// The underlying UTF-8 encoded byte buffer.
    bytes: NSTDVec,
}
impl NSTDString {
    /// Creates a new [NSTDString] from a Rust &[str].
    ///
    /// # Panics
    ///
    /// Panics if allocating fails.
    #[inline]
    pub(crate) fn from_str(str: &str) -> Self {
        NSTDString {
            bytes: NSTDVec::from_slice(str.as_bytes()),
        }
    }
}

/// Creates a new instance of `NSTDString`.
///
/// # Returns
///
/// `NSTDString string` - The new string.
///
/// # Example
///
/// ```
/// use nstd_sys::string::nstd_string_new;
///
/// let string = nstd_string_new();
/// ```
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
/// - `NSTDUInt cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDString string` - The new string.
///
/// # Panics
///
/// This function will panic if `cap` is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::string::nstd_string_new_with_cap;
///
/// let string = nstd_string_new_with_cap(20);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_new_with_cap(cap: NSTDUInt) -> NSTDString {
    NSTDString {
        bytes: nstd_vec_new_with_cap(1, cap),
    }
}

/// Creates an owned version of an unowned string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The unowned string slice.
///
/// # Returns
///
/// `NSTDString string` The new owned version of `str`.
///
/// # Panics
///
/// This operation will panic if allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure that `str`'s data is valid for reads.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::str::nstd_core_str_from_raw_cstr, string::nstd_string_from_str};
///
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr("Hello, world!\0".as_ptr().cast());
///     let string = nstd_string_from_str(&str);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_from_str(str: &NSTDStr) -> NSTDString {
    let bytes = nstd_core_str_as_bytes(str);
    NSTDString {
        bytes: nstd_vec_from_slice(&bytes),
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
/// `NSTDStr str` - The new string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_as_str(string: &NSTDString) -> NSTDStr {
    let bytes = nstd_vec_as_slice(&string.bytes);
    // SAFETY: The string's bytes are always be UTF-8 encoded.
    unsafe { nstd_core_str_from_bytes_unchecked(&bytes) }
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
/// `NSTDSlice bytes` - The string's active data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_as_bytes(string: &NSTDString) -> NSTDSlice {
    nstd_vec_as_slice(&string.bytes)
}

/// Returns a raw pointer to a string's memory.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_as_ptr(string: &NSTDString) -> *const NSTDByte {
    nstd_vec_as_ptr(&string.bytes).cast()
}

/// Returns ownership of an `NSTDString`'s raw data, taking ownership of said string.
///
/// # Parameters:
///
/// - `NSTDString string` - The string.
///
/// # Returns
///
/// `NSTDVec bytes` - The string's raw data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_into_bytes(string: NSTDString) -> NSTDVec {
    string.bytes
}

/// Returns the number of Unicode characters in a string.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the string.
///
/// # Panics
///
/// This operation will panic if the string's length is greater than `NSTDInt`'s max value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_len(string: &NSTDString) -> NSTDUInt {
    let str = nstd_string_as_str(string);
    // SAFETY: The string's data is valid here.
    unsafe { nstd_core_str_len(&str) }
}

/// Returns the number of bytes a string contains.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUInt byte_len` - The number of bytes in the string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_byte_len(string: &NSTDString) -> NSTDUInt {
    nstd_vec_len(&string.bytes)
}

/// Returns a string's capacity.
///
/// This is the max number of *bytes* the string can contain without reallocating.
///
/// # Parameters:
///
/// - `const NSTDString *string` - The string.
///
/// # Returns
///
/// `NSTDUInt cap` - The string's capacity.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_cap(string: &NSTDString) -> NSTDUInt {
    nstd_vec_cap(&string.bytes)
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
///
/// # Panics
///
/// Panics if the current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     string::{nstd_string_new, nstd_string_push},
///     NSTDUnichar,
/// };
///
/// let mut string = nstd_string_new();
/// assert!(nstd_string_push(&mut string, '🦀' as NSTDUnichar) == 0);
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_push(string: &mut NSTDString, chr: NSTDUnichar) -> NSTDErrorCode {
    if let Some(chr) = char::from_u32(chr) {
        let mut buf = [0; 4];
        chr.encode_utf8(&mut buf);
        let buf = nstd_core_slice_new(buf.as_ptr().cast(), 1, chr.len_utf8());
        // SAFETY: `buf`'s data is stored on the stack.
        let errc = unsafe { nstd_vec_extend(&mut string.bytes, &buf) };
        return (errc != NSTDAllocError::NSTD_ALLOC_ERROR_NONE).into();
    }
    1
}

/// Appends a string slice to the end of a string.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string.
///
/// - `const NSTDStr *str` - The string slice to append to the end of `string`.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// Panics if the current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This function will cause undefined behavior in the case where `str`'s data is no longer valid.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     core::str::nstd_core_str_from_raw_cstr,
///     string::{nstd_string_new, nstd_string_push_str},
/// };
///
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr("Hello, 🌎!\0".as_ptr().cast());
///     let mut string = nstd_string_new();
///     assert!(nstd_string_push_str(&mut string, &str) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_push_str(
    string: &mut NSTDString,
    str: &NSTDStr,
) -> NSTDAllocError {
    let str_bytes = nstd_core_str_as_bytes(str);
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
///
/// # Panics
///
/// This operation will panic if the string's length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::str::nstd_core_str_from_raw_cstr_with_null,
///     string::{nstd_string_from_str, nstd_string_pop},
/// };
///
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr_with_null("Hello, world!\0".as_ptr().cast());
///     let mut string = nstd_string_from_str(&str);
///     assert!(nstd_string_pop(&mut string) == 0);
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_pop(string: &mut NSTDString) -> NSTDUnichar {
    assert!(nstd_vec_len(&string.bytes) <= isize::MAX as usize);
    // SAFETY: `NSTDString` is always UTF-8 encoded.
    let str = unsafe { core::str::from_utf8_unchecked(string.bytes.as_slice()) };
    if let Some(chr) = str.chars().last() {
        let len = nstd_vec_len(&string.bytes) - chr.len_utf8();
        nstd_vec_truncate(&mut string.bytes, len);
        return chr as NSTDUnichar;
    }
    char::REPLACEMENT_CHARACTER as NSTDUnichar
}

gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDFloat32`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDFloat32 v` - The 32-bit floating-point value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 32-bit floating-point value as a string.
    nstd_string_from_f32,
    NSTDFloat32
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDFloat64`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDFloat64 v` - The 64-bit floating-point value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 64-bit floating-point value as a string.
    nstd_string_from_f64,
    NSTDFloat64
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDInt`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDInt v` - The arch-bit signed integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The arch-bit signed integer value as a string.
    nstd_string_from_int,
    NSTDInt
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDUInt`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt v` - The arch-bit unsigned integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The arch-bit unsigned integer value as a string.
    nstd_string_from_uint,
    NSTDUInt
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDInt8`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDInt8 v` - The 8-bit signed integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 8-bit signed integer value as a string.
    nstd_string_from_i8,
    NSTDInt8
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDUInt8`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt8 v` - The 8-bit unsigned integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 8-bit unsigned integer value as a string.
    nstd_string_from_u8,
    NSTDUInt8
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDInt16`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDInt16 v` - The 16-bit signed integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 16-bit signed integer value as a string.
    nstd_string_from_i16,
    NSTDInt16
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDUInt16`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt16 v` - The 16-bit unsigned integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 16-bit unsigned integer value as a string.
    nstd_string_from_u16,
    NSTDUInt16
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDInt32`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDInt32 v` - The 32-bit signed integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 32-bit signed integer value as a string.
    nstd_string_from_i32,
    NSTDInt32
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDUInt32`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt32 v` - The 32-bit unsigned integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 32-bit unsigned integer value as a string.
    nstd_string_from_u32,
    NSTDUInt32
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDInt64`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDInt64 v` - The 64-bit signed integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 64-bit signed integer value as a string.
    nstd_string_from_i64,
    NSTDInt64
);
gen_from_primitive!(
    /// Creates a new `NSTDString` from an `NSTDUInt64`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt64 v` - The 64-bit unsigned integer value.
    ///
    /// # Returns
    ///
    /// `NSTDString string` - The 64-bit unsigned integer value as a string.
    nstd_string_from_u64,
    NSTDUInt64
);

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString string` - The string to free.
///
/// # Panics
///
/// Panics if deallocating fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_string_free(string: NSTDString) {}
