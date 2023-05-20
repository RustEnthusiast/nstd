//! Dynamically sized UTF-8 encoded byte string.
extern crate alloc;
use crate::{
    alloc::{NSTDAllocError, NSTDAllocator, NSTD_ALLOCATOR},
    core::{
        def::NSTDByte,
        optional::NSTDOptional,
        slice::{nstd_core_slice_new_unchecked, NSTDSlice},
        str::{
            nstd_core_str_as_bytes, nstd_core_str_from_bytes_unchecked, nstd_core_str_len,
            nstd_core_str_mut_from_bytes_unchecked, NSTDStr, NSTDStrMut,
        },
        unichar::{NSTDOptionalUnichar, NSTDUnichar},
    },
    vec::{
        nstd_vec_as_ptr, nstd_vec_as_slice, nstd_vec_as_slice_mut, nstd_vec_cap, nstd_vec_clear,
        nstd_vec_clone, nstd_vec_extend, nstd_vec_from_slice, nstd_vec_len, nstd_vec_new,
        nstd_vec_new_with_cap, nstd_vec_truncate, NSTDVec,
    },
    NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt,
    NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8,
};
use alloc::string::ToString;
use nstdapi::nstdapi;

/// Generates the `nstd_string_from_[i|u|f]*` functions.
macro_rules! gen_from_primitive {
    (
        $(#[$meta:meta])*
        $name: ident, $FromT: ty
    ) => {
        $(#[$meta])*
        #[inline]
        #[nstdapi]
        pub fn $name(v: $FromT) -> NSTDOptionalString<'static> {
            NSTDString::from_str(&NSTD_ALLOCATOR, &v.to_string())
        }
    };
}

/// Dynamically sized UTF-8 encoded byte string.
#[nstdapi]
pub struct NSTDString<'a> {
    /// The underlying UTF-8 encoded byte buffer.
    bytes: NSTDVec<'a>,
}
impl<'a> NSTDString<'a> {
    /// Creates a new [NSTDString] from a Rust &[str].
    #[inline]
    pub(crate) fn from_str(allocator: &'a NSTDAllocator, str: &str) -> NSTDOptionalString<'a> {
        match NSTDVec::from_slice(allocator, str.as_bytes()) {
            NSTDOptional::Some(bytes) => NSTDOptional::Some(NSTDString { bytes }),
            _ => NSTDOptional::None,
        }
    }

    /// Returns a mutable reference to the string's buffer.
    ///
    /// # Safety
    ///
    /// When mutating the returned buffer, the buffer's data must remain valid UTF-8.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_mut_vec(&mut self) -> &mut NSTDVec<'a> {
        &mut self.bytes
    }
}

/// Represents an optional value of type `NSTDString`.
pub type NSTDOptionalString<'a> = NSTDOptional<NSTDString<'a>>;

/// Creates a new instance of `NSTDString`.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// # Returns
///
/// `NSTDString string` - The new string.
///
/// # Example
///
/// ```
/// use nstd_sys::{alloc::NSTD_ALLOCATOR, string::nstd_string_new};
///
/// let string = nstd_string_new(&NSTD_ALLOCATOR);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_string_new(allocator: &NSTDAllocator) -> NSTDString {
    NSTDString {
        bytes: nstd_vec_new(allocator, 1),
    }
}

/// Creates a new string initialized with the given capacity.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt cap` - The number of bytes to allocate ahead of time.
///
/// # Returns
///
/// `NSTDString string` - The new string.
///
/// # Example
///
/// ```
/// use nstd_sys::{alloc::NSTD_ALLOCATOR, string::nstd_string_new_with_cap};
///
/// let string = nstd_string_new_with_cap(&NSTD_ALLOCATOR, 20);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_string_new_with_cap(allocator: &NSTDAllocator, cap: NSTDUInt) -> NSTDString {
    NSTDString {
        bytes: nstd_vec_new_with_cap(allocator, 1, cap),
    }
}

/// Creates an owned version of an unowned string slice.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `const NSTDStr *str` - The unowned string slice.
///
/// # Returns
///
/// `NSTDOptionalString string` - The new owned version of `str` on success, or an uninitialized
/// "none" variant if allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure that `str`'s data is valid for reads.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR, core::str::nstd_core_str_from_raw_cstr, string::nstd_string_from_str,
/// };
///
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr("Hello, world!\0".as_ptr().cast()).unwrap();
///     let string = nstd_string_from_str(&NSTD_ALLOCATOR, &str);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_string_from_str<'a>(
    allocator: &'a NSTDAllocator,
    str: &NSTDStr,
) -> NSTDOptionalString<'a> {
    let bytes = nstd_core_str_as_bytes(str);
    match nstd_vec_from_slice(allocator, &bytes) {
        NSTDOptional::Some(bytes) => NSTDOptional::Some(NSTDString { bytes }),
        _ => NSTDOptional::None,
    }
}

/// Creates a new string from owned UTF-8 data.
///
/// # Parameters:
///
/// - `NSTDVec bytes` - The owned UTF-8 encoded buffer to take ownership of.
///
/// # Returns
///
/// `NSTDOptionalString string` - The new UTF-8 encoded string with ownership of `bytes` on success
/// or an uninitialized "none" variant if `bytes` contains invalid UTF-8.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
#[inline]
#[nstdapi]
pub fn nstd_string_from_bytes(bytes: NSTDVec) -> NSTDOptionalString {
    // SAFETY: We're ensuring that the vector is properly encoded as UTF-8.
    match core::str::from_utf8(unsafe { bytes.as_slice() }).is_ok() {
        true => NSTDOptional::Some(NSTDString { bytes }),
        false => NSTDOptional::None,
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
/// `NSTDOptionalString cloned` - A new deep copy of `string` on success, or an uninitialized
/// "none" variant if allocating fails.
#[inline]
#[nstdapi]
pub fn nstd_string_clone<'a>(string: &NSTDString<'a>) -> NSTDOptionalString<'a> {
    match nstd_vec_clone(&string.bytes) {
        NSTDOptional::Some(bytes) => NSTDOptional::Some(NSTDString { bytes }),
        _ => NSTDOptional::None,
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
#[nstdapi]
pub fn nstd_string_as_str(string: &NSTDString) -> NSTDStr {
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
#[nstdapi]
pub fn nstd_string_as_str_mut(string: &mut NSTDString) -> NSTDStrMut {
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
#[nstdapi]
pub fn nstd_string_as_bytes(string: &NSTDString) -> NSTDSlice {
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
#[nstdapi]
pub const fn nstd_string_as_ptr(string: &NSTDString) -> *const NSTDByte {
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
#[nstdapi]
pub fn nstd_string_into_bytes(string: NSTDString) -> NSTDVec {
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
#[inline]
#[nstdapi]
pub fn nstd_string_len(string: &NSTDString) -> NSTDUInt {
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
#[nstdapi]
pub const fn nstd_string_byte_len(string: &NSTDString) -> NSTDUInt {
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
#[nstdapi]
pub const fn nstd_string_cap(string: &NSTDString) -> NSTDUInt {
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
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{NSTDAllocError::NSTD_ALLOC_ERROR_NONE, NSTD_ALLOCATOR},
///     string::{nstd_string_new, nstd_string_push},
/// };
///
/// let mut string = nstd_string_new(&NSTD_ALLOCATOR);
/// assert!(nstd_string_push(&mut string, '🦀'.into()) == NSTD_ALLOC_ERROR_NONE);
/// ```
#[nstdapi]
pub fn nstd_string_push(string: &mut NSTDString, chr: NSTDUnichar) -> NSTDAllocError {
    let chr = char::from(chr);
    let mut buf = [0; 4];
    chr.encode_utf8(&mut buf);
    // SAFETY: `buf`'s data is stored on the stack, UTF-8 characters never occupy more than 4
    // bytes.
    unsafe {
        let buf = nstd_core_slice_new_unchecked(buf.as_ptr() as _, 1, chr.len_utf8());
        nstd_vec_extend(&mut string.bytes, &buf)
    }
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
/// # Safety
///
/// This function will cause undefined behavior in the case where `str`'s data is no longer valid.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{NSTDAllocError::NSTD_ALLOC_ERROR_NONE, NSTD_ALLOCATOR},
///     core::str::nstd_core_str_from_raw_cstr,
///     string::{nstd_string_new, nstd_string_push_str},
/// };
///
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr("Hello, 🌎!\0".as_ptr().cast()).unwrap();
///     let mut string = nstd_string_new(&NSTD_ALLOCATOR);
///     assert!(nstd_string_push_str(&mut string, &str) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_string_push_str(string: &mut NSTDString, str: &NSTDStr) -> NSTDAllocError {
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
/// `NSTDOptionalUnichar chr` - The removed character on success.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::{optional::NSTDOptional, str::nstd_core_str_from_raw_cstr_with_null},
///     string::{nstd_string_from_str, nstd_string_pop},
/// };
///
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr_with_null("Hello, world!\0".as_ptr().cast()).unwrap();
///     let mut string = nstd_string_from_str(&NSTD_ALLOCATOR, &str).unwrap();
///     assert!(nstd_string_pop(&mut string) == NSTDOptional::Some('\0'.into()));
/// }
/// ```
#[nstdapi]
pub fn nstd_string_pop(string: &mut NSTDString) -> NSTDOptionalUnichar {
    // SAFETY: `NSTDString` is always UTF-8 encoded.
    let str = unsafe { core::str::from_utf8_unchecked(string.bytes.as_slice()) };
    if let Some(chr) = str.chars().last() {
        let len = nstd_vec_len(&string.bytes) - chr.len_utf8();
        nstd_vec_truncate(&mut string.bytes, len);
        return NSTDOptional::Some(chr.into());
    }
    NSTDOptional::None
}

/// Sets a string's length to zero.
///
/// # Parameters:
///
/// - `NSTDString *string` - The string to clear.
#[inline]
#[nstdapi]
pub fn nstd_string_clear(string: &mut NSTDString) {
    nstd_vec_clear(&mut string.bytes);
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
    /// `NSTDOptionalString string` - The 32-bit floating-point value as a string.
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
    /// `NSTDOptionalString string` - The 64-bit floating-point value as a string.
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
    /// `NSTDOptionalString string` - The arch-bit signed integer value as a string.
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
    /// `NSTDOptionalString string` - The arch-bit unsigned integer value as a string.
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
    /// `NSTDOptionalString string` - The 8-bit signed integer value as a string.
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
    /// `NSTDOptionalString string` - The 8-bit unsigned integer value as a string.
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
    /// `NSTDOptionalString string` - The 16-bit signed integer value as a string.
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
    /// `NSTDOptionalString string` - The 16-bit unsigned integer value as a string.
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
    /// `NSTDOptionalString string` - The 32-bit signed integer value as a string.
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
    /// `NSTDOptionalString string` - The 32-bit unsigned integer value as a string.
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
    /// `NSTDOptionalString string` - The 64-bit signed integer value as a string.
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
    /// `NSTDOptionalString string` - The 64-bit unsigned integer value as a string.
    nstd_string_from_u64,
    NSTDUInt64
);

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString string` - The string to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_string_free(string: NSTDString) {}
