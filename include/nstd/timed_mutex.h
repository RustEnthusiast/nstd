#ifndef NSTD_TIMED_MUTEX_H
#define NSTD_TIMED_MUTEX_H
#include "core/optional.h"
#include "core/result.h"
#include "heap_ptr.h"
#include "nstd.h"

/// A mutual exclusion primitive with a timed locking mechanism.
typedef struct {
    /// The underlying mutex.
    NSTDAnyMut inner;
    /// The data to protect.
    NSTDHeapPtr data;
    /// Determines whether or not the mutex is poisoned.
    NSTDBool poisoned;
    /// Determines whether or not the mutex is currently locked.
    NSTDBool locked;
} NSTDTimedMutex;

/// A handle to a timed mutex's data.
typedef struct {
    /// A reference to the mutex.
    const NSTDTimedMutex *mutex;
} NSTDTimedMutexGuard;

/// A result type containing a timed mutex lock whether or not the mutex is poisoned.
NSTDResult(NSTDTimedMutexGuard, NSTDTimedMutexGuard) NSTDTimedMutexLockResult;

/// An optional value of type `NSTDTimedMutexLockResult`.
///
/// This type is returned from `nstd_timed_mutex_try_lock` where the uninitialized variant means
/// that the function would block.
NSTDOptional(NSTDTimedMutexLockResult) NSTDOptionalTimedMutexLockResult;

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
NSTDAPI NSTDTimedMutex nstd_timed_mutex_new(NSTDHeapPtr data);

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
NSTDAPI NSTDBool nstd_timed_mutex_is_poisoned(const NSTDTimedMutex *mutex);

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
NSTDAPI NSTDTimedMutexLockResult nstd_timed_mutex_lock(const NSTDTimedMutex *mutex);

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
NSTDAPI NSTDOptionalTimedMutexLockResult nstd_timed_mutex_try_lock(const NSTDTimedMutex *mutex);

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
NSTDAPI NSTDOptionalTimedMutexLockResult
nstd_timed_mutex_timed_lock(const NSTDTimedMutex *mutex, NSTDFloat64 seconds);

/// Returns an immutable raw pointer to a timed mutex guard's protected data.
///
/// # Parameters:
///
/// - `const NSTDTimedMutexGuard *guard` - The mutex guard.
///
/// # Returns
///
/// `NSTDAny data` - A pointer to the guard's protected data.
NSTDAPI NSTDAny nstd_timed_mutex_get(const NSTDTimedMutexGuard *guard);

/// Returns an raw pointer to a timed mutex guard's protected data.
///
/// # Parameters:
///
/// - `NSTDTimedMutexGuard *guard` - The mutex guard.
///
/// # Returns
///
/// `NSTDAnyMut data` - A pointer to the guard's protected data.
NSTDAPI NSTDAnyMut nstd_timed_mutex_get_mut(NSTDTimedMutexGuard *guard);

/// Unlocks a timed mutex by consuming a mutex guard.
///
/// # Parameters:
///
/// - `NSTDTimedMutexGuard guard` - The mutex guard.
NSTDAPI void nstd_timed_mutex_unlock(NSTDTimedMutexGuard guard);

/// Frees an instance of `NSTDTimedMutex`.
///
/// # Parameters:
///
/// - `NSTDTimedMutex mutex` - The timed mutex to free.
///
/// # Panics
///
/// This operation will panic if freeing the timed mutex's data fails.
NSTDAPI void nstd_timed_mutex_free(NSTDTimedMutex mutex);

#endif
