//! An unowned view into a UTF-8 encoded byte string.
use crate::{
    core::{
        cstr::{
            nstd_core_cstr_as_ptr, nstd_core_cstr_len, nstd_core_cstr_mut_as_ptr,
            nstd_core_cstr_mut_len,
            raw::{nstd_core_cstr_raw_len, nstd_core_cstr_raw_len_with_null},
            NSTDCStr, NSTDCStrMut,
        },
        def::{NSTDByte, NSTDErrorCode},
        range::NSTDURange,
        slice::{
            nstd_core_slice_as_ptr, nstd_core_slice_len, nstd_core_slice_mut_as_ptr,
            nstd_core_slice_mut_len, nstd_core_slice_mut_new, nstd_core_slice_mut_stride,
            nstd_core_slice_new, nstd_core_slice_stride, NSTDSlice, NSTDSliceMut,
        },
    },
    NSTDChar, NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8,
    NSTDUInt, NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8, NSTDUnichar,
};

/// Generates the `nstd_core_str_*_to_[i|u|f]*` functions.
macro_rules! gen_to_primitive {
    (
        $(#[$meta:meta])*
        $name: ident, $StrT: ty, $RetT: ty
    ) => {
        $(#[$meta])*
        ///
        /// # Safety:
        ///
        /// This operation can cause undefined behavior if `str`'s data is invalid.
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(str: &$StrT, errc: &mut NSTDErrorCode) -> $RetT {
            if let Ok(v) = str.as_str().parse() {
                return v;
            }
            *errc = 1;
            <$RetT>::default()
        }
    };
}

/// An immutable unowned view into a UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDStr {
    /// A raw pointer to the string's data.
    ptr: *const NSTDByte,
    /// The number of bytes in the string.
    len: NSTDUInt,
}
impl NSTDStr {
    /// Creates a Rust string slice from this [NSTDStr].
    ///
    /// # Panics
    ///
    /// This method will panic if the string slice's length is greater than `isize::MAX`.
    ///
    /// # Safety
    ///
    /// This string slice's data must remain valid UTF-8 and left unmodified while the returned
    /// string slice is in use.
    #[inline]
    pub(crate) unsafe fn as_str(&self) -> &str {
        assert!(self.len <= isize::MAX as usize);
        let bytes = core::slice::from_raw_parts(self.ptr, self.len);
        core::str::from_utf8_unchecked(bytes)
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
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `cstr`'s data is not valid UTF-8.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - `cstr`'s data must be valid for reads of at least `cstr.len` consecutive bytes.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_cstr(cstr: &NSTDCStr) -> NSTDStr {
    let ptr = nstd_core_cstr_as_ptr(cstr).cast();
    let len = nstd_core_cstr_len(cstr);
    assert!(len <= isize::MAX as usize);
    let bytes = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    NSTDStr { ptr, len }
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_cstr_unchecked(cstr: &NSTDCStr) -> NSTDStr {
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
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `cstr`'s data is not valid UTF-8.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - This function makes access to raw pointer data, which can cause undefined behavior in the
/// event that `cstr`'s data is invalid.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_raw_cstr(cstr: *const NSTDChar) -> NSTDStr {
    let ptr = cstr.cast();
    let len = nstd_core_cstr_raw_len(cstr);
    assert!(len <= isize::MAX as NSTDUInt);
    let bytes = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    NSTDStr { ptr, len }
}

/// Creates a new `NSTDStr` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDStr str` - The new string slice.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `cstr`'s data is not valid UTF-8.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - This function makes access to raw pointer data, which can cause undefined behavior in the
/// event that `cstr`'s data is invalid.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_raw_cstr_with_null(cstr: *const NSTDChar) -> NSTDStr {
    let ptr = cstr.cast();
    let len = nstd_core_cstr_raw_len_with_null(cstr);
    assert!(len <= isize::MAX as NSTDUInt);
    let bytes = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    NSTDStr { ptr, len }
}

/// Creates a string slice from raw bytes.
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
/// This operation will panic in the following situations:
///
/// - `bytes`'s stride is not 1.
///
/// - `bytes`'s length is greater than `NSTDInt`'s max value.
///
/// - `bytes` is not valid UTF-8.
///
/// # Safety
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_bytes(bytes: &NSTDSlice) -> NSTDStr {
    core::str::from_utf8(bytes.as_slice()).expect("Invalid UTF-8 bytes");
    let ptr = nstd_core_slice_as_ptr(bytes).cast();
    let len = nstd_core_slice_len(bytes);
    NSTDStr { ptr, len }
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
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_bytes_unchecked(bytes: &NSTDSlice) -> NSTDStr {
    assert!(nstd_core_slice_stride(bytes) == 1);
    let ptr = nstd_core_slice_as_ptr(bytes).cast();
    let len = nstd_core_slice_len(bytes);
    NSTDStr { ptr, len }
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_as_bytes(str: &NSTDStr) -> NSTDSlice {
    nstd_core_slice_new(str.ptr.cast(), 1, str.len)
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_as_ptr(str: &NSTDStr) -> *const NSTDByte {
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
/// # Panics
///
/// This operation may panic in the event that `str`'s calculated length is greater than the
/// highest number representable by `NSTDUInt`.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_len(str: &NSTDStr) -> NSTDUInt {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_byte_len(str: &NSTDStr) -> NSTDUInt {
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
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
///
/// # Safety
///
/// This operation could cause undefined behavior if `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_get_char(str: &NSTDStr, pos: NSTDUInt) -> NSTDUnichar {
    match str.as_str().chars().nth(pos) {
        Some(chr) => chr as NSTDUnichar,
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
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
/// `NSTDStr substr` - The new substring.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.start` is greater than `NSTDInt`'s max value.
///
/// - `range.start` is greater than `range.end`.
///
/// - `range.end` is greater than `str.len`.
///
/// - `range.end` - `range.start` is greater than `NSTDInt`'s max value.
///
/// - The substring bytes are not valid UTF-8.
///
/// # Safety
///
/// - `str`'s data must be valid for reads of at least `str.len` consecutive bytes.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_substr(str: &NSTDStr, range: NSTDURange) -> NSTDStr {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.start <= isize::MAX as usize && range.start <= range.end && range.end <= str.len);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = str.ptr.add(range.start).cast();
    let bytes = nstd_core_slice_new(start, 1, range.end - range.start);
    nstd_core_str_from_bytes(&bytes)
}

gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDFloat32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDFloat32 v` - The parsed 32-bit floating-point value.
    nstd_core_str_to_f32,
    NSTDStr,
    NSTDFloat32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDFloat64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDFloat64 v` - The parsed 64-bit floating-point value.
    nstd_core_str_to_f64,
    NSTDStr,
    NSTDFloat64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt v` - The parsed arch-bit signed integral value.
    nstd_core_str_to_int,
    NSTDStr,
    NSTDInt
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt v` - The parsed arch-bit unsigned integral value.
    nstd_core_str_to_uint,
    NSTDStr,
    NSTDUInt
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt8`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt8 v` - The parsed 8-bit signed integral value.
    nstd_core_str_to_i8,
    NSTDStr,
    NSTDInt8
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt8`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt8 v` - The parsed 8-bit unsigned integral value.
    nstd_core_str_to_u8,
    NSTDStr,
    NSTDUInt8
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt16`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt16 v` - The parsed 16-bit signed integral value.
    nstd_core_str_to_i16,
    NSTDStr,
    NSTDInt16
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt16`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt16 v` - The parsed 16-bit unsigned integral value.
    nstd_core_str_to_u16,
    NSTDStr,
    NSTDUInt16
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt32 v` - The parsed 32-bit signed integral value.
    nstd_core_str_to_i32,
    NSTDStr,
    NSTDInt32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt32 v` - The parsed 32-bit unsigned integral value.
    nstd_core_str_to_u32,
    NSTDStr,
    NSTDUInt32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt64 v` - The parsed 64-bit signed integral value.
    nstd_core_str_to_i64,
    NSTDStr,
    NSTDInt64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStr *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt64 v` - The parsed 64-bit unsigned integral value.
    nstd_core_str_to_u64,
    NSTDStr,
    NSTDUInt64
);

/// An unowned view into a UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDStrMut {
    /// A raw pointer to the string's data.
    ptr: *mut NSTDByte,
    /// The number of bytes in the string.
    len: NSTDUInt,
}
impl NSTDStrMut {
    /// Creates a Rust string slice from this [NSTDStrMut].
    ///
    /// # Panics
    ///
    /// This method will panic if the string slice's length is greater than `isize::MAX`.
    ///
    /// # Safety
    ///
    /// This string slice's data must remain valid UTF-8 and left unmodified while the returned
    /// string slice is in use.
    #[inline]
    unsafe fn as_str(&self) -> &str {
        assert!(self.len <= isize::MAX as usize);
        let bytes = core::slice::from_raw_parts(self.ptr, self.len);
        core::str::from_utf8_unchecked(bytes)
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
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `cstr`'s data is not valid UTF-8.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - `cstr`'s data must be valid for reads of at least `cstr.len` consecutive bytes.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_cstr(cstr: &mut NSTDCStrMut) -> NSTDStrMut {
    let ptr = nstd_core_cstr_mut_as_ptr(cstr).cast();
    let len = nstd_core_cstr_mut_len(cstr);
    assert!(len <= isize::MAX as usize);
    let bytes = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    NSTDStrMut { ptr, len }
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_cstr_unchecked(
    cstr: &mut NSTDCStrMut,
) -> NSTDStrMut {
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
/// `NSTDStrMut str` - The new string slice.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `cstr`'s data is not valid UTF-8.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - This function makes access to raw pointer data, which can cause undefined behavior in the
/// event that `cstr`'s data is invalid.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_raw_cstr(cstr: *mut NSTDChar) -> NSTDStrMut {
    let ptr = cstr.cast();
    let len = nstd_core_cstr_raw_len(cstr);
    assert!(len <= isize::MAX as usize);
    let bytes = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    NSTDStrMut { ptr, len }
}

/// Creates a new `NSTDStrMut` from a raw C string, including the null byte.
///
/// # Parameters:
///
/// - `NSTDChar *cstr` - The raw C string to wrap.
///
/// # Returns
///
/// `NSTDStrMut str` - The new string slice.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `cstr`'s data is not valid UTF-8.
///
/// - `cstr`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - This function makes access to raw pointer data, which can cause undefined behavior in the
/// event that `cstr`'s data is invalid.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_raw_cstr_with_null(
    cstr: *mut NSTDChar,
) -> NSTDStrMut {
    let ptr = cstr.cast();
    let len = nstd_core_cstr_raw_len_with_null(cstr);
    assert!(len <= isize::MAX as usize);
    let bytes = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    NSTDStrMut { ptr, len }
}

/// Creates a string slice from raw bytes.
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
/// This operation will panic in the following situations:
///
/// - `bytes`'s stride is not 1.
///
/// - `bytes`'s length is greater than `NSTDInt`'s max value.
///
/// - `bytes` is not valid UTF-8.
///
/// # Safety
///
/// - `bytes` must remain valid while the returned string slice is in use.
///
/// - `bytes`'s data must be valid for reads of at least `bytes.len` consecutive bytes.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_bytes(bytes: &mut NSTDSliceMut) -> NSTDStrMut {
    core::str::from_utf8(bytes.as_slice()).expect("Invalid UTF-8 bytes");
    let ptr = nstd_core_slice_mut_as_ptr(bytes).cast();
    let len = nstd_core_slice_mut_len(bytes);
    NSTDStrMut { ptr, len }
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
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_bytes_unchecked(
    bytes: &mut NSTDSliceMut,
) -> NSTDStrMut {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_as_const(str: &NSTDStrMut) -> NSTDStr {
    let bytes = nstd_core_str_mut_as_bytes(str);
    // SAFETY: String slices are UTF-8 encoded.
    unsafe { nstd_core_str_from_bytes_unchecked(&bytes) }
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_as_bytes(str: &NSTDStrMut) -> NSTDSlice {
    nstd_core_slice_new(str.ptr.cast(), 1, str.len)
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_as_ptr(str: &NSTDStrMut) -> *const NSTDByte {
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
/// # Panics
///
/// This operation may panic in the event that `str`'s calculated length is greater than the
/// highest number representable by `NSTDUInt`.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_len(str: &NSTDStrMut) -> NSTDUInt {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_byte_len(str: &NSTDStrMut) -> NSTDUInt {
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
/// `NSTDUnichar chr` - The character at index `pos`, or the Unicode replacement character on
/// error.
///
/// # Safety
///
/// This operation could cause undefined behavior if `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_get_char(
    str: &NSTDStrMut,
    pos: NSTDUInt,
) -> NSTDUnichar {
    match str.as_str().chars().nth(pos) {
        Some(chr) => chr as NSTDUnichar,
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
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
/// `NSTDStrMut substr` - The new substring.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.start` is greater than `NSTDInt`'s max value.
///
/// - `range.start` is greater than `range.end`.
///
/// - `range.end` is greater than `str.len`.
///
/// - `range.end` - `range.start` is greater than `NSTDInt`'s max value.
///
/// - The substring bytes are not valid UTF-8.
///
/// # Safety
///
/// - `str`'s data must be valid for reads of at least `str.len` consecutive bytes.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_substr(
    str: &mut NSTDStrMut,
    range: NSTDURange,
) -> NSTDStrMut {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.start <= isize::MAX as usize && range.start <= range.end && range.end <= str.len);
    // Create the byte slice with `range` and use it to create the new string slice.
    let start = str.ptr.add(range.start).cast();
    let mut bytes = nstd_core_slice_mut_new(start, 1, range.end - range.start);
    nstd_core_str_mut_from_bytes(&mut bytes)
}

gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDFloat32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDFloat32 v` - The parsed 32-bit floating-point value.
    nstd_core_str_mut_to_f32,
    NSTDStrMut,
    NSTDFloat32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDFloat64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDFloat64 v` - The parsed 64-bit floating-point value.
    nstd_core_str_mut_to_f64,
    NSTDStrMut,
    NSTDFloat64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt v` - The parsed arch-bit signed integral value.
    nstd_core_str_mut_to_int,
    NSTDStrMut,
    NSTDInt
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt v` - The parsed arch-bit unsigned integral value.
    nstd_core_str_mut_to_uint,
    NSTDStrMut,
    NSTDUInt
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt8`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt8 v` - The parsed 8-bit signed integral value.
    nstd_core_str_mut_to_i8,
    NSTDStrMut,
    NSTDInt8
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt8`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt8 v` - The parsed 8-bit unsigned integral value.
    nstd_core_str_mut_to_u8,
    NSTDStrMut,
    NSTDUInt8
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt16`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt16 v` - The parsed 16-bit signed integral value.
    nstd_core_str_mut_to_i16,
    NSTDStrMut,
    NSTDInt16
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt16`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt16 v` - The parsed 16-bit unsigned integral value.
    nstd_core_str_mut_to_u16,
    NSTDStrMut,
    NSTDUInt16
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt32 v` - The parsed 32-bit signed integral value.
    nstd_core_str_mut_to_i32,
    NSTDStrMut,
    NSTDInt32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt32 v` - The parsed 32-bit unsigned integral value.
    nstd_core_str_mut_to_u32,
    NSTDStrMut,
    NSTDUInt32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt64 v` - The parsed 64-bit signed integral value.
    nstd_core_str_mut_to_i64,
    NSTDStrMut,
    NSTDInt64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt64 v` - The parsed 64-bit unsigned integral value.
    nstd_core_str_mut_to_u64,
    NSTDStrMut,
    NSTDUInt64
);
