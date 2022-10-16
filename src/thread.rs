//! Thread spawning, joining, and detaching.
use crate::{
    core::{def::NSTDErrorCode, str::NSTDStr},
    io::NSTDIOError,
    NSTDFloat64, NSTDUInt,
};
use std::{
    thread::{Builder, JoinHandle},
    time::Duration,
};

/// A handle to a running thread.
pub type NSTDThread = Box<JoinHandle<NSTDErrorCode>>;

/// Describes the creation of a new thread.
///
/// This type is passed to the `nstd_thread_spawn_with_desc` function.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NSTDThreadDescriptor {
    /// The name of the thread.
    pub name: NSTDStr,
    /// The number of bytes that the thread's stack should have.
    ///
    /// Set this to 0 to let the host decide how much stack memory should be allocated.
    pub stack_size: NSTDUInt,
}

/// Spawns a new thread and returns a handle to it.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)()` - The thread function.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread, null on error.
///
/// # Safety
///
/// The caller of this function must guarantee that `thread_fn` is a valid function pointer.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn(
    thread_fn: Option<unsafe extern "C" fn() -> NSTDErrorCode>,
) -> Option<NSTDThread> {
    if let Some(thread_fn) = thread_fn {
        if let Ok(thread) = Builder::new().spawn(move || thread_fn()) {
            return Some(Box::new(thread));
        }
    }
    None
}

/// Spawns a new thread configured with a descriptor.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)()` - The thread function.
///
/// - `const NSTDThreadDescriptor *desc` - The thread descriptor.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread, null on error.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `desc.name` contains null bytes.
///
/// - `desc.name`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// - The caller of this function must guarantee that `thread_fn` is a valid function pointer.
///
/// - This operation can cause undefined behavior if `desc`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn_with_desc(
    thread_fn: Option<unsafe extern "C" fn() -> NSTDErrorCode>,
    desc: &NSTDThreadDescriptor,
) -> Option<NSTDThread> {
    if let Some(thread_fn) = thread_fn {
        // Create the thread builder.
        let mut builder = Builder::new();
        builder = builder.name(desc.name.as_str().to_string());
        if desc.stack_size != 0 {
            builder = builder.stack_size(desc.stack_size);
        }
        // Spawn the new thread.
        if let Ok(thread) = builder.spawn(move || thread_fn()) {
            return Some(Box::new(thread));
        }
    }
    None
}

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
///
/// # Returns
///
/// `NSTDErrorCode errc` - The thread function's return code.
///
/// # Panics
///
/// Panics if joining the thread fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_join(thread: NSTDThread) -> NSTDErrorCode {
    thread.join().expect("Failed to join a thread")
}

/// Detaches a thread from it's handle, allowing it to run in the background.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_thread_detach(thread: NSTDThread) {}

/// Puts the current thread to sleep for a specified number of seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 secs` - The number of seconds to put the thread to sleep for.
///
/// # Panics
///
/// Panics if `secs` is negative, overflows Rust's `Duration` structure, or is non-finite.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_sleep(secs: NSTDFloat64) {
    std::thread::sleep(Duration::from_secs_f64(secs));
}

/// Returns the number of recommended threads that a program should use.
///
/// # Parameters:
///
/// - `NSTDIOError *errc` - The operation error code.
///
/// # Returns
///
/// `NSTDUInt threads` - The estimated default amount of parallelism a program should use, 0 on
/// error.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_count(errc: &mut NSTDIOError) -> NSTDUInt {
    match std::thread::available_parallelism() {
        Ok(threads) => {
            *errc = NSTDIOError::NSTD_IO_ERROR_NONE;
            threads.get()
        }
        Err(err) => {
            *errc = NSTDIOError::from_err(err.kind());
            0
        }
    }
}
