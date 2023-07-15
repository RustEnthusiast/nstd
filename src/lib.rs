#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::missing_panics_doc,
    clippy::undocumented_unsafe_blocks
)]
#![cfg_attr(feature = "link", allow(dead_code, unused_imports))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
pub mod alloc;
#[cfg(feature = "core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "core")))]
pub mod core;
#[cfg(feature = "cstring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "cstring")))]
pub mod cstring;
#[cfg(feature = "env")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "env")))]
pub mod env;
#[cfg(feature = "fs")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fs")))]
pub mod fs;
#[cfg(feature = "heap_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "heap_ptr")))]
pub mod heap_ptr;
#[cfg(feature = "image")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "io")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "io")))]
pub mod io;
#[cfg(feature = "math")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "math")))]
pub mod math;
#[cfg(feature = "mutex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "mutex")))]
pub mod mutex;
#[cfg(feature = "os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "os")))]
pub mod os;
#[cfg(feature = "proc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proc")))]
pub mod proc;
#[cfg(feature = "shared_lib")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "shared_lib")))]
pub mod shared_lib;
#[cfg(feature = "shared_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "shared_ptr")))]
pub mod shared_ptr;
#[cfg(feature = "string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "string")))]
pub mod string;
#[cfg(test)]
pub(crate) mod test;
#[cfg(feature = "thread")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "thread")))]
pub mod thread;
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
pub mod time;
#[cfg(feature = "timed_mutex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "timed_mutex")))]
pub mod timed_mutex;
#[cfg(feature = "vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "vec")))]
pub mod vec;
use ::core::ffi::{c_char, c_void};

/// [NSTDInt]'s maximum value.
#[allow(dead_code)]
pub(crate) const NSTD_INT_MAX: NSTDInt = NSTDInt::MAX;

/// A null pointer value constant.
pub const NSTD_NULL: NSTDAnyMut = ::core::ptr::null_mut();

/// Boolean value false (0).
pub const NSTD_FALSE: NSTDBool = false;
/// Boolean value true (1).
pub const NSTD_TRUE: NSTDBool = true;

/// An integral type who's size matches the target architecture's pointer size.
pub type NSTDInt = isize;
/// An unsigned integral type who's size matches the target architecture's pointer size.
pub type NSTDUInt = usize;

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

/// An opaque pointer to some immutable data.
pub type NSTDAny = *const c_void;
/// An opaque pointer to some mutable data.
pub type NSTDAnyMut = *mut c_void;

/// A boolean type, can either be `NSTD_TRUE` (1) or `NSTD_FALSE` (0).
pub type NSTDBool = bool;
