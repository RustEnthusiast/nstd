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
#[cfg(feature = "nstd_cstring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_cstring")))]
pub mod cstring;
#[cfg(feature = "nstd_heap_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_heap_ptr")))]
pub mod heap_ptr;
#[cfg(feature = "nstd_os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os")))]
pub mod os;
#[cfg(feature = "nstd_shared_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_shared_ptr")))]
pub mod shared_ptr;
#[cfg(feature = "nstd_string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_string")))]
pub mod string;
#[cfg(test)]
pub(crate) mod test;
#[cfg(feature = "nstd_vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_vec")))]
pub mod vec;
use ::core::ffi::c_void;

/// A null pointer value constant.
pub const NSTD_NULL: NSTDAnyMut = ::core::ptr::null_mut();

/// Boolean value false (0).
pub const NSTD_FALSE: NSTDBool = 0;
/// Boolean value true (1).
pub const NSTD_TRUE: NSTDBool = 1;

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

/// A void pointer (a pointer to some arbitrary type).
pub type NSTDAnyMut = *mut c_void;
/// A void pointer to some immutable data.
pub type NSTDAnyConst = *const c_void;

/// A boolean type, can either be `NSTD_TRUE` (1) or `NSTD_FALSE` (0).
pub type NSTDBool = NSTDUInt8;
