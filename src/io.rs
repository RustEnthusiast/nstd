//! Provides functionality for interacting with the standard I/O streams.
use crate::core::{
    cstr::raw::nstd_core_cstr_raw_len,
    def::{NSTDChar, NSTDErrorCode},
};
use std::io::Write;

/// Writes a raw null-terminated C string to stdout.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw null-terminated C string.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - Writing `cstr`'s bytes to stdout failed.
///
/// - `2` - Flushing stdout failed.
///
/// # Safety
///
/// The provided C string must be null terminated, else this function can cause garbage bytes to be
/// written to stdout.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print(cstr: *const NSTDChar) -> NSTDErrorCode {
    let len = nstd_core_cstr_raw_len(cstr);
    let bytes = std::slice::from_raw_parts(cstr.cast(), len);
    let mut stdout = std::io::stdout();
    if let Err(_) = stdout.write_all(bytes) {
        return 1;
    } else if let Err(_) = stdout.flush() {
        return 2;
    }
    0
}
