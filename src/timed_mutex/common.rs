//! Contains cross-platform definitions for `NSTDTimedMutex`.
use super::{nstd_timed_mutex_free, nstd_timed_mutex_unlock};
use crate::{
    core::{optional::NSTDOptional, result::NSTDResult},
    heap_ptr::NSTDHeapPtr,
    NSTDAnyMut, NSTDBool,
};
use core::{marker::PhantomData, mem::ManuallyDrop};
use nstdapi::nstdapi;

/// A mutual exclusion primitive with a timed locking mechanism.
#[nstdapi]
pub struct NSTDTimedMutex {
    /// The underlying mutex.
    inner: NSTDAnyMut,
    /// The data to protect.
    data: ManuallyDrop<NSTDHeapPtr>,
    /// Determines whether or not the mutex is poisoned.
    poisoned: NSTDBool,
    /// Determines whether or not the mutex is currently locked.
    locked: NSTDBool,
}
impl Drop for NSTDTimedMutex {
    /// [NSTDTimedMutex]'s destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `NSTDTimeMutex` has been initialized and is valid for reads.
        unsafe { nstd_timed_mutex_free(core::ptr::read(self)) };
    }
}
/// # Safety
///
/// The data that the mutex is protecting must be able to be safely sent between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Send for NSTDTimedMutex {}
/// # Safety
///
/// The data that the mutex is protecting must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDTimedMutex {}

/// A handle to a timed mutex's data.
#[nstdapi]
pub struct NSTDTimedMutexGuard<'a> {
    /// A reference to the mutex.
    mutex: &'a NSTDTimedMutex,
    /// Ensures that the guard is not [Send].
    pd: PhantomData<*const ()>,
}
impl Drop for NSTDTimedMutexGuard<'_> {
    /// [NSTDTimedMutexGuard]'s destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `self` is a valid guard for the mutex.
        unsafe { nstd_timed_mutex_unlock(core::ptr::read(self)) };
    }
}
// # Safety
///
/// The data that the guard is protecting must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDTimedMutexGuard<'_> {}

/// A result type containing a timed mutex lock whether or not the mutex is poisoned.
pub type NSTDTimedMutexLockResult<'a> =
    NSTDResult<NSTDTimedMutexGuard<'a>, NSTDTimedMutexGuard<'a>>;

/// An optional value of type `NSTDTimedMutexLockResult`.
///
/// This type is returned from `nstd_timed_mutex_try_lock` where the uninitialized variant means
/// that the function would block.
pub type NSTDOptionalTimedMutexLockResult<'a> = NSTDOptional<NSTDTimedMutexLockResult<'a>>;
