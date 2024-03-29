#ifndef NSTD_PROC_H
#define NSTD_PROC_H
#include "core/optional.h"
#include "core/slice.h"
#include "core/str.h"
#include "io/io.h"
#include "nstd.h"

/// A handle to a child process.
typedef struct {
    /// A handle to a child process.
    NSTDAnyMut proc;
} NSTDChildProcess;

/// Represents an optional value of type `NSTDChildProcess`.
NSTDOptional(NSTDChildProcess) NSTDOptionalChildProcess;

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
/// `NSTDOptionalChildProcess child` - A handle to the new child process on success, or an
/// uninitialized "none" variant if spawning the child process fails.
///
/// # Safety
///
/// The user must ensure that all of `program`, `args`, and `vars` and their data remain valid for
/// reads while this function is executing.
NSTDAPI NSTDOptionalChildProcess
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
NSTDAPI void nstd_proc_abort(void);

/// Returns the ID of the current process.
///
/// # Returns
///
/// `NSTDUInt32 ID` - The process ID.
NSTDAPI NSTDUInt32 nstd_proc_id(void);

#endif
