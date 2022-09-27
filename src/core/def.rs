//! Common types used throughout `nstd`.
use crate::{NSTDInt32, NSTDUInt8};

/// The smallest addressable unit of memory.
pub type NSTDByte = NSTDUInt8;

/// An error code type to be returned from functions. An error code of 0 means success, while
/// anything else indicates failure.
pub type NSTDErrorCode = NSTDInt32;
