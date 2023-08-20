//! An unowned view into a UTF-8 encoded byte string.
use crate::{
    core::{
        cstr::{
            nstd_core_cstr_as_ptr, nstd_core_cstr_len, nstd_core_cstr_mut_as_ptr,
            nstd_core_cstr_mut_len, nstd_core_cstr_new_unchecked,
            raw::{nstd_core_cstr_raw_len, nstd_core_cstr_raw_len_with_null},
            NSTDCStr, NSTDCStrMut,
        },
        def::NSTDByte,
        optional::{
            gen_optional, NSTDOptional, NSTDOptionalFloat32, NSTDOptionalFloat64, NSTDOptionalInt,
            NSTDOptionalInt16, NSTDOptionalInt32, NSTDOptionalInt64, NSTDOptionalInt8,
            NSTDOptionalUInt, NSTDOptionalUInt16, NSTDOptionalUInt32, NSTDOptionalUInt64,
            NSTDOptionalUInt8,
        },
        range::NSTDURange,
        slice::{
            nstd_core_slice_as_ptr, nstd_core_slice_len, nstd_core_slice_mut_as_ptr,
            nstd_core_slice_mut_len, nstd_core_slice_mut_new_unchecked, nstd_core_slice_mut_stride,
            nstd_core_slice_new_unchecked, nstd_core_slice_stride, NSTDSlice, NSTDSliceMut,
        },
        unichar::NSTDOptionalUnichar,
    },
    NSTDChar, NSTDUInt, NSTD_INT_MAX,
};
use nstdapi::nstdapi;

/// Generates the `nstd_core_str_*_to_[i|u|f]*` functions.
macro_rules! gen_to_primitive {
    (
        $(#[$meta:meta])*
        $name: ident, $StrT: ty, $T: ty, $RetT: ty
    ) => {
        #[doc = concat!("Attempts to parse a string slice as an `", stringify!($T), "`.")]
        ///
        /// # Parameters:
        ///
        /// - `const NSTDStr *str` - The string slice.
        ///
        /// # Returns
        ///
        #[doc = concat!("`", stringify!($RetT), " v` - The parsed value, or none on error.")]
        ///
        /// # Safety
        ///
        /// This operation can cause undefined behavior in the event that `str`'s data is invalid.
        ///
        $(#[$meta])*
        #[inline]
        #[nstdapi]
        pub unsafe fn $name(str: &$StrT) -> $RetT {
            match str.as_str().parse() {
                Ok(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}

/// An immutable unowned view into a UTF-8 encoded byte string.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDStr {
    /// A raw pointer to the string's data.
    ptr: *const NSTDByte,
    /// The number of bytes in the string.
    len: NSTDUInt,
}
impl NSTDStr {
    /// Creates a new [`NSTDStr`] from a Rust [&str].
    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn from_str(str: &str) -> Self {
        Self {
            ptr: str.as_ptr(),
            len: str.len(),
        }
    }

    /// Creates a Rust string slice from this [`NSTDStr`].
    ///
    /// # Safety
    ///
    /// This string slice's data must remain valid UTF-8 and left unmodified while the returned
    /// string slice is in use.
    #[inline]
    pub(crate) const unsafe fn as_str(&self) -> &str {
        let bytes = core::slice::from_raw_parts(self.ptr, self.len);
        core::str::from_utf8_unchecked(bytes)
    }
}
gen_optional!(NSTDOptionalStr, NSTDStr);

/// Creates a new instance of an `NSTDStr` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new `NSTDStr` instance on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Safety
///
/// `cstr`'s data must be valid for reads of at least `cstr.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     cstr::nstd_core_cstr_from_raw,
///     str::{nstd_core_str_byte_len, nstd_core_str_from_cstr},
/// };
///
/// let s_str = "Hello, world!\0";
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw(s_str.as_ptr().cast());
///     let str = nstd_core_str_from_cstr(&cstr).unwrap();
///     assert!(nstd_core_str_byte_len(&str) == 13);
/// }
/// ```
#[nstdapi]
pub const unsafe fn nstd_core_str_from_cstr(cstr: &NSTDCStr) -> NSTDOptionalStr {
    match core::str::from_utf8(cstr.as_bytes()).is_ok() {
        true => {
            let ptr = nstd_core_cstr_as_ptr(cstr).cast();
            let len = nstd_core_cstr_len(cstr);
            NSTDOptional::Some(NSTDStr { ptr, len })
        }
        false => NSTDOptional::None,
    }
}

/// Creates a new instance of an `NSTDStr` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStr str` - The new `NSTDStr` instance.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     cstr::nstd_core_cstr_from_raw,
///     str::{nstd_core_str_byte_len, nstd_core_str_from_cstr_unchecked},
/// };
///
/// let s_str = "Goodbye, world!\0";
/// unsafe {
///     let cstr = nstd_core_cstr_from_raw(s_str.as_ptr().cast());
///     let str = nstd_core_str_from_cstr_unchecked(&cstr);
///     assert!(nstd_core_str_byte_len(&str) == 15);
/// }
/// ```
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_str_from_cstr_unchecked(cstr: &NSTDCStr) -> NSTDStr {
    let ptr = nstd_core_cstr_as_ptr(cstr).cast();
    let len = nstd_core_cstr_len(cstr);
    NSTDStr { ptr, len }
}

/// Creates a new `NSTDStr` from a raw C string.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new string slice on success or an uninitialized "none" variant if
/// `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_byte_len, nstd_core_str_from_raw_cstr};
///
/// let s_str = "Where I live is where I bleed.\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr(s_str.as_ptr().cast()).unwrap();
///     assert!(nstd_core_str_byte_len(&str) == 30);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_str_from_raw_cstr(cstr: *const NSTDChar) -> NSTDOptionalStr {
    if !cstr.is_null() {
        let len = nstd_core_cstr_raw_len(cstr);
        if len <= NSTD_INT_MAX {
            let ptr = cstr.cast();
            let bytes = core::slice::from_raw_parts(ptr, len);
            if core::str::from_utf8(bytes).is_ok() {
                return NSTDOptional::Some(NSTDStr { ptr, len });
            }
        }
    }
    NSTDOptional::None
}

/// Creates a new `NSTDStr` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new string slice on success or an uninitialized "none" variant if
/// `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_byte_len, nstd_core_str_from_raw_cstr_with_null};
///
/// let s_str = "{Hello, world!}}}%\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr_with_null(s_str.as_ptr().cast()).unwrap();
///     assert!(nstd_core_str_byte_len(&str) == 19);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_str_from_raw_cstr_with_null(cstr: *const NSTDChar) -> NSTDOptionalStr {
    if !cstr.is_null() {
        let len = nstd_core_cstr_raw_len_with_null(cstr);
        if len <= NSTD_INT_MAX {
            let ptr = cstr.cast();
            let bytes = core::slice::from_raw_parts(ptr, len);
            if core::str::from_utf8(bytes).is_ok() {
                return NSTDOptional::Some(NSTDStr { ptr, len });
            }
        }
    }
    NSTDOptional::None
}

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `const NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDOptionalStr str` - The new string slice on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     slice::nstd_core_slice_new,
///     str::{nstd_core_str_byte_len, nstd_core_str_from_bytes},
/// };
///
/// let s_str = "Hello, world!\0";
/// unsafe {
///     let bytes = nstd_core_slice_new(s_str.as_ptr().cast(), 1, s_str.len()).unwrap();
///     let str = nstd_core_str_from_bytes(&bytes).unwrap();
///     assert!(nstd_core_str_byte_len(&str) == 14);
/// }
/// ```
#[nstdapi]
pub const unsafe fn nstd_core_str_from_bytes(bytes: &NSTDSlice) -> NSTDOptionalStr {
    match core::str::from_utf8(bytes.as_slice()).is_ok() {
        true => {
            let ptr = nstd_core_slice_as_ptr(bytes).cast();
            let len = nstd_core_slice_len(bytes);
            NSTDOptional::Some(NSTDStr { ptr, len })
        }
        false => NSTDOptional::None,
    }
}

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `const NSTDSlice *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// - This function does not check to ensure that `bytes` are valid UTF-8.
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     slice::nstd_core_slice_new,
///     str::{nstd_core_str_byte_len, nstd_core_str_from_bytes_unchecked},
/// };
///
/// let s_str = "Goodbye, world!\0";
/// unsafe {
///     let bytes = nstd_core_slice_new(s_str.as_ptr().cast(), 1, s_str.len()).unwrap();
///     let str = nstd_core_str_from_bytes_unchecked(&bytes);
///     assert!(nstd_core_str_byte_len(&str) == 16);
/// }
/// ```
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_str_from_bytes_unchecked(bytes: &NSTDSlice) -> NSTDStr {
    assert!(nstd_core_slice_stride(bytes) == 1);
    let ptr = nstd_core_slice_as_ptr(bytes).cast();
    let len = nstd_core_slice_len(bytes);
    NSTDStr { ptr, len }
}

/// Returns a C string slice variant of this UTF-8 encoded string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
#[inline]
#[nstdapi]
pub const fn nstd_core_str_as_cstr(str: &NSTDStr) -> NSTDCStr {
    // SAFETY: `str.ptr` is never null, string slices are never longer than `NSTDInt`'s max value.
    unsafe { nstd_core_cstr_new_unchecked(str.ptr.cast(), str.len) }
}

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDSlice bytes` - An immutable byte slice over `str`'s data.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     slice::nstd_core_slice_len,
///     str::{nstd_core_str_as_bytes, nstd_core_str_byte_len, nstd_core_str_from_raw_cstr},
/// };
///
/// let s_str = "We won't be alone 🎶\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr(s_str.as_ptr().cast()).unwrap();
///     let bytes = nstd_core_str_as_bytes(&str);
///     assert!(nstd_core_str_byte_len(&str) == nstd_core_slice_len(&bytes));
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_str_as_bytes(str: &NSTDStr) -> NSTDSlice {
    // SAFETY: `str.ptr` is never null, string slice lengths are never greater than `NSTDInt`'s max
    // value.
    unsafe { nstd_core_slice_new_unchecked(str.ptr.cast(), 1, str.len) }
}

/// Returns a raw pointer to a string slice's memory.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string slice's memory.
#[inline]
#[nstdapi]
pub const fn nstd_core_str_as_ptr(str: &NSTDStr) -> *const NSTDByte {
    str.ptr
}

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the string slice.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_len, nstd_core_str_from_raw_cstr};
///
/// let s_str = "Hello, 🌎!\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr(s_str.as_ptr().cast()).unwrap();
///     assert!(nstd_core_str_len(&str) == 9);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_str_len(str: &NSTDStr) -> NSTDUInt {
    str.as_str().chars().count()
}

/// Returns the number of bytes a string slice contains.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt byte_len` - The number of bytes in the string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_byte_len, nstd_core_str_from_raw_cstr_with_null};
///
/// let s_str = "Hello, 🌎!\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr_with_null(s_str.as_ptr().cast()).unwrap();
///     assert!(nstd_core_str_byte_len(&str) == s_str.len());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_str_byte_len(str: &NSTDStr) -> NSTDUInt {
    str.len
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
/// - `NSTDUInt pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDOptionalUnichar chr` - The character at index `pos`, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_from_raw_cstr, nstd_core_str_get};
///
/// let s_str = "🦀🚀🦀!\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr(s_str.as_ptr().cast()).unwrap();
///     assert!(nstd_core_str_get(&str, 1).unwrap() == '🚀'.into());
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_str_get(str: &NSTDStr, pos: NSTDUInt) -> NSTDOptionalUnichar {
    str.as_str()
        .chars()
        .nth(pos)
        .map_or(NSTDOptional::None, |chr| NSTDOptional::Some(chr.into()))
}

/// Creates a substring of an existing string slice.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDOptionalStr substr` - The new substring on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.start` is greater than `range.end`.
///
/// - `range.end` is greater than `str.len`.
///
/// # Safety
///
/// `str`'s data must be valid for reads of at least `str.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     range::NSTDURange,
///     str::{nstd_core_str_byte_len, nstd_core_str_from_raw_cstr, nstd_core_str_substr},
/// };
///
/// let s_str = "33marrow\0";
/// unsafe {
///     let str = nstd_core_str_from_raw_cstr(s_str.as_ptr().cast()).unwrap();
///     let range = NSTDURange {
///         start: 2,
///         end: nstd_core_str_byte_len(&str),
///     };
///     let marrow = nstd_core_str_substr(&str, range).unwrap();
///     assert!(nstd_core_str_byte_len(&marrow) == 6);
/// }
/// ```
#[nstdapi]
#[allow(clippy::suspicious_operation_groupings)]
pub const unsafe fn nstd_core_str_substr(str: &NSTDStr, range: NSTDURange) -> NSTDOptionalStr {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.start <= range.end && range.end <= str.len);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = str.ptr.add(range.start).cast();
    let bytes = nstd_core_slice_new_unchecked(start, 1, range.end - range.start);
    nstd_core_str_from_bytes(&bytes)
}

gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_f32},
    /// };
    ///
    /// let str = "-420.69\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_f32(&str);
    ///     assert!(v == NSTDOptional::Some(-420.69));
    /// }
    /// ```
    nstd_core_str_to_f32,
    NSTDStr,
    NSTDFloat32,
    NSTDOptionalFloat32
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_f64},
    /// };
    ///
    /// let str = "-420.69\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_f64(&str);
    ///     assert!(v == NSTDOptional::Some(-420.69));
    /// }
    /// ```
    nstd_core_str_to_f64,
    NSTDStr,
    NSTDFloat64,
    NSTDOptionalFloat64
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_int},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_int(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_int,
    NSTDStr,
    NSTDInt,
    NSTDOptionalInt
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_uint},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_uint(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_uint,
    NSTDStr,
    NSTDUInt,
    NSTDOptionalUInt
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_i8},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_i8(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_i8,
    NSTDStr,
    NSTDInt8,
    NSTDOptionalInt8
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_u8},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_u8(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_u8,
    NSTDStr,
    NSTDUInt8,
    NSTDOptionalUInt8
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_i16},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_i16(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_i16,
    NSTDStr,
    NSTDInt16,
    NSTDOptionalInt16
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_u16},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_u16(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_u16,
    NSTDStr,
    NSTDUInt16,
    NSTDOptionalUInt16
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_i32},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_i32(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_i32,
    NSTDStr,
    NSTDInt32,
    NSTDOptionalInt32
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_u32},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_u32(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_u32,
    NSTDStr,
    NSTDUInt32,
    NSTDOptionalUInt32
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_i64},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_i64(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_i64,
    NSTDStr,
    NSTDInt64,
    NSTDOptionalInt64
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_from_raw_cstr, nstd_core_str_to_u64},
    /// };
    ///
    /// let str = "33\0";
    /// unsafe {
    ///     let str = nstd_core_str_from_raw_cstr(str.as_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_to_u64(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_to_u64,
    NSTDStr,
    NSTDUInt64,
    NSTDOptionalUInt64
);

/// An unowned view into a UTF-8 encoded byte string.
#[nstdapi]
pub struct NSTDStrMut {
    /// A raw pointer to the string's data.
    ptr: *mut NSTDByte,
    /// The number of bytes in the string.
    len: NSTDUInt,
}
impl NSTDStrMut {
    /// Creates a Rust string slice from this [`NSTDStrMut`].
    ///
    /// # Safety
    ///
    /// This string slice's data must remain valid UTF-8 and left unmodified while the returned
    /// string slice is in use.
    #[inline]
    const unsafe fn as_str(&self) -> &str {
        let bytes = core::slice::from_raw_parts(self.ptr, self.len);
        core::str::from_utf8_unchecked(bytes)
    }
}
gen_optional!(NSTDOptionalStrMut, NSTDStrMut);

/// Creates a new instance of an `NSTDStrMut` from a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new `NSTDStrMut` instance on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Safety
///
/// `cstr`'s data must be valid for reads of at least `cstr.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     cstr::nstd_core_cstr_mut_from_raw,
///     str::{nstd_core_str_mut_byte_len, nstd_core_str_mut_from_cstr},
/// };
///
/// let mut s_str = String::from("Hello, world!\0");
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     let str = nstd_core_str_mut_from_cstr(&mut cstr).unwrap();
///     assert!(nstd_core_str_mut_byte_len(&str) == 13);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_str_mut_from_cstr(cstr: &mut NSTDCStrMut) -> NSTDOptionalStrMut {
    match core::str::from_utf8(cstr.as_bytes()).is_ok() {
        true => {
            let ptr = nstd_core_cstr_mut_as_ptr(cstr).cast();
            let len = nstd_core_cstr_mut_len(cstr);
            NSTDOptional::Some(NSTDStrMut { ptr, len })
        }
        false => NSTDOptional::None,
    }
}

/// Creates a new instance of an `NSTDStrMut` from a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStrMut str` - The new `NSTDStrMut` instance.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     cstr::nstd_core_cstr_mut_from_raw,
///     str::{nstd_core_str_mut_byte_len, nstd_core_str_mut_from_cstr_unchecked},
/// };
///
/// let mut s_str = String::from("Goodbye, world!\0");
/// unsafe {
///     let mut cstr = nstd_core_cstr_mut_from_raw(s_str.as_mut_ptr().cast());
///     let str = nstd_core_str_mut_from_cstr_unchecked(&mut cstr);
///     assert!(nstd_core_str_mut_byte_len(&str) == 15);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_str_mut_from_cstr_unchecked(cstr: &mut NSTDCStrMut) -> NSTDStrMut {
    let ptr = nstd_core_cstr_mut_as_ptr(cstr).cast();
    let len = nstd_core_cstr_mut_len(cstr);
    NSTDStrMut { ptr, len }
}

/// Creates a new `NSTDStrMut` from a raw C string.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new string slice on success or an uninitialized "none" variant
/// if `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_mut_byte_len, nstd_core_str_mut_from_raw_cstr};
///
/// let mut s_str = String::from("Where I live is where I bleed.\0");
/// unsafe {
///     let str = nstd_core_str_mut_from_raw_cstr(s_str.as_mut_ptr().cast()).unwrap();
///     assert!(nstd_core_str_mut_byte_len(&str) == 30);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_str_mut_from_raw_cstr(cstr: *mut NSTDChar) -> NSTDOptionalStrMut {
    if !cstr.is_null() {
        let len = nstd_core_cstr_raw_len(cstr);
        if len <= NSTD_INT_MAX {
            let ptr = cstr.cast();
            let bytes = core::slice::from_raw_parts(ptr, len);
            if core::str::from_utf8(bytes).is_ok() {
                return NSTDOptional::Some(NSTDStrMut { ptr, len });
            }
        }
    }
    NSTDOptional::None
}

/// Creates a new `NSTDStrMut` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new string slice on success or an uninitialized "none" variant
/// if `cstr` is null, `cstr`'s length exceeds `NSTDInt`'s max value, or `cstr` is not valid UTF-8.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{
///     nstd_core_str_mut_byte_len, nstd_core_str_mut_from_raw_cstr_with_null,
/// };
///
/// let mut s_str = String::from("{Hello, world!}}}%\0");
/// unsafe {
///     let str = nstd_core_str_mut_from_raw_cstr_with_null(s_str.as_mut_ptr().cast()).unwrap();
///     assert!(nstd_core_str_mut_byte_len(&str) == 19);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_str_mut_from_raw_cstr_with_null(cstr: *mut NSTDChar) -> NSTDOptionalStrMut {
    if !cstr.is_null() {
        let len = nstd_core_cstr_raw_len_with_null(cstr);
        if len <= NSTD_INT_MAX {
            let ptr = cstr.cast();
            let bytes = core::slice::from_raw_parts(ptr, len);
            if core::str::from_utf8(bytes).is_ok() {
                return NSTDOptional::Some(NSTDStrMut { ptr, len });
            }
        }
    }
    NSTDOptional::None
}

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `NSTDSliceMut *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDOptionalStrMut str` - The new string slice on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     slice::nstd_core_slice_mut_new,
///     str::{nstd_core_str_mut_byte_len, nstd_core_str_mut_from_bytes},
/// };
///
/// let mut s_str = String::from("Hello, world!\0");
/// unsafe {
///     let mut bytes = nstd_core_slice_mut_new(s_str.as_mut_ptr().cast(), 1, s_str.len()).unwrap();
///     let str = nstd_core_str_mut_from_bytes(&mut bytes).unwrap();
///     assert!(nstd_core_str_mut_byte_len(&str) == 14);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_str_mut_from_bytes(bytes: &mut NSTDSliceMut) -> NSTDOptionalStrMut {
    match core::str::from_utf8(bytes.as_slice()).is_ok() {
        true => {
            let ptr = nstd_core_slice_mut_as_ptr(bytes).cast();
            let len = nstd_core_slice_mut_len(bytes);
            NSTDOptional::Some(NSTDStrMut { ptr, len })
        }
        false => NSTDOptional::None,
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
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// - This function does not check to ensure that `bytes` are valid UTF-8.
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     slice::nstd_core_slice_mut_new,
///     str::{nstd_core_str_mut_byte_len, nstd_core_str_mut_from_bytes_unchecked},
/// };
///
/// let mut s_str = String::from("Goodbye, world!\0");
/// unsafe {
///     let mut bytes = nstd_core_slice_mut_new(s_str.as_mut_ptr().cast(), 1, s_str.len()).unwrap();
///     let str = nstd_core_str_mut_from_bytes_unchecked(&mut bytes);
///     assert!(nstd_core_str_mut_byte_len(&str) == 16);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_str_mut_from_bytes_unchecked(bytes: &mut NSTDSliceMut) -> NSTDStrMut {
    assert!(nstd_core_slice_mut_stride(bytes) == 1);
    let ptr = nstd_core_slice_mut_as_ptr(bytes).cast();
    let len = nstd_core_slice_mut_len(bytes);
    NSTDStrMut { ptr, len }
}

/// Creates an immutable version of a mutable string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The mutable string slice.
///
/// # Returns
///
/// `NSTDStr str_const` - The immutable copy of `str`.
#[inline]
#[nstdapi]
pub const fn nstd_core_str_mut_as_const(str: &NSTDStrMut) -> NSTDStr {
    let bytes = nstd_core_str_mut_as_bytes(str);
    // SAFETY: String slices are UTF-8 encoded.
    unsafe { nstd_core_str_from_bytes_unchecked(&bytes) }
}

/// Returns a C string slice variant of this UTF-8 encoded string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice.
#[inline]
#[nstdapi]
pub const fn nstd_core_str_mut_as_cstr(str: &NSTDStrMut) -> NSTDCStr {
    // SAFETY: `str.ptr` is never null, string slices are never longer than `NSTDInt`'s max value.
    unsafe { nstd_core_cstr_new_unchecked(str.ptr.cast(), str.len) }
}

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDSlice bytes` - An immutable byte slice over `str`'s data.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     slice::nstd_core_slice_len,
///     str::{
///         nstd_core_str_mut_as_bytes, nstd_core_str_mut_byte_len, nstd_core_str_mut_from_raw_cstr,
///     },
/// };
///
/// let mut s_str = String::from("We won't be alone 🎶\0");
/// unsafe {
///     let mut str = nstd_core_str_mut_from_raw_cstr(s_str.as_mut_ptr().cast()).unwrap();
///     let bytes = nstd_core_str_mut_as_bytes(&str);
///     assert!(nstd_core_str_mut_byte_len(&str) == nstd_core_slice_len(&bytes));
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_str_mut_as_bytes(str: &NSTDStrMut) -> NSTDSlice {
    // SAFETY: `str.ptr` is never null, string slice lengths are never greater than `NSTDInt`'s max
    // value.
    unsafe { nstd_core_slice_new_unchecked(str.ptr.cast(), 1, str.len) }
}

/// Returns an immutable raw pointer to a string slice's memory.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string slice's memory.
#[inline]
#[nstdapi]
pub const fn nstd_core_str_mut_as_ptr(str: &NSTDStrMut) -> *const NSTDByte {
    str.ptr
}

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the string slice.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_mut_len, nstd_core_str_mut_from_raw_cstr};
///
/// let mut s_str = String::from("Hello, 🌎!\0");
/// unsafe {
///     let str = nstd_core_str_mut_from_raw_cstr(s_str.as_mut_ptr().cast()).unwrap();
///     assert!(nstd_core_str_mut_len(&str) == 9);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_str_mut_len(str: &NSTDStrMut) -> NSTDUInt {
    str.as_str().chars().count()
}

/// Returns the number of bytes a string slice contains.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDUInt byte_len` - The number of bytes in the string slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{
///     nstd_core_str_mut_byte_len, nstd_core_str_mut_from_raw_cstr_with_null,
/// };
///
/// let mut s_str = String::from("Hello, 🌎!\0");
/// unsafe {
///     let str = nstd_core_str_mut_from_raw_cstr_with_null(s_str.as_mut_ptr().cast()).unwrap();
///     assert!(nstd_core_str_mut_byte_len(&str) == s_str.len());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_str_mut_byte_len(str: &NSTDStrMut) -> NSTDUInt {
    str.len
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
/// - `NSTDUInt pos` - The index of the character to get.
///
/// # Returns
///
/// `NSTDOptionalUnichar chr` - The character at index `pos`, or none on error.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::core::str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_get};
///
/// let mut s_str = String::from("🦀🚀🦀!\0");
/// unsafe {
///     let str = nstd_core_str_mut_from_raw_cstr(s_str.as_mut_ptr().cast()).unwrap();
///     assert!(nstd_core_str_mut_get(&str, 1).unwrap() == '🚀'.into());
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_str_mut_get(str: &NSTDStrMut, pos: NSTDUInt) -> NSTDOptionalUnichar {
    str.as_str()
        .chars()
        .nth(pos)
        .map_or(NSTDOptional::None, |chr| NSTDOptional::Some(chr.into()))
}

/// Creates a substring of an existing string slice.
///
/// # Parameters:
///
/// - `NSTDStrMut *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDOptionalStrMut substr` - The new substring on success, or a "none" variant if the
/// result is not valid UTF-8.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.start` is greater than `range.end`.
///
/// - `range.end` is greater than `str.len`.
///
/// # Safety
///
/// `str`'s data must be valid for reads of at least `str.len` consecutive bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{
///     range::NSTDURange,
///     str::{
///         nstd_core_str_mut_byte_len, nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_substr,
///     },
/// };
///
/// let mut s_str = String::from("33marrow\0");
/// unsafe {
///     let mut str = nstd_core_str_mut_from_raw_cstr(s_str.as_mut_ptr().cast()).unwrap();
///     let range = NSTDURange {
///         start: 2,
///         end: nstd_core_str_mut_byte_len(&str),
///     };
///     let marrow = nstd_core_str_mut_substr(&mut str, range).unwrap();
///     assert!(nstd_core_str_mut_byte_len(&marrow) == 6);
/// }
/// ```
#[nstdapi]
#[allow(clippy::suspicious_operation_groupings)]
pub unsafe fn nstd_core_str_mut_substr(
    str: &mut NSTDStrMut,
    range: NSTDURange,
) -> NSTDOptionalStrMut {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.start <= range.end && range.end <= str.len);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = str.ptr.add(range.start).cast();
    let mut bytes = nstd_core_slice_mut_new_unchecked(start, 1, range.end - range.start);
    nstd_core_str_mut_from_bytes(&mut bytes)
}

gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_f32},
    /// };
    ///
    /// let mut str = String::from("-420.69\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_f32(&str);
    ///     assert!(v == NSTDOptional::Some(-420.69));
    /// }
    /// ```
    nstd_core_str_mut_to_f32,
    NSTDStrMut,
    NSTDFloat32,
    NSTDOptionalFloat32
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_f64},
    /// };
    ///
    /// let mut str = String::from("-420.69\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_f64(&str);
    ///     assert!(v == NSTDOptional::Some(-420.69));
    /// }
    /// ```
    nstd_core_str_mut_to_f64,
    NSTDStrMut,
    NSTDFloat64,
    NSTDOptionalFloat64
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_int},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_int(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_int,
    NSTDStrMut,
    NSTDInt,
    NSTDOptionalInt
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_uint},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_uint(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_uint,
    NSTDStrMut,
    NSTDUInt,
    NSTDOptionalUInt
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_i8},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_i8(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_i8,
    NSTDStrMut,
    NSTDInt8,
    NSTDOptionalInt8
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_u8},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_u8(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_u8,
    NSTDStrMut,
    NSTDUInt8,
    NSTDOptionalUInt8
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_i16},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_i16(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_i16,
    NSTDStrMut,
    NSTDInt16,
    NSTDOptionalInt16
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_u16},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_u16(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_u16,
    NSTDStrMut,
    NSTDUInt16,
    NSTDOptionalUInt16
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_i32},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_i32(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_i32,
    NSTDStrMut,
    NSTDInt32,
    NSTDOptionalInt32
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_u32},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_u32(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_u32,
    NSTDStrMut,
    NSTDUInt32,
    NSTDOptionalUInt32
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_i64},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_i64(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_i64,
    NSTDStrMut,
    NSTDInt64,
    NSTDOptionalInt64
);
gen_to_primitive!(
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::{
    ///     optional::NSTDOptional,
    ///     str::{nstd_core_str_mut_from_raw_cstr, nstd_core_str_mut_to_u64},
    /// };
    ///
    /// let mut str = String::from("33\0");
    /// unsafe {
    ///     let str = nstd_core_str_mut_from_raw_cstr(str.as_mut_ptr().cast()).unwrap();
    ///     let v = nstd_core_str_mut_to_u64(&str);
    ///     assert!(v == NSTDOptional::Some(33));
    /// }
    /// ```
    nstd_core_str_mut_to_u64,
    NSTDStrMut,
    NSTDUInt64,
    NSTDOptionalUInt64
);
