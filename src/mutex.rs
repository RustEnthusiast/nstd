//! A mutual exclusion primitive useful for protecting shared data.
use crate::{
    alloc::CBox,
    core::{optional::NSTDOptional, result::NSTDResult},
    heap_ptr::{
        nstd_heap_ptr_drop, nstd_heap_ptr_get, nstd_heap_ptr_get_mut, NSTDHeapPtr,
        NSTDOptionalHeapPtr,
    },
    NSTDAny, NSTDAnyMut, NSTDBool,
};
use nstdapi::nstdapi;
use std::sync::{Mutex, MutexGuard, TryLockError};

/// A mutual exclusion primitive useful for protecting shared data.
#[nstdapi]
pub struct NSTDMutex<'a> {
    /// The Rust [Mutex].
    mtx: CBox<Mutex<NSTDHeapPtr<'a>>>,
}

/// Represents an optional value of type `NSTDMutex`.
pub type NSTDOptionalMutex<'a> = NSTDOptional<NSTDMutex<'a>>;

/// A guard providing access to a mutex's protected data.
#[nstdapi]
pub struct NSTDMutexGuard<'m, 'a> {
    /// The Rust [MutexGuard].
    guard: CBox<MutexGuard<'m, NSTDHeapPtr<'a>>>,
}

/// A lock result returned from `nstd_mutex_lock` containing the mutex guard whether or not the
/// data is poisoned.
pub type NSTDMutexLockResult<'m, 'a> = NSTDResult<NSTDMutexGuard<'m, 'a>, NSTDMutexGuard<'m, 'a>>;

/// An optional value of type `NSTDMutexLockResult`.
///
/// This type is returned from `nstd_mutex_try_lock` where the uninitialized variant means that the
/// function would block.
pub type NSTDOptionalMutexLockResult<'m, 'a> = NSTDOptional<NSTDMutexLockResult<'m, 'a>>;

/// Creates a new mutual exclusion primitive.
///
/// # Parameters:
///
/// - `NSTDHeapPtr data` - The data to protect.
///
/// # Returns
///
/// `NSTDOptionalMutex mutex` - The new mutex protecting `data` on success, or an uninitialized
/// "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_mutex_new(data: NSTDHeapPtr<'_>) -> NSTDOptionalMutex<'_> {
    CBox::new(Mutex::new(data)).map_or(NSTDOptional::None, |mtx| {
        NSTDOptional::Some(NSTDMutex { mtx })
    })
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
#[nstdapi]
pub fn nstd_mutex_is_poisoned(mutex: &NSTDMutex<'_>) -> NSTDBool {
    mutex.mtx.is_poisoned()
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
/// `NSTDOptionalMutexLockResult guard` - A handle to the mutex's protected data on success, or an
/// uninitialized "none" variant on error.
///
/// # Panics
///
/// This operation may panic if the lock is already held by the current thread.
#[nstdapi]
pub fn nstd_mutex_lock<'m, 'a>(mutex: &'m NSTDMutex<'a>) -> NSTDOptionalMutexLockResult<'m, 'a> {
    match mutex.mtx.lock() {
        Ok(guard) => CBox::new(guard).map_or(NSTDOptional::None, |guard| {
            NSTDOptional::Some(NSTDResult::Ok(NSTDMutexGuard { guard }))
        }),
        Err(err) => CBox::new(err.into_inner()).map_or(NSTDOptional::None, |guard| {
            NSTDOptional::Some(NSTDResult::Err(NSTDMutexGuard { guard }))
        }),
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
#[nstdapi]
pub fn nstd_mutex_try_lock<'m, 'a>(
    mutex: &'m NSTDMutex<'a>,
) -> NSTDOptionalMutexLockResult<'m, 'a> {
    match mutex.mtx.try_lock() {
        Ok(guard) => CBox::new(guard).map_or(NSTDOptional::None, |guard| {
            NSTDOptional::Some(NSTDResult::Ok(NSTDMutexGuard { guard }))
        }),
        Err(err) => match err {
            TryLockError::WouldBlock => NSTDOptional::None,
            TryLockError::Poisoned(err) => CBox::new(err.into_inner())
                .map_or(NSTDOptional::None, |guard| {
                    NSTDOptional::Some(NSTDResult::Err(NSTDMutexGuard { guard }))
                }),
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
#[nstdapi]
pub fn nstd_mutex_get(guard: &NSTDMutexGuard<'_, '_>) -> NSTDAny {
    nstd_heap_ptr_get(&guard.guard)
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
#[nstdapi]
pub fn nstd_mutex_get_mut(guard: &mut NSTDMutexGuard<'_, '_>) -> NSTDAnyMut {
    nstd_heap_ptr_get_mut(&mut guard.guard)
}

/// Consumes a mutex and returns the data it was protecting.
///
/// # Parameters:
///
/// - `NSTDMutex mutex` - The mutex to take ownership of.
///
/// # Returns
///
/// `NSTDOptionalHeapPtr data` - Ownership of the mutex's data, or an uninitialized "none" variant
/// if the mutex was poisoned.
#[inline]
#[nstdapi]
pub fn nstd_mutex_into_inner(mutex: NSTDMutex<'_>) -> NSTDOptionalHeapPtr<'_> {
    mutex
        .mtx
        .into_inner()
        .into_inner()
        .map_or(NSTDOptional::None, NSTDOptional::Some)
}

/// Unlocks a mutex by consuming a mutex guard.
///
/// # Parameters:
///
/// - `NSTDMutexGuard guard` - The mutex guard.
#[inline]
#[nstdapi]
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
pub fn nstd_mutex_unlock(guard: NSTDMutexGuard<'_, '_>) {}

/// Frees an instance of `NSTDMutex`.
///
/// # Parameters:
///
/// - `NSTDMutex mutex` - The mutex to free.
#[inline]
#[nstdapi]
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
pub fn nstd_mutex_free(mutex: NSTDMutex<'_>) {}

/// Frees an instance of `NSTDMutex` after invoking `callback` with the mutex's data.
///
/// `callback` will not be called if the mutex is poisoned.
///
/// # Parameters:
///
/// - `NSTDMutex mutex` - The mutex to free.
///
/// - `void (*callback)(NSTDAnyMut)` - The mutex data's destructor.
///
/// # Safety
///
/// This operation makes a direct call on a C function pointer (`callback`).
#[inline]
#[nstdapi]
pub unsafe fn nstd_mutex_drop(mutex: NSTDMutex<'_>, callback: unsafe extern "C" fn(NSTDAnyMut)) {
    if let Ok(data) = mutex.mtx.into_inner().into_inner() {
        nstd_heap_ptr_drop(data, callback);
    }
}
