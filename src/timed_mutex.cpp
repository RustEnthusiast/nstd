#include <nstd/core/optional.h>
#include <nstd/core/result.h>
#include <nstd/core/time.h>
#include <nstd/heap_ptr.h>
#include <nstd/nstd.h>
#include <nstd/thread.h>
#include <nstd/timed_mutex.h>
#include <chrono>
#include <mutex>

#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
#    include <nstd/os/unix/mutex.h>
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
NSTDAPI NSTDOptionalTimedMutex nstd_timed_mutex_new(const NSTDHeapPtr data) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_new(data);
#else
    try {
        const std::timed_mutex *const mutex{new std::timed_mutex{}};
        const NSTDTimedMutex timed_mutex{(NSTDAnyMut)mutex, data, NSTD_FALSE, NSTD_FALSE};
        return NSTDOptionalTimedMutex{NSTD_OPTIONAL_STATUS_SOME, {timed_mutex}};
    } catch (...) {
        return NSTDOptionalTimedMutex{NSTD_OPTIONAL_STATUS_NONE, {}};
    }
#endif
}

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
NSTDAPI NSTDBool nstd_timed_mutex_is_poisoned(const NSTDTimedMutex *const mutex) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_is_poisoned(mutex);
#else
    return mutex->poisoned;
#endif
}

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
NSTDAPI NSTDOptionalTimedMutexLockResult nstd_timed_mutex_lock(const NSTDTimedMutex *const mutex) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_lock(mutex);
#else
    try {
        ((std::timed_mutex *)mutex->inner)->lock();
        const_cast<NSTDTimedMutex *>(mutex)->locked = NSTD_TRUE;
        NSTDOptionalTimedMutexLockResult ret{NSTD_OPTIONAL_STATUS_SOME, {}};
        const NSTDTimedMutexGuard guard{mutex};
        if (mutex->poisoned)
            ret.value.some = NSTDTimedMutexLockResult{NSTD_RESULT_STATUS_ERR, {guard}};
        else
            ret.value.some = NSTDTimedMutexLockResult{NSTD_RESULT_STATUS_OK, {guard}};
        return ret;
    } catch (...) {
        return NSTDOptionalTimedMutexLockResult{NSTD_OPTIONAL_STATUS_NONE, {}};
    }
#endif
}

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
NSTDAPI NSTDOptionalTimedMutexLockResult nstd_timed_mutex_try_lock(const NSTDTimedMutex *const mutex
) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_try_lock(mutex);
#else
    if (((std::timed_mutex *)mutex->inner)->try_lock()) {
        const_cast<NSTDTimedMutex *>(mutex)->locked = NSTD_TRUE;
        NSTDOptionalTimedMutexLockResult ret{NSTD_OPTIONAL_STATUS_SOME, {}};
        const NSTDTimedMutexGuard guard{mutex};
        if (mutex->poisoned)
            ret.value.some = NSTDTimedMutexLockResult{NSTD_RESULT_STATUS_ERR, {guard}};
        else
            ret.value.some = NSTDTimedMutexLockResult{NSTD_RESULT_STATUS_OK, {guard}};
        return ret;
    } else
        return NSTDOptionalTimedMutexLockResult{NSTD_OPTIONAL_STATUS_NONE, {}};
#endif
}

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
NSTDAPI NSTDOptionalTimedMutexLockResult
nstd_timed_mutex_timed_lock(const NSTDTimedMutex *const mutex, const NSTDDuration duration) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_timed_lock(mutex, duration);
#else
    const std::chrono::duration<NSTDFloat64> dur{nstd_core_time_duration_get(duration)};
    if (((std::timed_mutex *)mutex->inner)->try_lock_for(dur)) {
        const_cast<NSTDTimedMutex *>(mutex)->locked = NSTD_TRUE;
        NSTDOptionalTimedMutexLockResult ret{NSTD_OPTIONAL_STATUS_SOME, {}};
        const NSTDTimedMutexGuard guard{mutex};
        if (mutex->poisoned)
            ret.value.some = NSTDTimedMutexLockResult{NSTD_RESULT_STATUS_ERR, {guard}};
        else
            ret.value.some = NSTDTimedMutexLockResult{NSTD_RESULT_STATUS_OK, {guard}};
        return ret;
    } else
        return NSTDOptionalTimedMutexLockResult{NSTD_OPTIONAL_STATUS_NONE, {}};
#endif
}

/// Returns an immutable raw pointer to a timed mutex guard's protected data.
///
/// # Parameters:
///
/// - `const NSTDTimedMutexGuard *guard` - The mutex guard.
///
/// # Returns
///
/// `NSTDAny data` - A pointer to the guard's protected data.
NSTDAPI NSTDAny nstd_timed_mutex_get(const NSTDTimedMutexGuard *const guard) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_get(guard);
#else
    return nstd_heap_ptr_get(&guard->mutex->data);
#endif
}

/// Returns an raw pointer to a timed mutex guard's protected data.
///
/// # Parameters:
///
/// - `NSTDTimedMutexGuard *guard` - The mutex guard.
///
/// # Returns
///
/// `NSTDAnyMut data` - A pointer to the guard's protected data.
NSTDAPI NSTDAnyMut nstd_timed_mutex_get_mut(NSTDTimedMutexGuard *const guard) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    return nstd_os_unix_mutex_get_mut(guard);
#else
    return nstd_heap_ptr_get_mut(const_cast<NSTDHeapPtr *>(&guard->mutex->data));
#endif
}

/// Unlocks a timed mutex by consuming a mutex guard.
///
/// # Parameters:
///
/// - `NSTDTimedMutexGuard guard` - The mutex guard.
NSTDAPI void nstd_timed_mutex_unlock(const NSTDTimedMutexGuard guard) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    nstd_os_unix_mutex_unlock(guard);
#else
    NSTDTimedMutex *const mutex{const_cast<NSTDTimedMutex *>(guard.mutex)};
    if (nstd_thread_is_panicking())
        mutex->poisoned = NSTD_TRUE;
    mutex->locked = NSTD_FALSE;
    ((std::timed_mutex *)mutex->inner)->unlock();
#endif
}

/// Frees an instance of `NSTDTimedMutex`.
///
/// # Parameters:
///
/// - `NSTDTimedMutex mutex` - The timed mutex to free.
NSTDAPI void nstd_timed_mutex_free(const NSTDTimedMutex mutex) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    nstd_os_unix_mutex_free(mutex);
#else
    // Destroying a locked mutex results in undefined behavior, so here we check if the mutex is
    // locked. If the mutex *is* locked then it's guard must have been leaked, in this case we will
    // leak the raw mutex as well.
    if (!mutex.locked)
        delete (std::timed_mutex *)mutex.inner;
    nstd_heap_ptr_free(mutex.data);
#endif
}

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
NSTDAPI void nstd_timed_mutex_drop(const NSTDTimedMutex mutex, void (*const callback)(NSTDAnyMut)) {
#ifdef NSTD_TIMED_MUTEX_OS_UNIX_IMPL
    nstd_os_unix_mutex_drop(mutex, callback);
#else
    // Destroying a locked mutex results in undefined behavior, so here we check if the mutex is
    // locked. If the mutex *is* locked then it's guard must have been leaked, in this case we will
    // leak the raw mutex as well.
    if (!mutex.locked)
        delete (std::timed_mutex *)mutex.inner;
    // Accessing the data through `callback` may result in undefined behavior if this mutex is
    // poisoned.
    if (!mutex.poisoned)
        nstd_heap_ptr_drop(mutex.data, callback);
#endif
}
