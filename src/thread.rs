//! Thread spawning, joining, and detaching.
use crate::{
    core::{
        def::NSTDErrorCode,
        optional::NSTDOptional,
        result::NSTDResult,
        slice::nstd_core_slice_new,
        str::{nstd_core_str_from_bytes_unchecked, NSTDStr},
    },
    heap_ptr::NSTDHeapPtr,
    io::NSTDIOError,
    NSTDBool, NSTDFloat64, NSTDUInt, NSTD_NULL,
};
use std::{
    thread::{Builder, JoinHandle, Thread, ThreadId},
    time::Duration,
};

/// Represents a running thread.
pub type NSTDThread = Box<JoinHandle<NSTDErrorCode>>;

/// A handle to a running thread.
pub type NSTDThreadHandle<'a> = &'a Thread;

/// A thread's unique identifier.
pub type NSTDThreadID = Box<ThreadId>;

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

/// Returned from `nstd_thread_join`, contains the thread function's return value on success.
pub type NSTDOptionalThreadResult = NSTDOptional<NSTDErrorCode>;

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
/// `NSTDThread thread` - A handle to the new thread, null on error.
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
) -> Option<NSTDThread> {
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
///
/// - The data type that `data` holds must be able to be safely sent between threads.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn_with_desc(
    thread_fn: Option<unsafe extern "C" fn(NSTDHeapPtr) -> NSTDErrorCode>,
    data: NSTDHeapPtr,
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
        let data = ThreadData(data);
        if let Ok(thread) = builder.spawn(move || thread_fn(data.into())) {
            return Some(Box::new(thread));
        }
    }
    None
}

/// Retrieves a raw handle to a thread.
///
/// # Parameters:
///
/// - `const NSTDThread *thread` - A handle to the thread.
///
/// # Returns
///
/// `NSTDThreadHandle handle` - A raw handle to the thread.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_handle(thread: &NSTDThread) -> NSTDThreadHandle {
    thread.thread()
}

/// Checks if a thread has finished running.
///
/// # Parameters:
///
/// - `const NSTDThread *thread` - A handle to the thread.
///
/// # Returns
///
/// `NSTDBool is_finished` - True if the thread associated with the handle has finished executing.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_is_finished(thread: &NSTDThread) -> NSTDBool {
    thread.is_finished()
}

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
///
/// # Returns
///
/// `NSTDOptionalThreadResult errc` - The thread function's return code, or none if joining the
/// thread fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_join(thread: NSTDThread) -> NSTDOptionalThreadResult {
    match thread.join() {
        Ok(errc) => NSTDOptional::Some(errc),
        _ => NSTDOptional::None,
    }
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

/// Returns the name of a thread.
///
/// # Parameters:
///
/// - `NSTDThreadHandle handle` - A handle to the thread.
///
/// # Returns
///
/// `NSTDStr name` - The name of the thread, or an empty string slice if the thread is unnamed.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_name(handle: NSTDThreadHandle) -> NSTDStr {
    match handle.name() {
        Some(name) => NSTDStr::from_str(name),
        _ => {
            let empty = nstd_core_slice_new(NSTD_NULL, 1, 0);
            // SAFETY: `empty` is an empty slice.
            unsafe { nstd_core_str_from_bytes_unchecked(&empty) }
        }
    }
}

/// Returns a thread's unique identifier.
///
/// # Parameters:
///
/// - `NSTDThreadHandle handle` - A handle to the thread.
///
/// # Returns
///
/// `NSTDThreadID id` - The thread's unique ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_id(handle: NSTDThreadHandle) -> NSTDThreadID {
    Box::new(handle.id())
}

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

/// Compares two thread identifiers.
///
/// # Parameters:
///
/// - `const NSTDThreadID *xid` - The first identifier.
///
/// - `const NSTDThreadID *yid` - The second identifier.
///
/// # Returns
///
/// `NSTDBool is_eq` - True if the two identifiers refer to the same thread.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_id_compare(xid: &NSTDThreadID, yid: &NSTDThreadID) -> NSTDBool {
    xid == yid
}

/// Frees an instance of `NSTDThreadID`.
///
/// # Parameters:
///
/// - `NSTDThreadID id` - A thread identifier.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_thread_id_free(id: NSTDThreadID) {}
