//! Dynamically sized UTF-8 encoded byte string.
use crate::vec::{nstd_vec_free, nstd_vec_new, NSTDVec};

/// Dynamically sized UTF-8 encoded byte string.
#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct NSTDString {
    /// The underlying UTF-8 encoded byte buffer.
    pub bytes: NSTDVec,
}

/// Creates a new instance of `NSTDString`.
///
/// # Returns
///
/// `NSTDString string` - The new string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_new() -> NSTDString {
    NSTDString {
        bytes: nstd_vec_new(1),
    }
}

/// Frees an instance of `NSTDString`.
///
/// # Parameters:
///
/// - `NSTDString *string` - A pointer to the string to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_string_free(string: &mut NSTDString) {
    nstd_vec_free(&mut string.bytes);
}
