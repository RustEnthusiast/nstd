//! Process environment management.
use crate::{
    alloc::{NSTDAllocError::NSTD_ALLOC_ERROR_NONE, NSTD_ALLOCATOR},
    core::{optional::NSTDOptional, result::NSTDResult, str::NSTDStr},
    io::{NSTDIOError, NSTDIOStringResult},
    string::{NSTDOptionalString, NSTDString},
    vec::{nstd_vec_new, nstd_vec_push, NSTDVec},
};
use nstdapi::nstdapi;
use std::{env::VarError, ptr::addr_of};

/// Returns a complete path to the process's current working directory.
///
/// # Returns
///
/// `NSTDIOStringResult working_dir` - A path to the current working directory on success, or the
/// I/O operation error code on failure.
#[nstdapi]
pub fn nstd_env_current_dir() -> NSTDIOStringResult<'static> {
    match std::env::current_dir() {
        Ok(dir) => match dir.to_str() {
            Some(dir) => NSTDResult::Ok(NSTDString::from_string(dir.into())),
            _ => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_DATA),
        },
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Returns a complete path to the process executable.
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
#[nstdapi]
pub fn nstd_env_current_exe() -> NSTDIOStringResult<'static> {
    match std::env::current_exe() {
        Ok(exe) => match exe.to_str() {
            Some(exe) => NSTDResult::Ok(NSTDString::from_string(exe.into())),
            _ => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_DATA),
        },
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Returns a complete path to a temporary directory.
///
/// # Returns
///
/// `NSTDOptionalString temp` - A path to the temporary directory.
#[nstdapi]
pub fn nstd_env_temp_dir() -> NSTDOptionalString<'static> {
    match std::env::temp_dir().to_str() {
        Some(temp) => NSTDOptional::Some(NSTDString::from_string(temp.into())),
        _ => NSTDOptional::None,
    }
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
/// # Safety
///
/// The user of this function must ensure that `key` is valid for reads.
#[nstdapi]
pub unsafe fn nstd_env_var(key: &NSTDStr) -> NSTDIOStringResult<'_> {
    match std::env::var(key.as_str()) {
        Ok(var) => NSTDResult::Ok(NSTDString::from_string(var)),
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
/// This operation will panic if any program arguments contain invalid Unicode.
#[nstdapi]
pub fn nstd_env_args() -> NSTDVec<'static> {
    let mut args = nstd_vec_new(&NSTD_ALLOCATOR, std::mem::size_of::<NSTDString<'_>>());
    for arg in std::env::args() {
        let arg = NSTDString::from_string(arg);
        // SAFETY: `arg` is stored on the stack.
        let errc = unsafe { nstd_vec_push(&mut args, addr_of!(arg) as _) };
        if errc == NSTD_ALLOC_ERROR_NONE {
            std::mem::forget(arg);
        }
    }
    args
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
/// This operation will panic if any environment variables contain invalid Unicode.
#[nstdapi]
pub fn nstd_env_vars() -> NSTDVec<'static> {
    let mut vars = nstd_vec_new(&NSTD_ALLOCATOR, std::mem::size_of::<[NSTDString<'_>; 2]>());
    for (k, v) in std::env::vars() {
        let var = [NSTDString::from_string(k), NSTDString::from_string(v)];
        // SAFETY: `var` is stored on the stack.
        let errc = unsafe { nstd_vec_push(&mut vars, addr_of!(var) as _) };
        if errc == NSTD_ALLOC_ERROR_NONE {
            std::mem::forget(var);
        }
    }
    vars
}
