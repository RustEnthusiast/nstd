//! Provides functionality for interacting with the standard I/O streams.
use crate::core::{
    cstr::{nstd_core_cstr_const_as_ptr, nstd_core_cstr_const_len, NSTDCStrConst},
    def::NSTDErrorCode,
};
use std::io::Write;

/// Writes a C string slice to stdout.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice to write to stdout.
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
/// The provided C string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print(cstr: &NSTDCStrConst) -> NSTDErrorCode {
    let ptr = nstd_core_cstr_const_as_ptr(cstr).cast();
    let len = nstd_core_cstr_const_len(cstr);
    let bytes = std::slice::from_raw_parts(ptr, len);
    let mut stdout = std::io::stdout();
    if let Err(_) = stdout.write_all(bytes) {
        return 1;
    } else if let Err(_) = stdout.flush() {
        return 2;
    }
    0
}
