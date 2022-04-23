//! Common types used throughout `nstd`.
use core::ffi::c_void;
use cty::c_char;

/// A void pointer (a pointer to some arbitrary type).
pub type NSTDAny = *mut c_void;
/// A void pointer to some immutable data.
pub type NSTDAnyConst = *const c_void;

/// An integral type who's size matches the target architecture's pointer size.
pub type NSTDISize = isize;
/// An unsigned integral type who's size matches the target architecture's pointer size.
pub type NSTDUSize = usize;

/// An 8-bit signed integer type.
pub type NSTDInt8 = i8;
/// An 8-bit unsigned integer type.
pub type NSTDUInt8 = u8;
/// A 16-bit signed integer type.
pub type NSTDInt16 = i16;
/// A 16-bit unsigned integer type.
pub type NSTDUInt16 = u16;
/// A 32-bit signed integer type.
pub type NSTDInt32 = i32;
/// A 32-bit unsigned integer type.
pub type NSTDUInt32 = u32;
/// A 64-bit signed integer type.
pub type NSTDInt64 = i64;
/// A 64-bit unsigned integer type.
pub type NSTDUInt64 = u64;

/// A 32-bit floating point type.
pub type NSTDFloat32 = f32;
/// A 64-bit floating point type.
pub type NSTDFloat64 = f64;

/// Equivalent to C's `char` type.
pub type NSTDChar = c_char;
/// An 8-bit character type.
pub type NSTDChar8 = NSTDUInt8;
/// A 16-bit character type.
pub type NSTDChar16 = NSTDUInt16;
/// A 32-bit character type.
pub type NSTDChar32 = NSTDUInt32;
/// Represents a Unicode scalar value.
pub type NSTDUnichar = NSTDChar32;

/// The smallest addressable unit of memory.
pub type NSTDByte = NSTDUInt8;

/// An error code type to be returned from functions. An error code of 0 means success, while
/// anything else indicates failure.
pub type NSTDErrorCode = NSTDInt32;

/// A boolean type, can either be `NSTD_BOOL_TRUE` (1) or `NSTD_BOOL_FALSE` (0).
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDBool {
    /// Boolean value false (0).
    NSTD_BOOL_FALSE,
    /// Boolean value true (1).
    NSTD_BOOL_TRUE,
}
impl Default for NSTDBool {
    /// Returns `NSTD_BOOL_FALSE`.
    #[inline]
    fn default() -> Self {
        Self::NSTD_BOOL_FALSE
    }
}
