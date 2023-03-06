#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "nstd_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_alloc")))]
pub mod alloc;
#[cfg(feature = "nstd_app")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_app")))]
pub mod app;
#[cfg(feature = "nstd_core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_core")))]
pub mod core;
#[cfg(feature = "nstd_cstring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_cstring")))]
pub mod cstring;
#[cfg(feature = "nstd_env")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_env")))]
pub mod env;
#[cfg(feature = "nstd_fs")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_fs")))]
pub mod fs;
#[cfg(feature = "nstd_gl")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_gl")))]
pub mod gl;
#[cfg(feature = "nstd_heap_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_heap_ptr")))]
pub mod heap_ptr;
#[cfg(feature = "nstd_image")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_image")))]
pub mod image;
#[cfg(feature = "nstd_io")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_io")))]
pub mod io;
#[cfg(feature = "nstd_math")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_math")))]
pub mod math;
#[cfg(feature = "nstd_mutex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_mutex")))]
pub mod mutex;
#[cfg(feature = "nstd_os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os")))]
pub mod os;
#[cfg(feature = "nstd_proc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_proc")))]
pub mod proc;
#[cfg(feature = "nstd_shared_lib")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_shared_lib")))]
pub mod shared_lib;
#[cfg(feature = "nstd_shared_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_shared_ptr")))]
pub mod shared_ptr;
#[cfg(feature = "nstd_string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_string")))]
pub mod string;
#[cfg(test)]
pub(crate) mod test;
#[cfg(feature = "nstd_thread")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_thread")))]
pub mod thread;
#[cfg(feature = "nstd_time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_time")))]
pub mod time;
#[cfg(feature = "nstd_timed_mutex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_timed_mutex")))]
pub mod timed_mutex;
#[cfg(feature = "nstd_vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_vec")))]
pub mod vec;
#[cfg(feature = "nstd_window")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_window")))]
pub mod window;
use ::core::ffi::{c_char, c_void};

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
///
/// # Safety
///
/// Accessing any data through this pointer type is unsafe. Raw pointers have no way of knowing if
/// the data being pointed to is or isn't valid.
pub type NSTDAny = *const c_void;
/// An opaque pointer to some mutable data.
///
/// # Safety
///
/// Accessing or mutating any data through this pointer type is unsafe. Raw pointers have no way of
/// knowing if the data being pointed to is or isn't valid.
pub type NSTDAnyMut = *mut c_void;

/// A boolean type, can either be `NSTD_TRUE` (1) or `NSTD_FALSE` (0).
pub type NSTDBool = bool;
