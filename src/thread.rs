//! Thread spawning, joining, and detaching.
use crate::{
    core::{def::NSTDErrorCode, str::NSTDStrConst},
    NSTDAnyMut, NSTDFloat64, NSTDUSize,
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
    pub name: NSTDStrConst,
    /// A pointer to the data to be passed to the thread.
    ///
    /// # Note
    ///
    /// This will only be passed to the thread function on platforms that support atomic pointers,
    /// on other platforms `NSTD_NULL` will be passed.
    pub data: NSTDAnyMut,
    /// The number of bytes that the thread's stack should have.
    pub stack_size: NSTDUSize,
}

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
///
/// # Safety
///
/// The caller of this function must guarantee that `thread_fn` is a valid function pointer.
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(not(target_has_atomic = "ptr"), allow(unused_variables))]
pub unsafe extern "C" fn nstd_thread_spawn(
    thread_fn: Option<unsafe extern "C" fn(NSTDAnyMut) -> NSTDErrorCode>,
    data: NSTDAnyMut,
) -> Option<NSTDThread> {
    if let Some(thread_fn) = thread_fn {
        #[cfg(target_has_atomic = "ptr")]
        {
            use std::sync::atomic::AtomicPtr;
            let data = AtomicPtr::new(data);
            return Some(Box::new(std::thread::spawn(move || {
                thread_fn(data.into_inner())
            })));
        }
        #[cfg(not(target_has_atomic = "ptr"))]
        {
            use crate::NSTD_NULL;
            return Some(Box::new(std::thread::spawn(move || thread_fn(NSTD_NULL))));
        }
    }
    None
}

/// Spawns a new thread configured with a descriptor.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDAnyMut)` - The thread function.
///
/// - `const NSTDThreadDescriptor *desc` - The thread descriptor.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread.
///
/// # Safety
///
/// This operation can cause undefined behavior if `desc`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn_with_desc(
    thread_fn: Option<unsafe extern "C" fn(NSTDAnyMut) -> NSTDErrorCode>,
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
        #[cfg(target_has_atomic = "ptr")]
        {
            use std::sync::atomic::AtomicPtr;
            let data = AtomicPtr::new(desc.data);
            return match builder.spawn(move || thread_fn(data.into_inner())) {
                Ok(thread) => Some(Box::new(thread)),
                _ => None,
            };
        }
        #[cfg(not(target_has_atomic = "ptr"))]
        {
            use crate::NSTD_NULL;
            return match builder.spawn(move || thread_fn(NSTD_NULL)) {
                Ok(thread) => return Some(Box::new(thread)),
                _ => None,
            };
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
