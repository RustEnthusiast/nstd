//! Process environment management.
use crate::{
    core::{result::NSTDResult, str::NSTDStr},
    io::{NSTDIOError, NSTDIOStringResult},
    string::NSTDString,
};

/// Returns a complete path to the process's current working directory.
///
/// Any non-Unicode sequences are replaced with the Unicode replacement character.
///
/// # Returns
///
/// `NSTDIOStringResult working_dir` - A path to the current working directory on success, or the
/// I/O operation error code on failure.
///
/// # Panics
///
/// This operation will panic if allocating the string fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_env_current_dir() -> NSTDIOStringResult {
    match std::env::current_dir() {
        Ok(dir) => NSTDResult::Ok(NSTDString::from_str(&dir.to_string_lossy())),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Returns a complete path to the process executable.
///
/// Any non-Unicode sequences are replaced with the Unicode replacement character.
///
/// # Note
///
/// Please see Rust's documentation for information about the security of this function
/// <https://doc.rust-lang.org/std/env/fn.current_exe.html>.
///
/// # Returns
///
/// `NSTDIOStringResult exe` - A complete path to process executable on success, or the I/O
/// operation error code on failure.
///
/// # Panics
///
/// This operation will panic if allocating the string fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_env_current_exe() -> NSTDIOStringResult {
    match std::env::current_exe() {
        Ok(exe) => NSTDResult::Ok(NSTDString::from_str(&exe.to_string_lossy())),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Returns a complete path to a temporary directory.
///
/// Any non-Unicode sequences are replaced with the Unicode replacement character.
///
/// # Returns
///
/// `NSTDString temp` - A path to the temporary directory.
///
/// # Panics
///
/// This operation will panic if allocating the string fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_env_temp_dir() -> NSTDString {
    NSTDString::from_str(&std::env::temp_dir().to_string_lossy())
}

/// Sets the current working directory for the process.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - The directory to set as the process working directory.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This operation will panic if `path`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// The user of this function must ensure that `path` is valid for reads.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_set_current_dir(path: &NSTDStr) -> NSTDIOError {
    match std::env::set_current_dir(path.as_str()) {
        Err(err) => NSTDIOError::from_err(err.kind()),
        _ => NSTDIOError::NSTD_IO_ERROR_NONE,
    }
}
