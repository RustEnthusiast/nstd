//! Process environment management.
use crate::{
    core::{result::NSTDResult, str::NSTDStr},
    io::{NSTDIOError, NSTDIOStringResult},
    string::NSTDString,
    vec::NSTDVec,
};
use nstdapi::nstdapi;
use std::env::VarError;

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
#[nstdapi]
pub fn nstd_env_current_dir() -> NSTDIOStringResult {
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
#[nstdapi]
pub fn nstd_env_current_exe() -> NSTDIOStringResult {
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
#[nstdapi]
pub fn nstd_env_temp_dir() -> NSTDString {
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
#[nstdapi]
pub unsafe fn nstd_env_set_current_dir(path: &NSTDStr) -> NSTDIOError {
    match std::env::set_current_dir(path.as_str()) {
        Err(err) => NSTDIOError::from_err(err.kind()),
        _ => NSTDIOError::NSTD_IO_ERROR_NONE,
    }
}

/// Retrieves a variable from the process environment.
///
/// # Parameters:
///
/// - `const NSTDStr *key` - The variable's key.
///
/// # Returns
///
/// `NSTDIOStringResult var` - The value of the environment variable, or the I/O operation error
/// code on failure. This will return as `NSTD_IO_ERROR_NOT_FOUND` if they variable cannot be found,
/// and `NSTD_IO_ERROR_INVALID_DATA` if the variable isn't valid Unicode.
///
/// # Panics
///
/// This operation will panic if `key`'s length in bytes exceeds `NSTDInt`'s max value, or
/// allocating the string fails.
///
/// # Safety
///
/// The user of this function must ensure that `key` is valid for reads.
#[nstdapi]
pub unsafe fn nstd_env_var(key: &NSTDStr) -> NSTDIOStringResult {
    match std::env::var(key.as_str()) {
        Ok(var) => NSTDResult::Ok(NSTDString::from_str(&var)),
        Err(VarError::NotPresent) => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_NOT_FOUND),
        Err(VarError::NotUnicode(_)) => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_DATA),
    }
}

/// Sets an environment variable for the current process.
///
/// # Parameters:
///
/// - `const NSTDStr *key` - The environment variable's identification key.
///
/// - `const NSTDStr *value` - The environment variable's value.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `key` is empty or contains either of the following ASCII characters: `'='` or `'\0'`.
///
/// - `value` contains the ASCII null character `'\0'`.
///
/// - Either `key` or `value`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// The user of this function must ensure that both `key` and `value` are valid for reads.
#[inline]
#[nstdapi]
pub unsafe fn nstd_env_set_var(key: &NSTDStr, value: &NSTDStr) {
    std::env::set_var(key.as_str(), value.as_str());
}

/// Removes an environment variable from the current process.
///
/// # Parameters:
///
/// - `const NSTDStr *key` - The environment variable's identification key.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `key` is empty or contains either of the following ASCII characters: `'='` or `'\0'`.
///
/// - `key`'s size in bytes exceeds `NSTDInt`'s max value.
///
/// - The environment variable's value contains the ASCII null character `'\0'`.
///
/// # Safety
///
/// The user of this function must ensure that `key` is valid for reads.
#[inline]
#[nstdapi]
pub unsafe fn nstd_env_remove_var(key: &NSTDStr) {
    std::env::remove_var(key.as_str());
}

/// Returns an `NSTDVec` of `NSTDString`s that each represent an argument received at program start.
///
/// # Returns
///
/// `NSTDVec args` - The `NSTDString` arguments that the program was started with.
///
/// # Panics
///
/// This operation may panic in the following situations:
///
/// - Any arguments are invalid Unicode.
///
/// - Allocating for any of the arguments fails.
///
/// - The total number of bytes required for the vector exceeds `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_env_args() -> NSTDVec {
    let args = std::env::args();
    NSTDVec::from_iter(args.map(|arg| NSTDString::from_str(&arg)))
}

/// Returns an `NSTDVec` of `NSTDString[2]` which each represent an environment variable from the
/// current process.
///
/// # Returns
///
/// `NSTDVec vars` - A list of the process environment variables.
///
/// # Panics
///
/// This operation may panic in the following situations:
///
/// - Any of the environment variable's keys or values are invalid Unicode.
///
/// - Allocating for any of the keys/values fails.
///
/// - The total number of bytes required for the vector exceeds `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_env_vars() -> NSTDVec {
    let vars = std::env::vars();
    NSTDVec::from_iter(vars.map(|(k, v)| [NSTDString::from_str(&k), NSTDString::from_str(&v)]))
}
