//! An unowned view into a UTF-8 encoded byte string.
use crate::{
    core::{
        cstr::{
            nstd_core_cstr_const_as_bytes, nstd_core_cstr_mut_as_ptr, nstd_core_cstr_mut_len,
            NSTDCStrConst, NSTDCStrMut,
        },
        def::{NSTDByte, NSTDErrorCode},
        range::NSTDURange,
        slice::{
            nstd_core_slice_const_as_ptr, nstd_core_slice_const_new, nstd_core_slice_const_stride,
            nstd_core_slice_mut_as_ptr_const, nstd_core_slice_mut_new, nstd_core_slice_mut_stride,
            NSTDSliceConst, NSTDSliceMut,
        },
    },
    NSTDFloat32, NSTDFloat64, NSTDISize, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt16,
    NSTDUInt32, NSTDUInt64, NSTDUInt8, NSTDUSize, NSTDUnichar,
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
pub struct NSTDStrConst {
    /// A view into the UTF-8 encoded buffer.
    bytes: NSTDSliceConst,
}
impl NSTDStrConst {
    /// Creates a Rust string slice from this [NSTDStrConst].
    ///
    /// # Safety
    ///
    /// This string slice's data must remain valid while the returned string slice is in use.
    #[inline]
    pub(crate) unsafe fn as_str(&self) -> &str {
        core::str::from_utf8_unchecked(self.bytes.as_slice())
    }
}

/// Creates a new instance of `NSTDStrConst` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStrConst str` - The new `NSTDStrConst` instance.
///
/// # Panics
///
/// This function will panic if `cstr`'s data is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_from_cstr(cstr: &NSTDCStrConst) -> NSTDStrConst {
    let bytes = nstd_core_cstr_const_as_bytes(cstr);
    // SAFETY: The returned string slice is already unsafe to access.
    core::str::from_utf8(unsafe { bytes.as_slice() }).expect("Invalid UTF-8 bytes");
    NSTDStrConst { bytes }
}

/// Creates a new instance of `NSTDStrConst` from a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice to wrap.
///
/// # Returns
///
/// `NSTDStrConst str` - The new `NSTDStrConst` instance.
///
/// # Safety
///
/// This function does not check to ensure that `cstr` is valid UTF-8. `cstr`'s data must remain
/// valid while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_const_from_cstr_unchecked(
    cstr: &NSTDCStrConst,
) -> NSTDStrConst {
    NSTDStrConst {
        bytes: nstd_core_cstr_const_as_bytes(cstr),
    }
}

/// Creates a string slice from raw bytes.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1, or `bytes` is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_from_bytes(bytes: &NSTDSliceConst) -> NSTDStrConst {
    // SAFETY: The returned string slice is already unsafe to access.
    core::str::from_utf8(unsafe { bytes.as_slice() }).expect("Invalid UTF-8 bytes");
    let stride = nstd_core_slice_const_stride(bytes);
    NSTDStrConst {
        bytes: nstd_core_slice_const_new(bytes.ptr.raw, stride, bytes.len),
    }
}

/// Creates a string slice from raw bytes, without checking for UTF-8.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// `NSTDStrConst str` - The new string slice.
///
/// # Panics
///
/// This operation will panic if `bytes`'s stride is not 1.
///
/// # Safety
///
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_const_from_bytes_unchecked(
    bytes: &NSTDSliceConst,
) -> NSTDStrConst {
    let stride = nstd_core_slice_const_stride(bytes);
    assert!(stride == 1);
    NSTDStrConst {
        bytes: nstd_core_slice_const_new(bytes.ptr.raw, stride, bytes.len),
    }
}

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice over `str`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_as_bytes(str: &NSTDStrConst) -> NSTDSliceConst {
    nstd_core_slice_const_new(str.bytes.ptr.raw.cast(), 1, str.bytes.len)
}

/// Returns a raw pointer to a string slice's memory.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice.
///
/// # Returns
///
/// `const NSTDByte *ptr` - A raw pointer to a string slice's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_as_ptr(str: &NSTDStrConst) -> *const NSTDByte {
    nstd_core_slice_const_as_ptr(&str.bytes).cast()
}

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the string slice.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_const_len(str: &NSTDStrConst) -> NSTDUSize {
    str.as_str().chars().count()
}

/// Gets the `NSTDUnichar` at index `pos` in `str`.
///
/// # Note
///
/// `pos` does not refer to the byte index of the character, but the `NSTDUnichar` index instead.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice to index.
///
/// - `NSTDUSize pos` - The index of the character to get.
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
pub unsafe extern "C" fn nstd_core_str_const_get_char(
    str: &NSTDStrConst,
    pos: NSTDUSize,
) -> NSTDUnichar {
    match str.as_str().chars().nth(pos) {
        Some(chr) => chr as NSTDUnichar,
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
}

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
///
/// # Parameters:
///
/// - `const NSTDStrConst *str` - The string slice to create the new substring from.
///
/// - `NSTDURange range` - The bounds of the new substring (indexed by bytes).
///
/// # Returns
///
/// `NSTDStrConst substr` - The new substring.
///
/// # Panics
///
/// This operation can panic under the following circumstances:
///
/// - `range.end` is greater than `str.bytes.len`.
///
/// - `range.start` is greater than `range.end`.
///
/// - The substring bytes are not valid UTF-8.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_const_substr(
    str: &NSTDStrConst,
    range: NSTDURange,
) -> NSTDStrConst {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.end <= str.bytes.len);
    assert!(range.start <= range.end);
    // Create the byte slice with `range` and use it to create the new string slice.
    // SAFETY: The returned string slice is already unsafe to access.
    let start = unsafe { str.bytes.ptr.raw.add(range.start) };
    let bytes = nstd_core_slice_const_new(start, 1, range.end - range.start);
    nstd_core_str_const_from_bytes(&bytes)
}

gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDFloat32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDFloat32 v` - The parsed 32-bit floating-point value.
    nstd_core_str_const_to_f32,
    NSTDStrConst,
    NSTDFloat32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDFloat64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDFloat64 v` - The parsed 64-bit floating-point value.
    nstd_core_str_const_to_f64,
    NSTDStrConst,
    NSTDFloat64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt8`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt8 v` - The parsed 8-bit unsigned integral value.
    nstd_core_str_const_to_u8,
    NSTDStrConst,
    NSTDUInt8
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt8`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt8 v` - The parsed 8-bit signed integral value.
    nstd_core_str_const_to_i8,
    NSTDStrConst,
    NSTDInt8
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt16`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt16 v` - The parsed 16-bit unsigned integral value.
    nstd_core_str_const_to_u16,
    NSTDStrConst,
    NSTDUInt16
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt16`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt16 v` - The parsed 16-bit signed integral value.
    nstd_core_str_const_to_i16,
    NSTDStrConst,
    NSTDInt16
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt32 v` - The parsed 32-bit unsigned integral value.
    nstd_core_str_const_to_u32,
    NSTDStrConst,
    NSTDUInt32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt32`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt32 v` - The parsed 32-bit signed integral value.
    nstd_core_str_const_to_i32,
    NSTDStrConst,
    NSTDInt32
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUInt64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUInt64 v` - The parsed 64-bit unsigned integral value.
    nstd_core_str_const_to_u64,
    NSTDStrConst,
    NSTDUInt64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDInt64`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDInt64 v` - The parsed 64-bit signed integral value.
    nstd_core_str_const_to_i64,
    NSTDStrConst,
    NSTDInt64
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDUSize`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUSize v` - The parsed arch-bit unsigned integral value.
    nstd_core_str_const_to_usize,
    NSTDStrConst,
    NSTDUSize
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDISize`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrConst *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDISize v` - The parsed arch-bit signed integral value.
    nstd_core_str_const_to_isize,
    NSTDStrConst,
    NSTDISize
);

/// An unowned view into a UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDStrMut {
    /// A view into the UTF-8 encoded buffer.
    bytes: NSTDSliceMut,
}
impl NSTDStrMut {
    /// Creates a Rust string slice from this [NSTDStrMut].
    ///
    /// # Safety
    ///
    /// This string slice's data must remain valid while the returned string slice is in use.
    #[inline]
    unsafe fn as_str(&self) -> &str {
        core::str::from_utf8_unchecked(self.bytes.as_slice())
    }
}

/// Creates a new instance of `NSTDStrMut` from a C string slice.
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
/// This function will panic if `cstr`'s data is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_from_cstr(cstr: &mut NSTDCStrMut) -> NSTDStrMut {
    let ptr = nstd_core_cstr_mut_as_ptr(cstr);
    let len = nstd_core_cstr_mut_len(cstr);
    let bytes = nstd_core_slice_mut_new(ptr.cast(), 1, len);
    // SAFETY: The returned string slice is already unsafe to access.
    core::str::from_utf8(unsafe { bytes.as_slice() }).expect("Invalid UTF-8 bytes");
    NSTDStrMut { bytes }
}

/// Creates a new instance of `NSTDStrMut` from a C string slice.
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
    NSTDStrMut {
        bytes: nstd_core_slice_mut_new(ptr, 1, len),
    }
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
/// This operation will panic if `bytes`'s stride is not 1, or `bytes` is not valid UTF-8.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_from_bytes(bytes: &mut NSTDSliceMut) -> NSTDStrMut {
    // SAFETY: The returned string slice is already unsafe to access.
    core::str::from_utf8(unsafe { bytes.as_slice() }).expect("Invalid UTF-8 bytes");
    let stride = nstd_core_slice_mut_stride(bytes);
    NSTDStrMut {
        bytes: nstd_core_slice_mut_new(bytes.ptr.raw, stride, bytes.len),
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
/// This function does not check to ensure that `bytes` are valid UTF-8.`bytes` must remain valid
/// while the returned string slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_from_bytes_unchecked(
    bytes: &mut NSTDSliceMut,
) -> NSTDStrMut {
    let stride = nstd_core_slice_mut_stride(bytes);
    assert!(stride == 1);
    NSTDStrMut {
        bytes: nstd_core_slice_mut_new(bytes.ptr.raw, stride, bytes.len),
    }
}

/// Returns an immutable byte slice over `str`'s data.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice over `str`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_as_bytes(str: &NSTDStrMut) -> NSTDSliceConst {
    nstd_core_slice_const_new(str.bytes.ptr.raw.cast(), 1, str.bytes.len)
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
    nstd_core_slice_mut_as_ptr_const(&str.bytes).cast()
}

/// Returns the number of Unicode characters in a string slice.
///
/// # Parameters:
///
/// - `const NSTDStrMut *str` - The string slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the string slice.
///
/// # Safety
///
/// This operation can cause undefined behavior in the event that `str`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_mut_len(str: &NSTDStrMut) -> NSTDUSize {
    str.as_str().chars().count()
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
/// - `NSTDUSize pos` - The index of the character to get.
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
    pos: NSTDUSize,
) -> NSTDUnichar {
    match str.as_str().chars().nth(pos) {
        Some(chr) => chr as NSTDUnichar,
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
}

/// Creates a substring of an existing string slice.
///
/// # Note
///
/// This function is considered safe because the returned string slice is already unsafe to operate
/// on.
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
/// - `range.end` is greater than `str.bytes.len`.
///
/// - `range.start` is greater than `range.end`.
///
/// - The substring bytes are not valid UTF-8.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_str_mut_substr(str: &mut NSTDStrMut, range: NSTDURange) -> NSTDStrMut {
    // Make sure the range is valid for the bounds of `str`.
    assert!(range.end <= str.bytes.len);
    assert!(range.start <= range.end);
    // Create the byte slice with `range` and use it to create the new string slice.
    // SAFETY: The returned string slice is already unsafe to access.
    let start = unsafe { str.bytes.ptr.raw.add(range.start) };
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
    /// Attempts to parse a string slice as an `NSTDUSize`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDUSize v` - The parsed arch-bit unsigned integral value.
    nstd_core_str_mut_to_usize,
    NSTDStrMut,
    NSTDUSize
);
gen_to_primitive!(
    /// Attempts to parse a string slice as an `NSTDISize`.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDStrMut *str` - The string slice.
    ///
    /// - `NSTDErrorCode *errc` - Set to nonzero on error.
    ///
    /// # Returns
    ///
    /// `NSTDISize v` - The parsed arch-bit signed integral value.
    nstd_core_str_mut_to_isize,
    NSTDStrMut,
    NSTDISize
);
