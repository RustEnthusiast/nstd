//! A dynamically sized, null terminated, C string.
use crate::vec::{nstd_vec_free, nstd_vec_new, NSTDVec};

/// A dynamically sized, null terminated, C string.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDCString {
    /// The underlying vector of `NSTDChar`s.
    pub bytes: NSTDVec,
}

/// Creates a new empty `NSTDCString`.
///
/// # Returns
///
/// `NSTDCString cstring` - The new C string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_new() -> NSTDCString {
    NSTDCString {
        bytes: nstd_vec_new(1),
    }
}

/// Frees an instance of `NSTDCString`.
///
/// # Parameters:
///
/// - `NSTDCString *cstring` - The C string to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_cstring_free(cstring: &mut NSTDCString) {
    nstd_vec_free(&mut cstring.bytes);
}
