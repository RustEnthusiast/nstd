//! Common types used throughout `nstd`.
use crate::{NSTDInt32, NSTDUInt8};
use core::ffi::c_void;
use cty::c_char;

/// A void pointer (a pointer to some arbitrary type).
pub type NSTDAny = *mut c_void;
/// A void pointer to some immutable data.
pub type NSTDAnyConst = *const c_void;

/// Equivalent to C's `char` type.
pub type NSTDChar = c_char;

/// The smallest addressable unit of memory.
pub type NSTDByte = NSTDUInt8;

/// An error code type to be returned from functions. An error code of 0 means success, while
/// anything else indicates failure.
pub type NSTDErrorCode = NSTDInt32;
