//! Thread spawning, joining, and detaching.
use crate::{
    core::{def::NSTDErrorCode, result::NSTDResult, str::NSTDStr},
    heap_ptr::NSTDHeapPtr,
    io::NSTDIOError,
    NSTDBool, NSTDFloat64, NSTDUInt,
};
use std::{
    thread::{Builder, JoinHandle},
    time::Duration,
};

/// A handle to a running thread.
pub type NSTDThreadHandle = Box<JoinHandle<NSTDErrorCode>>;

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

/// Returned from `nstd_thread_count`, contains the number of threads detected on the system on
/// success.
pub type NSTDThreadCountResult = NSTDResult<NSTDUInt, NSTDIOError>;

/// Data type that wraps [NSTDHeapPtr] and implements the [Send] trait.
struct ThreadData(NSTDHeapPtr);
// SAFETY: `nstd_thread_spawn` documents the safety of passing data between threads.
unsafe impl Send for ThreadData {}
impl From<ThreadData> for NSTDHeapPtr {
    /// Consumes `data` returning the inner heap pointer.
    #[inline]
    fn from(data: ThreadData) -> Self {
        data.0
    }
}

/// Spawns a new thread and returns a handle to it.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDHeapPtr)` - The thread function.
///
/// - `NSTDHeapPtr data` - Data to pass to the thread.
///
/// # Returns
///
/// `NSTDThreadHandle thread` - A handle to the new thread, null on error.
///
/// # Safety
///
/// - The caller of this function must guarantee that `thread_fn` is a valid function pointer.
///
/// - The data type that `data` holds must be able to be safely sent between threads.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn(
    thread_fn: Option<unsafe extern "C" fn(NSTDHeapPtr) -> NSTDErrorCode>,
    data: NSTDHeapPtr,
) -> Option<NSTDThreadHandle> {
    if let Some(thread_fn) = thread_fn {
        let data = ThreadData(data);
        if let Ok(thread) = Builder::new().spawn(move || thread_fn(data.into())) {
            return Some(Box::new(thread));
        }
    }
    None
}

/// Spawns a new thread configured with a descriptor.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDHeapPtr)` - The thread function.
///
/// - `NSTDHeapPtr data` - Data to pass to the thread.
///
/// - `const NSTDThreadDescriptor *desc` - The thread descriptor.
///
/// # Returns
///
/// `NSTDThreadHandle thread` - A handle to the new thread, null on error.
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
///
/// - The data type that `data` holds must be able to be safely sent between threads.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn_with_desc(
    thread_fn: Option<unsafe extern "C" fn(NSTDHeapPtr) -> NSTDErrorCode>,
    data: NSTDHeapPtr,
    desc: &NSTDThreadDescriptor,
) -> Option<NSTDThreadHandle> {
    if let Some(thread_fn) = thread_fn {
        // Create the thread builder.
        let mut builder = Builder::new();
        builder = builder.name(desc.name.as_str().to_string());
        if desc.stack_size != 0 {
            builder = builder.stack_size(desc.stack_size);
        }
        // Spawn the new thread.
        let data = ThreadData(data);
        if let Ok(thread) = builder.spawn(move || thread_fn(data.into())) {
            return Some(Box::new(thread));
        }
    }
    None
}

/// Checks if a thread has finished running.
///
/// # Parameters:
///
/// - `const NSTDThreadHandle *thread` - A handle to the thread.
///
/// # Returns
///
/// `NSTDBool is_finished` - True if the thread associated with the handle has finished executing.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_is_finished(thread: &NSTDThreadHandle) -> NSTDBool {
    thread.is_finished()
}

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThreadHandle thread` - The thread handle.
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
pub extern "C" fn nstd_thread_join(thread: NSTDThreadHandle) -> NSTDErrorCode {
    thread.join().expect("Failed to join a thread")
}

/// Detaches a thread from it's handle, allowing it to run in the background.
///
/// # Parameters:
///
/// - `NSTDThreadHandle thread` - The thread handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_thread_detach(thread: NSTDThreadHandle) {}

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
/// # Returns
///
/// `NSTDThreadCountResult threads` - The estimated default amount of parallelism a program should
/// use on success, or the I/O error code on failure.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_count() -> NSTDThreadCountResult {
    match std::thread::available_parallelism() {
        Ok(threads) => NSTDResult::Ok(threads.get()),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}
