#ifndef NSTD_PROC_H
#define NSTD_PROC_H
#include "core/slice.h"
#include "core/str.h"
#include "io/io.h"
#include "nstd.h"

/// A handle to a child process.
typedef NSTDAnyMut NSTDChildProcess;

/// Spawns a new child process with the name `program` and returns a handle to it.
///
/// # Parameters:
///
/// - `const NSTDStr *program` - A path to the program to run as a child process.
///
/// - `const NSTDSlice *args` - A slice of `NSTDStr` arguments to pass to the program.
///
/// - `const NSTDSlice *vars` - A slice of `NSTDStr[2]` key/value environment variables to
/// give to the program.
///
/// # Returns
///
/// `NSTDChildProcess child` - A handle to the new child process, null on error.
///
/// # Panics
///
/// This operation will panic in any of the following situations:
///
/// - `program`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// - `args`'s stride is not equal to `sizeof(NSTDStr)`.
///
/// - `args`'s length is greater than `NSTDInt`'s max value.
///
/// - `vars`'s stride is not equal to `sizeof(NSTDStr[2])`.
///
/// - `vars`'s length is greater than `NSTDInt`'s max value.
///
/// - Any of the arguments or environment variable keys/values length in bytes exceeds `NSTDInt`'s
/// max value.
///
/// # Safety
///
/// The user must ensure that all of `program`, `args`, and `vars` and their data remain valid for
/// reads while this function is executing.
NSTDAPI NSTDChildProcess
nstd_proc_spawn(const NSTDStr *program, const NSTDSlice *args, const NSTDSlice *vars);

/// Returns the OS-assigned ID of a child process.
///
/// # Parameters:
///
/// - `const NSTDChildProcess *handle` - A handle to the process.
///
/// # Returns
///
/// `NSTDUInt32 ID` - The child process ID.
NSTDAPI NSTDUInt32 nstd_proc_child_id(const NSTDChildProcess *handle);

/// Attempts to kill a child process.
///
/// # Parameters:
///
/// - `NSTDChildProcess *handle` - A handle to the child process.
///
/// # Returns
///
/// `NSTDIOError errc` - The operation error code.
NSTDAPI NSTDIOError nstd_proc_kill(NSTDChildProcess *handle);

/// Waits for a child process to exit.
///
/// # Parameters:
///
/// - `NSTDChildProcess *handle` - A handle to the process.
///
/// # Returns
///
/// `NSTDIOError errc` - The operation error code.
NSTDAPI NSTDIOError nstd_proc_join(NSTDChildProcess *handle);

/// Frees a handle to a child process, allowing the process to run in the background.
///
/// # Parameters:
///
/// - `NSTDChildProcess handle` - A handle to the child process.
NSTDAPI void nstd_proc_free(NSTDChildProcess handle);

/// Terminates the process with the given `exit_code`.
///
/// # Parameters:
///
/// - `NSTDInt32 exit_code` - The process exit code.
NSTDAPI NSTDInt32 nstd_proc_exit(NSTDInt32 exit_code);

/// Terminates the program in an abnormal fashion.
NSTDAPI void nstd_proc_abort();

/// Returns the ID of the current process.
///
/// # Returns
///
/// `NSTDUInt32 ID` - The process ID.
NSTDAPI NSTDUInt32 nstd_proc_id();

#endif
