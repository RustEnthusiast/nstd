#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "nstd_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_alloc")))]
pub mod alloc;
#[cfg(feature = "nstd_core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_core")))]
pub mod core;
#[cfg(feature = "nstd_heap_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_heap_ptr")))]
pub mod heap_ptr;
#[cfg(feature = "nstd_os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os")))]
pub mod os;
#[cfg(feature = "nstd_string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_string")))]
pub mod string;
#[cfg(test)]
pub(crate) mod test;
#[cfg(feature = "nstd_vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_vec")))]
pub mod vec;

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

/// An 8-bit character type.
pub type NSTDChar8 = NSTDUInt8;
/// A 16-bit character type.
pub type NSTDChar16 = NSTDUInt16;
/// A 32-bit character type.
pub type NSTDChar32 = NSTDUInt32;
/// Represents a Unicode scalar value.
pub type NSTDUnichar = NSTDChar32;

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
impl From<bool> for NSTDBool {
    /// Creates an `NSTDBool` from a Rust [bool].
    #[inline]
    fn from(b: bool) -> Self {
        match b {
            true => Self::NSTD_BOOL_TRUE,
            false => Self::NSTD_BOOL_FALSE,
        }
    }
}
