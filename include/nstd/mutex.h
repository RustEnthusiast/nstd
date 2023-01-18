#ifndef NSTD_MUTEX_H
#define NSTD_MUTEX_H
#include "core/optional.h"
#include "core/result.h"
#include "heap_ptr.h"
#include "nstd.h"

/// A mutual exclusion primitive useful for protecting shared data.
typedef NSTDAnyMut NSTDMutex;

/// A guard providing access to a mutex's protected data.
typedef NSTDAnyMut NSTDMutexGuard;

/// A lock result returned from `nstd_mutex_lock` containing the mutex guard whether or not the
/// data is poisoned.
NSTDResult(NSTDMutexGuard, NSTDMutexGuard) NSTDMutexLockResult;

/// An optional value of type `NSTDMutexLockResult`.
///
/// This type is returned from `nstd_mutex_try_lock` where the uninitialized variant means that the
/// function would block.
NSTDOptional(NSTDMutexLockResult) NSTDOptionalMutexLockResult;

/// Creates a new mutual exclusion primitive.
///
/// # Parameters:
///
/// - `NSTDHeapPtr data` - The data to protect.
///
/// # Returns
///
/// `NSTDMutex mutex` - The new mutex protecting `data`.
NSTDAPI NSTDMutex nstd_mutex_new(NSTDHeapPtr data);

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
NSTDAPI NSTDMutexLockResult nstd_mutex_lock(const NSTDMutex *mutex);

/// Returns a pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `const NSTDMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAny data` - A pointer to the mutex's data.
NSTDAPI NSTDAny nstd_mutex_get(const NSTDMutexGuard *guard);

/// Returns a mutable pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `NSTDMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAnyMut data` - A mutable pointer to the mutex's data.
NSTDAPI NSTDAnyMut nstd_mutex_get_mut(NSTDMutexGuard *guard);

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
NSTDAPI NSTDOptionalMutexLockResult nstd_mutex_try_lock(const NSTDMutex *mutex);

/// Unlocks a mutex by consuming a mutex guard.
///
/// # Parameters:
///
/// - `NSTDMutexGuard guard` - The mutex guard.
NSTDAPI void nstd_mutex_unlock(NSTDMutexGuard guard);

/// Frees an instance of `NSTDMutex`.
///
/// # Parameters:
///
/// - `NSTDMutex mutex` - The mutex to free.
NSTDAPI void nstd_mutex_free(NSTDMutex mutex);

#endif
