#ifndef NSTD_ENV_H
#define NSTD_ENV_H
#include "core/str.h"
#include "io/io.h"
#include "nstd.h"
#include "string.h"
#include "vec.h"

/// Returns a complete path to the process's current working directory.
///
/// # Returns
///
/// `NSTDIOStringResult working_dir` - A path to the current working directory on success, or the
/// I/O operation error code on failure.
NSTDAPI NSTDIOStringResult nstd_env_current_dir(void);

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
NSTDAPI NSTDIOStringResult nstd_env_current_exe(void);

/// Returns a complete path to a temporary directory.
///
/// # Returns
///
/// `NSTDOptionalString temp` - A path to the temporary directory.
NSTDAPI NSTDOptionalString nstd_env_temp_dir(void);

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
NSTDAPI NSTDIOError nstd_env_set_current_dir(const NSTDStr *path);

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
NSTDAPI NSTDIOStringResult nstd_env_var(const NSTDStr *key);

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
NSTDAPI void nstd_env_set_var(const NSTDStr *key, const NSTDStr *value);

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
NSTDAPI void nstd_env_remove_var(const NSTDStr *key);

/// Returns an `NSTDVec` of `NSTDString`s that each represent an argument received at program start.
///
/// # Returns
///
/// `NSTDVec args` - The `NSTDString` arguments that the program was started with.
///
/// # Panics
///
/// This operation will panic if any program arguments contain invalid Unicode.
NSTDAPI NSTDVec nstd_env_args(void);

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
NSTDAPI NSTDVec nstd_env_vars(void);

#endif
