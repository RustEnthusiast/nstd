#ifndef NSTD_MUTEX_H
#define NSTD_MUTEX_H
#include "core/optional.h"
#include "core/result.h"
#include "heap_ptr.h"
#include "nstd.h"

/// A mutual exclusion primitive useful for protecting shared data.
typedef struct {
    /// The Rust [Mutex].
    NSTDAnyMut mtx;
} NSTDMutex;

/// Represents an optional value of type `NSTDMutex`.
NSTDOptional(NSTDMutex) NSTDOptionalMutex;

/// A guard providing access to a mutex's protected data.
typedef struct {
    /// The Rust [MutexGuard].
    NSTDAnyMut guard;
} NSTDMutexGuard;

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
/// `NSTDOptionalMutex mutex` - The new mutex protecting `data` on success, or an uninitialized
/// "none" variant on error.
NSTDAPI NSTDOptionalMutex nstd_mutex_new(NSTDHeapPtr data);

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
NSTDAPI NSTDBool nstd_mutex_is_poisoned(const NSTDMutex *mutex);

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
NSTDAPI NSTDOptionalMutexLockResult nstd_mutex_lock(const NSTDMutex *mutex);

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
NSTDAPI NSTDOptionalHeapPtr nstd_mutex_into_inner(NSTDMutex mutex);

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
NSTDAPI void nstd_mutex_drop(NSTDMutex mutex, void (*callback)(NSTDAnyMut));

#endif
