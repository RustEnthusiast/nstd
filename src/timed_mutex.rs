//! A mutual exclusion primitive with a timed locking mechanism.
use crate::{
    core::time::NSTDDuration,
    heap_ptr::{NSTDHeapPtr, NSTDOptionalHeapPtr},
    NSTDAny, NSTDAnyMut, NSTDBool,
};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "haiku",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "nto",
        target_os = "openbsd",
        target_os = "solaris"
    ))] {
        use crate::os::unix::mutex::{
            NSTDUnixMutex, NSTDUnixMutexGuard, NSTDUnixMutexLockResult, NSTDUnixOptionalMutex,
            NSTDUnixOptionalMutexLockResult,
        };

        /// A mutual exclusion primitive with a timed locking mechanism.
        pub type NSTDTimedMutex<'a> = NSTDUnixMutex<'a>;

        /// Represents an optional value of type `NSTDTimedMutex`.
        pub type NSTDOptionalTimedMutex<'a> = NSTDUnixOptionalMutex<'a>;

        /// A handle to a timed mutex's data.
        pub type NSTDTimedMutexGuard<'m, 'a> = NSTDUnixMutexGuard<'m, 'a>;

        /// A result type containing a timed mutex lock whether or not the mutex is poisoned.
        pub type NSTDTimedMutexLockResult<'m, 'a> = NSTDUnixMutexLockResult<'m, 'a>;

        /// An optional value of type `NSTDTimedMutexLockResult`.
        ///
        /// This type is returned from `nstd_timed_mutex_try_lock` where the uninitialized variant
        /// means that the function would block.
        pub type NSTDOptionalTimedMutexLockResult<'m, 'a> = NSTDUnixOptionalMutexLockResult<'m, 'a>;
    } else {
        use crate::core::{optional::NSTDOptional, result::NSTDResult};
        use core::{marker::PhantomData, mem::ManuallyDrop};
        use nstdapi::nstdapi;

        /// A mutual exclusion primitive with a timed locking mechanism.
        #[nstdapi]
        pub struct NSTDTimedMutex<'a> {
            /// The underlying mutex.
            inner: NSTDAnyMut,
            /// The data to protect.
            data: ManuallyDrop<NSTDHeapPtr<'a>>,
            /// Determines whether or not the mutex is poisoned.
            poisoned: NSTDBool,
            /// Determines whether or not the mutex is currently locked.
            locked: NSTDBool,
        }
        impl Drop for NSTDTimedMutex<'_> {
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
        unsafe impl Send for NSTDTimedMutex<'_> {}
        /// # Safety
        ///
        /// The data that the mutex is protecting must be able to be safely shared between threads.
        // SAFETY: The user guarantees that the data is thread-safe.
        unsafe impl Sync for NSTDTimedMutex<'_> {}

        /// Represents an optional value of type `NSTDTimedMutex`.
        pub type NSTDOptionalTimedMutex<'a> = NSTDOptional<NSTDTimedMutex<'a>>;

        /// A handle to a timed mutex's data.
        #[nstdapi]
        pub struct NSTDTimedMutexGuard<'m, 'a> {
            /// A reference to the mutex.
            mutex: &'m NSTDTimedMutex<'a>,
            /// Ensures that the guard is not [Send].
            pd: PhantomData<*const ()>,
        }
        impl Drop for NSTDTimedMutexGuard<'_, '_> {
            /// [NSTDTimedMutexGuard]'s destructor.
            #[inline]
            fn drop(&mut self) {
                // SAFETY: `self` is a valid guard for the mutex.
                unsafe { nstd_timed_mutex_unlock(core::ptr::read(self)) };
            }
        }
        /// # Safety
        ///
        /// The data that the guard is protecting must be able to be safely shared between threads.
        // SAFETY: The user guarantees that the data is thread-safe.
        unsafe impl Sync for NSTDTimedMutexGuard<'_, '_> {}

        /// A result type containing a timed mutex lock whether or not the mutex is poisoned.
        pub type NSTDTimedMutexLockResult<'m, 'a> =
            NSTDResult<NSTDTimedMutexGuard<'m, 'a>, NSTDTimedMutexGuard<'m, 'a>>;

        /// An optional value of type `NSTDTimedMutexLockResult`.
        ///
        /// This type is returned from `nstd_timed_mutex_try_lock` where the uninitialized variant
        /// means that the function would block.
        pub type NSTDOptionalTimedMutexLockResult<'m, 'a> =
            NSTDOptional<NSTDTimedMutexLockResult<'m, 'a>>;
    }
}

extern "C" {
    /// Creates a new timed mutual exclusion primitive.
    ///
    /// # Parameters:
    ///
    /// - `NSTDHeapPtr data` - The data to protect.
    ///
    /// # Returns
    ///
    /// `NSTDOptionalTimedMutex mutex` - The new mutex protecting `data` on success, or an
    /// uninitialized "none" value if the OS failed to initialize the mutex.
    pub fn nstd_timed_mutex_new(data: NSTDHeapPtr<'_>) -> NSTDOptionalTimedMutex<'_>;

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
    pub fn nstd_timed_mutex_is_poisoned(mutex: &NSTDTimedMutex<'_>) -> NSTDBool;

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
    /// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data, or an
    /// uninitialized "none" value if the OS fails to lock the mutex.
    ///
    /// # Safety
    ///
    /// The mutex lock must not already be owned by the calling thread.
    pub fn nstd_timed_mutex_lock<'m, 'a>(
        mutex: &'m NSTDTimedMutex<'a>,
    ) -> NSTDOptionalTimedMutexLockResult<'m, 'a>;

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
    pub fn nstd_timed_mutex_try_lock<'m, 'a>(
        mutex: &'m NSTDTimedMutex<'a>,
    ) -> NSTDOptionalTimedMutexLockResult<'m, 'a>;

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
    /// - `NSTDDuration duration` - The amount of time to block for.
    ///
    /// # Returns
    ///
    /// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data.
    ///
    /// # Safety
    ///
    /// The mutex lock must not already be owned by the calling thread.
    pub fn nstd_timed_mutex_timed_lock<'m, 'a>(
        mutex: &'m NSTDTimedMutex<'a>,
        duration: NSTDDuration,
    ) -> NSTDOptionalTimedMutexLockResult<'m, 'a>;

    /// Returns an immutable raw pointer to a timed mutex guard's protected data.
    ///
    /// # Parameters:
    ///
    /// - `const NSTDTimedMutexGuard *guard` - The mutex guard.
    ///
    /// # Returns
    ///
    /// `NSTDAny data` - A pointer to the guard's protected data.
    pub fn nstd_timed_mutex_get(guard: &NSTDTimedMutexGuard<'_, '_>) -> NSTDAny;

    /// Returns an raw pointer to a timed mutex guard's protected data.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutexGuard *guard` - The mutex guard.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut data` - A pointer to the guard's protected data.
    pub fn nstd_timed_mutex_get_mut(guard: &mut NSTDTimedMutexGuard<'_, '_>) -> NSTDAnyMut;

    /// Consumes a timed mutex and returns the data it was protecting.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutex mutex` - The mutex to take ownership of.
    ///
    /// # Returns
    ///
    /// `NSTDOptionalHeapPtr data` - Ownership of the mutex's data, or an uninitialized "none"
    /// variant if the mutex was poisoned.
    pub fn nstd_timed_mutex_into_inner(mutex: NSTDTimedMutex<'_>) -> NSTDOptionalHeapPtr<'_>;

    /// Unlocks a timed mutex by consuming a mutex guard.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutexGuard guard` - The mutex guard.
    pub fn nstd_timed_mutex_unlock(guard: NSTDTimedMutexGuard<'_, '_>);

    /// Frees an instance of `NSTDTimedMutex`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutex mutex` - The timed mutex to free.
    pub fn nstd_timed_mutex_free(mutex: NSTDTimedMutex<'_>);

    /// Frees an instance of `NSTDTimedMutex` after invoking `callback` with the mutex's data.
    ///
    /// `callback` will not be called if the mutex is poisoned.
    ///
    /// # Parameters:
    ///
    /// - `NSTDTimedMutex mutex` - The timed mutex to free.
    ///
    /// - `void (*callback)(NSTDAnyMut)` - The mutex data's destructor.
    ///
    /// # Safety
    ///
    /// This operation makes a direct call on a C function pointer (`callback`).
    pub fn nstd_timed_mutex_drop(
        mutex: NSTDTimedMutex<'_>,
        callback: unsafe extern "C" fn(NSTDAnyMut),
    );
}
