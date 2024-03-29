//! Calling/Child process management.
use crate::{
    alloc::CBox,
    core::{
        optional::{gen_optional, NSTDOptional},
        slice::NSTDSlice,
        str::NSTDStr,
    },
    io::NSTDIOError,
    NSTDInt32, NSTDUInt32,
};
use nstdapi::nstdapi;
use std::process::{Child, Command};

/// A handle to a child process.
#[nstdapi]
pub struct NSTDChildProcess {
    /// A handle to a child process.
    proc: CBox<Child>,
}
gen_optional!(NSTDOptionalChildProcess, NSTDChildProcess);

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
#[nstdapi]
pub unsafe fn nstd_proc_spawn(
    program: &NSTDStr,
    args: &NSTDSlice,
    vars: &NSTDSlice,
) -> NSTDOptionalChildProcess {
    // Create the process command builder.
    let mut cmd = Command::new(program.as_str());
    if let Some(args) = args.as_slice::<NSTDStr>() {
        if let Some(vars) = vars.as_slice::<[NSTDStr; 2]>() {
            // Add the arguments.
            cmd.args(args.iter().map(|arg| arg.as_str()));
            // Add the environment variables.
            cmd.envs(vars.iter().map(|vars| {
                (
                    vars.get_unchecked(0).as_str(),
                    vars.get_unchecked(1).as_str(),
                )
            }));
            // Spawn the process.
            if let Ok(proc) = cmd.spawn() {
                if let Some(proc) = CBox::new(proc) {
                    return NSTDOptional::Some(NSTDChildProcess { proc });
                }
            }
        }
    }
    NSTDOptional::None
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
    handle.proc.id()
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
    if let Err(err) = handle.proc.kill() {
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
    match handle.proc.wait() {
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
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
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
/// unsafe { nstd_proc_exit(0) };
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
