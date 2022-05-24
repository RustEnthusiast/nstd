//! Dynamically sized UTF-8 encoded byte string.
use crate::vec::NSTDVec;

/// Dynamically sized UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct NSTDString {
    /// The underlying UTF-8 encoded byte buffer.
    pub bytes: NSTDVec,
}
