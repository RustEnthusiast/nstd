#ifndef NSTD_TIMED_MUTEX_H
#define NSTD_TIMED_MUTEX_H
#include "core/optional.h"
#include "core/result.h"
#include "heap_ptr.h"
#include "nstd.h"
#include "os/os.h"
#include "time.h"

#if defined(NSTD_OS_ANDROID) || defined(NSTD_OS_DRAGONFLY) || defined(NSTD_OS_FREEBSD) \
    || defined(NSTD_OS_HAIKU) || defined(NSTD_OS_LINUX) || defined(NSTD_OS_NETBSD)     \
    || defined(NSTD_OS_NTO) || defined(NSTD_OS_OPENBSD) || defined(NSTD_OS_SOLARIS)
#    define NSTD_TIMED_MUTEX_OS_UNIX_IMPL
#endif

#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
#    include "os/unix/mutex.h"
#endif

/// A mutual exclusion primitive with a timed locking mechanism.
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
typedef NSTDUnixMutex NSTDTimedMutex;
#else
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
#endif

/// Represents an optional value of type `NSTDTimedMutex`.
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
typedef NSTDUnixOptionalMutex NSTDOptionalTimedMutex;
#else
NSTDOptional(NSTDTimedMutex) NSTDOptionalTimedMutex;
#endif

/// A handle to a timed mutex's data.
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
typedef NSTDUnixMutexGuard NSTDTimedMutexGuard;
#else
typedef struct {
    /// A reference to the mutex.
    const NSTDTimedMutex *mutex;
} NSTDTimedMutexGuard;
#endif

/// A result type containing a timed mutex lock whether or not the mutex is poisoned.
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
typedef NSTDUnixMutexLockResult NSTDTimedMutexLockResult;
#else
NSTDResult(NSTDTimedMutexGuard, NSTDTimedMutexGuard) NSTDTimedMutexLockResult;
#endif

/// An optional value of type `NSTDTimedMutexLockResult`.
///
/// This type is returned from `nstd_timed_mutex_try_lock` where the uninitialized variant means
/// that the function would block.
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
typedef NSTDUnixOptionalMutexLockResult NSTDOptionalTimedMutexLockResult;
#else
NSTDOptional(NSTDTimedMutexLockResult) NSTDOptionalTimedMutexLockResult;
#endif

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
NSTDAPI NSTDOptionalTimedMutex nstd_timed_mutex_new(NSTDHeapPtr data);

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
/// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data, or an
/// uninitialized "none" value if the OS fails to lock the mutex.
///
/// # Safety
///
/// The mutex lock must not already be owned by the calling thread.
NSTDAPI NSTDOptionalTimedMutexLockResult nstd_timed_mutex_lock(const NSTDTimedMutex *mutex);

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
/// - `const NSTDDuration *duration` - The amount of time to block for.
///
/// # Returns
///
/// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data.
///
/// # Safety
///
/// The mutex lock must not already be owned by the calling thread.
NSTDAPI NSTDOptionalTimedMutexLockResult
nstd_timed_mutex_timed_lock(const NSTDTimedMutex *mutex, const NSTDDuration *duration);

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
NSTDAPI void nstd_timed_mutex_free(NSTDTimedMutex mutex);

#endif
