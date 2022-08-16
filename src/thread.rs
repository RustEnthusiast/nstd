//! Thread spawning, joining, and detaching.
use crate::{core::def::NSTDErrorCode, NSTDAnyMut, NSTDFloat64};
use std::{thread::JoinHandle, time::Duration};

/// A handle to a running thread.
pub type NSTDThread = Box<JoinHandle<NSTDErrorCode>>;

/// Spawns a new thread and returns a handle to it.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDAnyMut)` - The thread function.
///
/// - `NSTDAnyMut data` - Data to pass to the thread. This will only be passed to `thread_fn` on
/// platforms that support atomic pointers, on other platforms `NSTD_NULL` will be passed.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread.
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(not(target_has_atomic = "ptr"), allow(unused_variables))]
pub extern "C" fn nstd_thread_spawn(
    thread_fn: Option<unsafe extern "C" fn(NSTDAnyMut) -> NSTDErrorCode>,
    data: NSTDAnyMut,
) -> Option<NSTDThread> {
    if let Some(thread_fn) = thread_fn {
        #[cfg(target_has_atomic = "ptr")]
        {
            use std::sync::atomic::AtomicPtr;
            let data = AtomicPtr::new(data);
            return Some(Box::new(std::thread::spawn(move || unsafe {
                thread_fn(data.into_inner())
            })));
        }
        #[cfg(not(target_has_atomic = "ptr"))]
        {
            use crate::NSTD_NULL;
            return Some(Box::new(std::thread::spawn(move || unsafe {
                thread_fn(NSTD_NULL)
            })));
        }
    }
    None
}

/// Puts the current thread to sleep for a specified number of seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 secs` - The number of seconds to put the thread to sleep for.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_sleep(secs: NSTDFloat64) {
    std::thread::sleep(Duration::from_secs_f64(secs));
}

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
///
/// # Returns
///
/// `NSTDErrorCode errc` - The thread functions return code. This can also be non-zero if joining
/// the thread fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_thread_join(thread: NSTDThread) -> NSTDErrorCode {
    match thread.join() {
        Ok(errc) => errc,
        _ => 1,
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
