//! Thread spawning, joining, and detaching.
use crate::{
    core::{
        cstr::nstd_core_cstr_get_null,
        optional::{gen_optional, NSTDOptional},
        result::NSTDResult,
        str::{nstd_core_str_as_cstr, NSTDOptionalStr, NSTDStr},
        time::NSTDDuration,
    },
    heap_ptr::NSTDOptionalHeapPtr,
    io::NSTDIOError,
    NSTDBool, NSTDUInt,
};
use nstdapi::nstdapi;
use std::thread::{Builder, JoinHandle, Thread, ThreadId};

/// Represents a running thread.
#[nstdapi]
pub struct NSTDThread {
    /// The thread join handle.
    thread: Box<JoinHandle<NSTDThreadResult>>,
}
gen_optional!(NSTDOptionalThread, NSTDThread);

/// A handle to a running thread.
#[nstdapi]
pub struct NSTDThreadHandle {
    /// A handle to the thread.
    handle: Box<Thread>,
}

/// A thread's unique identifier.
#[nstdapi]
pub struct NSTDThreadID {
    /// The thread ID.
    id: Box<ThreadId>,
}

/// Describes the creation of a new thread.
///
/// This type is passed to the `nstd_thread_spawn_with_desc` function.
#[nstdapi]
#[derive(Clone, Copy)]
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
pub type NSTDThreadResult = NSTDOptionalHeapPtr<'static>;

/// Returned from `nstd_thread_join`, contains the thread function's return value on success.
pub type NSTDOptionalThreadResult = NSTDOptional<NSTDThreadResult>;

/// Returned from `nstd_thread_count`, contains the number of threads detected on the system on
/// success.
pub type NSTDThreadCountResult = NSTDResult<NSTDUInt, NSTDIOError>;

/// Spawns a new thread executing the function `thread_fn` and returns a handle to the new thread.
///
/// # Parameters:
///
/// - `NSTDThreadResult (*thread_fn)(NSTDOptionalHeapPtr)` - The thread function.
///
/// - `NSTDOptionalHeapPtr data` - Data to send to the thread.
///
/// - `const NSTDThreadDescriptor *desc` - The thread descriptor. This value may be null.
///
/// # Returns
///
/// `NSTDOptionalThread thread` - A handle to the new thread on success, or an uninitialized "none"
/// variant on error.
///
/// # Safety
///
/// - The caller of this function must guarantee that `thread_fn` is a valid function pointer.
///
/// - This operation can cause undefined behavior if `desc.name`'s data is invalid.
///
/// - The data type that `data` holds must be able to be safely sent between threads.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::optional::NSTDOptional,
///     heap_ptr::NSTDOptionalHeapPtr,
///     thread::{nstd_thread_join, nstd_thread_spawn, NSTDThreadResult},
/// };
///
/// unsafe extern "C" fn thread_fn(data: NSTDOptionalHeapPtr) -> NSTDThreadResult {
///     NSTDOptional::None
/// }
///
/// let thread = unsafe { nstd_thread_spawn(thread_fn, NSTDOptional::None, None) };
/// if let NSTDOptional::Some(thread) = thread {
///     if let NSTDOptional::Some(ret) = unsafe { nstd_thread_join(thread) } {
///         if let NSTDOptional::Some(_) = ret {
///             panic!("this shouldn't be here");
///         }
///     }
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_thread_spawn(
    thread_fn: unsafe extern "C" fn(NSTDOptionalHeapPtr) -> NSTDThreadResult,
    data: NSTDOptionalHeapPtr<'static>,
    desc: Option<&NSTDThreadDescriptor>,
) -> NSTDOptionalThread {
    // Create the thread builder.
    let mut builder = Builder::new();
    if let Some(desc) = desc {
        // Set the thread name.
        if let NSTDOptional::Some(name) = &desc.name {
            // Make sure `name` doesn't contain any null bytes.
            let c_name = nstd_core_str_as_cstr(name);
            if !nstd_core_cstr_get_null(&c_name).is_null() {
                return NSTDOptional::None;
            }
            builder = builder.name(name.as_str().to_string());
        }
        // Set the thread stack size.
        if desc.stack_size != 0 {
            builder = builder.stack_size(desc.stack_size);
        }
    }
    // Spawn the new thread.
    match builder.spawn(move || thread_fn(data)) {
        Ok(thread) => NSTDOptional::Some(NSTDThread {
            thread: Box::new(thread),
        }),
        _ => NSTDOptional::None,
    }
}

/// Returns a handle to the calling thread.
///
/// # Returns
///
/// `NSTDThreadHandle handle` - A handle to the current thread.
#[inline]
#[nstdapi]
pub fn nstd_thread_current() -> NSTDThreadHandle {
    NSTDThreadHandle {
        handle: Box::new(std::thread::current()),
    }
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
#[nstdapi]
pub fn nstd_thread_handle(thread: &NSTDThread) -> NSTDThreadHandle {
    NSTDThreadHandle {
        handle: Box::new(thread.thread.thread().clone()),
    }
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
#[nstdapi]
pub fn nstd_thread_is_finished(thread: &NSTDThread) -> NSTDBool {
    thread.thread.is_finished()
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
///
/// # Safety
///
/// The data type that the thread function returns must be able to be safely sent between threads.
#[inline]
#[nstdapi]
pub unsafe fn nstd_thread_join(thread: NSTDThread) -> NSTDOptionalThreadResult {
    match thread.thread.join() {
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
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_thread_detach(thread: NSTDThread) {}

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
#[nstdapi]
pub fn nstd_thread_name(handle: &NSTDThreadHandle) -> NSTDOptionalStr {
    match handle.handle.name() {
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
#[nstdapi]
pub fn nstd_thread_id(handle: &NSTDThreadHandle) -> NSTDThreadID {
    NSTDThreadID {
        id: Box::new(handle.handle.id()),
    }
}

/// Frees an instance of `NSTDThreadHandle`.
///
/// # Parameters:
///
/// - `NSTDThreadHandle handle` - The handle to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_thread_handle_free(handle: NSTDThreadHandle) {}

/// Puts the current thread to sleep for a specified duration.
///
/// # Parameters:
///
/// - `NSTDDuration duration` - The duration to put the thread to sleep for.
///
/// # Panics
///
/// Panics if `duration` is negative, overflows Rust's `Duration` structure, or is non-finite.
#[inline]
#[nstdapi]
pub fn nstd_thread_sleep(duration: NSTDDuration) {
    std::thread::sleep(duration.into_duration());
}

/// Returns the number of recommended threads that a program should use.
///
/// # Returns
///
/// `NSTDThreadCountResult threads` - The estimated default amount of parallelism a program should
/// use on success, or the I/O error code on failure.
#[inline]
#[nstdapi]
pub fn nstd_thread_count() -> NSTDThreadCountResult {
    match std::thread::available_parallelism() {
        Ok(threads) => NSTDResult::Ok(threads.get()),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Checks if the current thread is unwinding due to a panic.
///
/// # Returns
///
/// `NSTDBool is_panicking` - Determines whether or not the calling thread is panicking.
#[inline]
#[nstdapi]
pub fn nstd_thread_is_panicking() -> NSTDBool {
    #[cfg(panic = "unwind")]
    return std::thread::panicking();
    #[cfg(panic = "abort")]
    return false;
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
#[nstdapi]
pub fn nstd_thread_id_compare(xid: &NSTDThreadID, yid: &NSTDThreadID) -> NSTDBool {
    xid.id == yid.id
}

/// Frees an instance of `NSTDThreadID`.
///
/// # Parameters:
///
/// - `NSTDThreadID id` - A thread identifier.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_thread_id_free(id: NSTDThreadID) {}
