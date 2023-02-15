//! A mutual exclusion primitive with a timed locking mechanism.
#[cfg(not(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "solaris"
)))]
mod common;
#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "solaris"
))]
mod unix;
#[cfg(not(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "solaris"
)))]
pub use self::common::*;
#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "solaris"
))]
pub use self::unix::*;
use crate::{heap_ptr::NSTDHeapPtr, NSTDAny, NSTDAnyMut, NSTDBool, NSTDFloat64};

extern "C" {
    /// Creates a new timed mutual exclusion primitive.
    ///
    /// # Parameters:
    ///
    /// - `NSTDHeapPtr data` - The data to protect.
    ///
    /// # Returns
    ///
    /// `NSTDTimedMutex mutex` - The new mutex protecting `data`.
    ///
    /// # Panics
    ///
    /// This operation will panic if creating the timed mutex fails.
    pub fn nstd_timed_mutex_new(data: NSTDHeapPtr) -> NSTDTimedMutex;

    /// Determines whether or not a timed mutex's data is poisoned.
    ///
    /// Mutexes are poisoned when a thread that owns the mutex guard panics. This function is useful
    /// for those that configure `nstd` to unwind the stack instead of aborting on panic.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDTimedMutex *mutex` - The mutex.
    ///
    /// # Returns
    ///
    /// `NSTDBool is_poisoned` - A boolean value indicating whether or not `mutex` is poisoned.
    pub fn nstd_timed_mutex_is_poisoned(mutex: &NSTDTimedMutex) -> NSTDBool;

    /// Waits for a timed mutex lock to become acquired, returning a guard wrapping the protected data.
    ///
    /// Attempting to call this function on a thread that already owns the lock will result in
    /// undefined behavior.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDTimedMutex *mutex` - The mutex to lock.
    ///
    /// # Returns
    ///
    /// `NSTDTimedMutexLockResult guard` - A handle to the mutex's protected data.
    ///
    /// # Panics
    ///
    /// This operation will panic if locking the mutex fails, this includes the situation where the
    /// calling thread already owns the mutex lock.
    ///
    /// # Safety
    ///
    /// The mutex lock must not already be owned by the calling thread.
    pub fn nstd_timed_mutex_lock(mutex: &NSTDTimedMutex) -> NSTDTimedMutexLockResult;

    /// The non-blocking variant of `nstd_timed_mutex_lock` returning an uninitialized "none" result if
    /// the mutex is locked by another thread.
    ///
    /// Attempting to call this function on a thread that already owns the lock will result in
    /// undefined behavior.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDTimedMutex *mutex` - The mutex to lock.
    ///
    /// # Returns
    ///
    /// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data.
    ///
    /// # Safety
    ///
    /// The mutex lock must not already be owned by the calling thread.
    pub fn nstd_timed_mutex_try_lock(mutex: &NSTDTimedMutex) -> NSTDOptionalTimedMutexLockResult;

    /// The timed variant of `nstd_timed_mutex_lock` returning an uninitialized "none" result if
    /// the mutex lock could not be acquired after a specified number of `seconds`.
    ///
    /// Attempting to call this function on a thread that already owns the lock will result in
    /// undefined behavior.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDTimedMutex *mutex` - The mutex to lock.
    ///
    /// - `NSTDFloat64 seconds` - The number of seconds to block for.
    ///
    /// # Returns
    ///
    /// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data.
    ///
    /// # Safety
    ///
    /// The mutex lock must not already be owned by the calling thread.
    pub fn nstd_timed_mutex_timed_lock(
        mutex: &NSTDTimedMutex,
        seconds: NSTDFloat64,
    ) -> NSTDOptionalTimedMutexLockResult;

    /// Returns an immutable raw pointer to a timed mutex guard's protected data.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDTimedMutexGuard *guard` - The mutex guard.
    ///
    /// # Returns
    ///
    /// `NSTDAny data` - A pointer to the guard's protected data.
    pub fn nstd_timed_mutex_get(guard: &NSTDTimedMutexGuard) -> NSTDAny;

    /// Returns an raw pointer to a timed mutex guard's protected data.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutexGuard *guard` - The mutex guard.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut data` - A pointer to the guard's protected data.
    pub fn nstd_timed_mutex_get_mut(guard: &mut NSTDTimedMutexGuard) -> NSTDAnyMut;

    /// Unlocks a timed mutex by consuming a mutex guard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutexGuard guard` - The mutex guard.
    pub fn nstd_timed_mutex_unlock(guard: NSTDTimedMutexGuard);

    /// Frees an instance of `NSTDTimedMutex`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutex mutex` - The timed mutex to free.
    pub fn nstd_timed_mutex_free(mutex: NSTDTimedMutex);
}
