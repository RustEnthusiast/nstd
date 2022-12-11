#ifndef NSTD_ENV_H
#define NSTD_ENV_H
#include "core/str.h"
#include "io/io.h"
#include "nstd.h"
#include "string.h"

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
NSTDAPI NSTDIOStringResult nstd_env_current_dir();

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
NSTDAPI NSTDIOStringResult nstd_env_current_exe();

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
NSTDAPI NSTDString nstd_env_temp_dir();

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
NSTDAPI NSTDIOError nstd_env_set_current_dir(const NSTDStr *path);

#endif
