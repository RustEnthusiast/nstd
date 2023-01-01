//! Thread spawning, joining, and detaching.
use crate::{
    core::{
        cstr::nstd_core_cstr_get_null,
        def::NSTDErrorCode,
        optional::NSTDOptional,
        result::NSTDResult,
        str::{nstd_core_str_as_cstr, NSTDOptionalStr, NSTDStr},
    },
    heap_ptr::NSTDHeapPtr,
    io::NSTDIOError,
    NSTDBool, NSTDFloat64, NSTDUInt,
};
use std::{
    thread::{Builder, JoinHandle, Thread, ThreadId},
    time::Duration,
};

/// Represents a running thread.
pub type NSTDThread = Box<JoinHandle<NSTDThreadResult>>;

/// A handle to a running thread.
pub type NSTDThreadHandle = Box<Thread>;

/// A thread's unique identifier.
pub type NSTDThreadID = Box<ThreadId>;

/// Describes the creation of a new thread.
///
/// This type is passed to the `nstd_thread_spawn_with_desc` function.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NSTDThreadDescriptor {
    /// The name of the thread.
    ///
    /// If present, this must not contain any null bytes.
    pub name: NSTDOptionalStr,
    /// The number of bytes that the thread's stack should have.
    ///
    /// Set this to 0 to let the host decide how much stack memory should be allocated.
    pub stack_size: NSTDUInt,
}

/// A thread function's return value.
pub type NSTDThreadResult = NSTDErrorCode;

/// Returned from `nstd_thread_join`, contains the thread function's return value on success.
pub type NSTDOptionalThreadResult = NSTDOptional<NSTDThreadResult>;

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
/// - `NSTDThreadResult (*thread_fn)(NSTDHeapPtr)` - The thread function.
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
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::optional::NSTDOptional,
///     heap_ptr::{nstd_heap_ptr_new_zeroed, NSTDHeapPtr},
///     thread::{nstd_thread_join, nstd_thread_spawn, NSTDThreadResult},
/// };
///
/// unsafe extern "C" fn thread_fn(data: NSTDHeapPtr) -> NSTDThreadResult {
///     0
/// }
///
/// let data = unsafe { nstd_heap_ptr_new_zeroed(0) };
/// if let Some(thread) = unsafe { nstd_thread_spawn(Some(thread_fn), data) } {
///     if let NSTDOptional::Some(errc) = nstd_thread_join(thread) {
///         assert!(errc == 0);
///     }
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn(
    thread_fn: Option<unsafe extern "C" fn(NSTDHeapPtr) -> NSTDThreadResult>,
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
/// - `NSTDThreadResult (*thread_fn)(NSTDHeapPtr)` - The thread function.
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
    thread_fn: Option<unsafe extern "C" fn(NSTDHeapPtr) -> NSTDThreadResult>,
    data: NSTDHeapPtr,
    desc: &NSTDThreadDescriptor,
) -> Option<NSTDThread> {
    if let Some(thread_fn) = thread_fn {
        // Create the thread builder.
        let mut builder = Builder::new();
        // Set the thread name.
        if let NSTDOptional::Some(name) = &desc.name {
            // Make sure `name` doesn't contain any null bytes.
            let c_name = nstd_core_str_as_cstr(name);
            if !nstd_core_cstr_get_null(&c_name).is_null() {
                return None;
            }
            builder = builder.name(name.as_str().to_string());
        }
        // Set the thread stack size.
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

/// Returns a handle to the calling thread.
///
/// # Returns
///
/// `NSTDThreadHandle handle` - A handle to the current thread.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_current() -> NSTDThreadHandle {
    Box::new(std::thread::current())
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
    Box::new(thread.thread().clone())
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
/// - `const NSTDThreadHandle *handle` - A handle to the thread.
///
/// # Returns
///
/// `NSTDOptionalStr name` - The name of the thread, or none if the thread is unnamed.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_name(handle: &NSTDThreadHandle) -> NSTDOptionalStr {
    match handle.name() {
        Some(name) => NSTDOptional::Some(NSTDStr::from_str(name)),
        _ => NSTDOptional::None,
    }
}

/// Returns a thread's unique identifier.
///
/// # Parameters:
///
/// - `const NSTDThreadHandle *handle` - A handle to the thread.
///
/// # Returns
///
/// `NSTDThreadID id` - The thread's unique ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_id(handle: &NSTDThreadHandle) -> NSTDThreadID {
    Box::new(handle.id())
}

/// Frees an instance of `NSTDThreadHandle`.
///
/// # Parameters:
///
/// - `NSTDThreadHandle handle` - The handle to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_thread_handle_free(handle: NSTDThreadHandle) {}

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
