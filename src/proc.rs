//! Calling/Child process management.
use crate::{
    core::{slice::NSTDSlice, str::NSTDStr},
    io::NSTDIOError,
    NSTDInt32, NSTDUInt32,
};
use nstdapi::nstdapi;
use std::process::{Child, Command};

/// A handle to a child process.
pub type NSTDChildProcess = Box<Child>;

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
#[nstdapi]
pub unsafe fn nstd_proc_spawn(
    program: &NSTDStr,
    args: &NSTDSlice,
    vars: &NSTDSlice,
) -> Option<NSTDChildProcess> {
    // Create the process command builder.
    let mut cmd = Command::new(program.as_str());
    // Add the arguments.
    cmd.args(args.as_slice::<NSTDStr>().iter().map(|arg| arg.as_str()));
    // Add the environment variables.
    cmd.envs(vars.as_slice::<[NSTDStr; 2]>().iter().map(|vars| {
        (
            vars.get_unchecked(0).as_str(),
            vars.get_unchecked(1).as_str(),
        )
    }));
    // Spawn the process.
    cmd.spawn().ok().map(Box::new)
}

/// Returns the OS-assigned ID of a child process.
///
/// # Parameters:
///
/// - `const NSTDChildProcess *handle` - A handle to the process.
///
/// # Returns
///
/// `NSTDUInt32 ID` - The child process ID.
#[inline]
#[nstdapi]
pub fn nstd_proc_child_id(handle: &NSTDChildProcess) -> NSTDUInt32 {
    handle.id()
}

/// Attempts to kill a child process.
///
/// # Parameters:
///
/// - `NSTDChildProcess *handle` - A handle to the child process.
///
/// # Returns
///
/// `NSTDIOError errc` - The operation error code.
#[inline]
#[nstdapi]
pub fn nstd_proc_kill(handle: &mut NSTDChildProcess) -> NSTDIOError {
    if let Err(err) = handle.kill() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Waits for a child process to exit.
///
/// # Parameters:
///
/// - `NSTDChildProcess *handle` - A handle to the process.
///
/// # Returns
///
/// `NSTDIOError errc` - The operation error code.
#[nstdapi]
pub fn nstd_proc_join(handle: &mut NSTDChildProcess) -> NSTDIOError {
    match handle.wait() {
        Ok(status) if status.success() => NSTDIOError::NSTD_IO_ERROR_NONE,
        Err(err) => NSTDIOError::from_err(err.kind()),
        _ => NSTDIOError::NSTD_IO_ERROR_UNKNOWN,
    }
}

/// Frees a handle to a child process, allowing the process to run in the background.
///
/// # Parameters:
///
/// - `NSTDChildProcess handle` - A handle to the child process.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_proc_free(handle: NSTDChildProcess) {}

/// Terminates the process with the given `exit_code`.
///
/// # Parameters:
///
/// - `NSTDInt32 exit_code` - The process exit code.
///
/// # Example
///
/// ```
/// use nstd_sys::proc::nstd_proc_exit;
///
/// nstd_proc_exit(0);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_proc_exit(exit_code: NSTDInt32) -> ! {
    std::process::exit(exit_code);
}

/// Terminates the program in an abnormal fashion.
#[inline]
#[nstdapi]
pub fn nstd_proc_abort() -> ! {
    std::process::abort();
}

/// Returns the ID of the current process.
///
/// # Returns
///
/// `NSTDUInt32 ID` - The process ID.
#[inline]
#[nstdapi]
pub fn nstd_proc_id() -> NSTDUInt32 {
    std::process::id()
}
