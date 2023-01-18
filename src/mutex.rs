//! A mutual exclusion primitive useful for protecting shared data.
use crate::{
    core::{optional::NSTDOptional, result::NSTDResult},
    heap_ptr::{nstd_heap_ptr_get, nstd_heap_ptr_get_mut, NSTDHeapPtr},
    NSTDAny, NSTDAnyMut, NSTDBool,
};
use std::sync::{Mutex, MutexGuard, TryLockError};

/// A mutual exclusion primitive useful for protecting shared data.
pub type NSTDMutex = Box<Mutex<NSTDHeapPtr>>;

/// A guard providing access to a mutex's protected data.
pub type NSTDMutexGuard<'a> = Box<MutexGuard<'a, NSTDHeapPtr>>;

/// A lock result returned from `nstd_mutex_lock` containing the mutex guard whether or not the
/// data is poisoned.
pub type NSTDMutexLockResult<'a> = NSTDResult<NSTDMutexGuard<'a>, NSTDMutexGuard<'a>>;

/// An optional value of type `NSTDMutexLockResult`.
///
/// This type is returned from `nstd_mutex_try_lock` where the uninitialized variant means that the
/// function would block.
pub type NSTDOptionalMutexLockResult<'a> = NSTDOptional<NSTDMutexLockResult<'a>>;

/// Creates a new mutual exclusion primitive.
///
/// # Parameters:
///
/// - `NSTDHeapPtr data` - The data to protect.
///
/// # Returns
///
/// `NSTDMutex mutex` - The new mutex protecting `data`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_mutex_new(data: NSTDHeapPtr) -> NSTDMutex {
    Box::new(Mutex::new(data))
}

/// Determines whether or not a mutex's data is poisoned.
///
/// Mutexes are poisoned when a thread that owns the mutex guard panics. This function is useful
/// for those that configure `nstd` to unwind the stack instead of aborting on panic.
///
/// # Parameters:
///
/// - `const NSTDMutex *mutex` - The mutex.
///
/// # Returns
///
/// `NSTDBool is_poisoned` - A boolean value indicating whether or not `mutex` is poisoned.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_mutex_is_poisoned(mutex: &NSTDMutex) -> NSTDBool {
    mutex.is_poisoned()
}

/// Waits for a mutex lock to become acquired, returning a guard wrapping the protected data.
///
/// Attempting to call this function on a thread that already owns the lock will either result in a
/// panic or a deadlock.
///
/// # Parameters:
///
/// - `const NSTDMutex *mutex` - The mutex to lock.
///
/// # Returns
///
/// `NSTDMutexLockResult guard` - A handle to the mutex's protected data.
///
/// # Panics
///
/// This operation may panic if the lock is already held by the current thread.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_mutex_lock(mutex: &NSTDMutex) -> NSTDMutexLockResult {
    match mutex.lock() {
        Ok(guard) => NSTDResult::Ok(Box::new(guard)),
        Err(err) => NSTDResult::Err(Box::new(err.into_inner())),
    }
}

/// The non-blocking variant of `nstd_mutex_lock` returning an uninitialized "none" result if the
/// mutex is locked by another thread.
///
/// # Parameters:
///
/// - `const NSTDMutex *mutex` - The mutex to lock.
///
/// # Returns
///
/// `NSTDOptionalMutexLockResult guard` - A handle to the mutex's protected data.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_mutex_try_lock(mutex: &NSTDMutex) -> NSTDOptionalMutexLockResult {
    match mutex.try_lock() {
        Ok(guard) => NSTDOptional::Some(NSTDResult::Ok(Box::new(guard))),
        Err(err) => match err {
            TryLockError::WouldBlock => NSTDOptional::None,
            TryLockError::Poisoned(err) => {
                NSTDOptional::Some(NSTDResult::Err(Box::new(err.into_inner())))
            }
        },
    }
}

/// Returns a pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `const NSTDMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAny data` - A pointer to the mutex's data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_mutex_get(guard: &NSTDMutexGuard) -> NSTDAny {
    nstd_heap_ptr_get(guard)
}

/// Returns a mutable pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `NSTDMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAnyMut data` - A mutable pointer to the mutex's data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_mutex_get_mut(guard: &mut NSTDMutexGuard) -> NSTDAnyMut {
    nstd_heap_ptr_get_mut(guard)
}

/// Unlocks a mutex by consuming a mutex guard.
///
/// # Parameters:
///
/// - `NSTDMutexGuard guard` - The mutex guard.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_mutex_unlock(guard: NSTDMutexGuard) {}

/// Frees an instance of `NSTDMutex`.
///
/// # Parameters:
///
/// - `NSTDMutex mutex` - The mutex to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_mutex_free(mutex: NSTDMutex) {}
