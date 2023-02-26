#ifndef NSTD_OS_UNIX_MUTEX_H
#define NSTD_OS_UNIX_MUTEX_H
#include "../../core/optional.h"
#include "../../core/result.h"
#include "../../heap_ptr.h"
#include "../../nstd.h"
#include "../../time.h"
#include <pthread.h>

/// A mutual exclusion primitive useful for protecting shared data.
typedef struct {
    /// The underlying mutex.
    pthread_mutex_t inner;
    /// The protected data.
    NSTDHeapPtr data;
    /// Determines whether or not the mutex is poisoned.
    NSTDBool poisoned;
} NSTDUnixMutex;

/// Represents an optional value of type `NSTDUnixMutex`.
NSTDOptional(NSTDUnixMutex) NSTDUnixOptionalMutex;

/// A handle to a mutex's protected data.
typedef struct {
    /// A reference to the mutex.
    const NSTDUnixMutex *mutex;
} NSTDUnixMutexGuard;

/// A result type returned from `nstd_os_unix_mutex_lock` containing the mutex guard whether or not
/// the data is poisoned.
NSTDResult(NSTDUnixMutexGuard, NSTDUnixMutexGuard) NSTDUnixMutexLockResult;

/// An optional value of type `NSTDUnixMutexLockResult`.
///
/// This type is returned from the `nstd_os_unix_mutex_try_lock` where the uninitialized variant
/// means that the function would block.
NSTDOptional(NSTDUnixMutexLockResult) NSTDUnixOptionalMutexLockResult;

/// Creates a new mutex in an unlocked state.
///
/// # Parameters:
///
/// - `NSTDHeapPtr data` - The data to be protected by the mutex.
///
/// # Returns
///
/// `NSTDUnixOptionalMutex mutex` - The new initialized mutex on success, or an uninitialized "none"
/// value if the OS was unable to create and initialize the mutex.
NSTDAPI NSTDUnixOptionalMutex nstd_os_unix_mutex_new(NSTDHeapPtr data);

/// Returns a Unix mutex's native OS handle.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex.
///
/// # Returns
///
/// `pthread_mutex_t raw` - The native mutex handle.
NSTDAPI pthread_mutex_t nstd_os_unix_mutex_handle(const NSTDUnixMutex *mutex);

/// Determines whether or not a mutex's data is poisoned.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to check.
///
/// # Returns
///
/// `NSTDBool is_poisoned` - `NSTD_TRUE` if the mutex's data is poisoned.
NSTDAPI NSTDBool nstd_os_unix_mutex_is_poisoned(const NSTDUnixMutex *mutex);

/// Waits for a mutex lock to become acquired, returning a guard wrapping the protected data.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to lock.
///
/// # Returns
///
/// `NSTDUnixOptionalMutexLockResult guard` - A handle to the mutex's protected data on success, or
/// an uninitialized "none" value if the OS failed to lock the mutex.
NSTDAPI NSTDUnixOptionalMutexLockResult nstd_os_unix_mutex_lock(const NSTDUnixMutex *mutex);

/// The non-blocking variant of `nstd_os_unix_mutex_lock`. This will return immediately with an
/// uninitialized "none" value if the mutex is locked.
///
/// # Note
///
/// This operation may return a "none" value in the case that the OS fails to lock the mutex.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to lock.
///
/// # Returns
///
/// `NSTDUnixOptionalMutexLockResult guard` - A handle to the mutex's data, or "none" if the mutex
/// is locked.
NSTDAPI NSTDUnixOptionalMutexLockResult nstd_os_unix_mutex_try_lock(const NSTDUnixMutex *mutex);

/// The timed variant of `nstd_os_unix_mutex_lock`. This will return with an uninitialized "none"
/// value if the mutex remains locked for the time span of `duration`.
///
/// # Notes
///
/// This operation may return a "none" value in the case that the OS fails to lock the mutex.
///
/// This function will return immediately with a "none" value on unsupported platforms.
/// Supported platforms include Android, DragonFly BSD, FreeBSD, NetBSD, OpenBSD, Haiku, illumos,
/// Linux, QNX Neutrino, and Oracle Solaris.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to lock.
///
/// - `const NSTDDuration *duration` - The amount of time to block for.
///
/// # Returns
///
/// `NSTDUnixOptionalMutexLockResult guard` - A handle to the mutex's data, or "none" if the mutex
/// remains locked for the time span of `duration`.
NSTDAPI NSTDUnixOptionalMutexLockResult
nstd_os_unix_mutex_timed_lock(const NSTDUnixMutex *mutex, const NSTDDuration *duration);

/// Returns a pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `const NSTDUnixMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAny data` - A pointer to the mutex's data.
NSTDAPI NSTDAny nstd_os_unix_mutex_get(const NSTDUnixMutexGuard *guard);

/// Returns a mutable pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `NSTDUnixMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAnyMut data` - A pointer to the mutex's data.
NSTDAPI NSTDAnyMut nstd_os_unix_mutex_get_mut(NSTDUnixMutexGuard *guard);

/// Unlocks a mutex by consuming it's guard.
///
/// # Parameters:
///
/// - `NSTDUnixMutexGuard guard` - The mutex guard to take ownership of.
NSTDAPI void nstd_os_unix_mutex_unlock(NSTDUnixMutexGuard guard);

/// Frees a mutex and the data it is protecting.
///
/// # Parameters:
///
/// - `NSTDUnixMutex mutex` - The mutex to free.
NSTDAPI void nstd_os_unix_mutex_free(NSTDUnixMutex mutex);

#endif
