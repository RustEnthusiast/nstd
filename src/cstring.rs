//! A dynamically sized, null terminated, C string.
use crate::vec::NSTDVec;

/// A dynamically sized, null terminated, C string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCString {
    /// The underlying vector of `NSTDChar`s.
    pub bytes: NSTDVec,
}
