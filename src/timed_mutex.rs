//! A mutual exclusion primitive with a timed locking mechanism.
use crate::{
    core::{optional::NSTDOptional, result::NSTDResult},
    heap_ptr::NSTDHeapPtr,
    NSTDAny, NSTDAnyMut, NSTDBool, NSTDFloat64,
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
