#include <nstd/core/core.h>
#include <nstd/core/optional.h>
#include <nstd/core/result.h>
#include <nstd/core/str.h>
#include <nstd/heap_ptr.h>
#include <nstd/nstd.h>
#include <nstd/thread.h>
#include <nstd/timed_mutex.h>
#include <chrono>
#include <mutex>

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
NSTDAPI NSTDTimedMutex nstd_timed_mutex_new(const NSTDHeapPtr data)
{
    try
    {
        const std::timed_mutex *const mutex{new std::timed_mutex{}};
        return {.inner = (NSTDAnyMut)mutex, .data = data, .poisoned = NSTD_FALSE};
    }
    catch (...)
    {
        const NSTDStr msg{nstd_core_str_from_raw_cstr("failed to create a timed mutex")};
        nstd_core_panic_with_msg(&msg);
    }
    // Unreachable.
    return {};
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
NSTDAPI NSTDBool nstd_timed_mutex_is_poisoned(const NSTDTimedMutex *const mutex)
{
    return mutex->poisoned;
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
NSTDAPI NSTDTimedMutexLockResult nstd_timed_mutex_lock(const NSTDTimedMutex *const mutex)
{
    try
    {
        ((std::timed_mutex *)mutex->inner)->lock();
        if (mutex->poisoned)
            return NSTDTimedMutexLockResult{
                .status = NSTD_RESULT_STATUS_ERR,
                .err = NSTDTimedMutexGuard{.mutex = mutex}
            };
        else
            return NSTDTimedMutexLockResult{
                .status = NSTD_RESULT_STATUS_OK,
                .ok = NSTDTimedMutexGuard{.mutex = mutex}
            };
    }
    catch (...)
    {
        const NSTDStr msg{nstd_core_str_from_raw_cstr("failed to lock a timed mutex")};
        nstd_core_panic_with_msg(&msg);
    }
    // Unreachable.
    return {};
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
NSTDAPI NSTDOptionalTimedMutexLockResult nstd_timed_mutex_try_lock(
const NSTDTimedMutex *const mutex)
{
    if (((std::timed_mutex *)mutex->inner)->try_lock())
    {
        NSTDOptionalTimedMutexLockResult ret{.status = NSTD_OPTIONAL_STATUS_SOME, {}};
        if (mutex->poisoned)
            ret.some = NSTDTimedMutexLockResult{
                .status = NSTD_RESULT_STATUS_ERR,
                .err = NSTDTimedMutexGuard{.mutex = mutex}
            };
        else
            ret.some = NSTDTimedMutexLockResult{
                .status = NSTD_RESULT_STATUS_OK,
                .ok = NSTDTimedMutexGuard{.mutex = mutex}
            };
        return ret;
    }
    else
        return NSTDOptionalTimedMutexLockResult{.status = NSTD_OPTIONAL_STATUS_NONE, {}};
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
/// - `NSTDFloat64 seconds` - The number of seconds to block for.
///
/// # Returns
///
/// `NSTDOptionalTimedMutexLockResult guard` - A handle to the mutex's protected data.
///
/// # Safety
///
/// The mutex lock must not already be owned by the calling thread.
NSTDAPI NSTDOptionalTimedMutexLockResult nstd_timed_mutex_timed_lock(
const NSTDTimedMutex *const mutex, const NSTDFloat64 seconds)
{
    const std::chrono::duration<NSTDFloat64> duration{seconds};
    if (((std::timed_mutex *)mutex->inner)->try_lock_for(duration))
    {
        NSTDOptionalTimedMutexLockResult ret{.status = NSTD_OPTIONAL_STATUS_SOME, {}};
        if (mutex->poisoned)
            ret.some = NSTDTimedMutexLockResult{
                .status = NSTD_RESULT_STATUS_ERR,
                .err = NSTDTimedMutexGuard{.mutex = mutex}
            };
        else
            ret.some = NSTDTimedMutexLockResult{
                .status = NSTD_RESULT_STATUS_OK,
                .ok = NSTDTimedMutexGuard{.mutex = mutex}
            };
        return ret;
    }
    else
        return NSTDOptionalTimedMutexLockResult{.status = NSTD_OPTIONAL_STATUS_NONE, {}};
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
NSTDAPI NSTDAny nstd_timed_mutex_get(const NSTDTimedMutexGuard *const guard)
{
    return nstd_heap_ptr_get(&guard->mutex->data);
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
NSTDAPI NSTDAnyMut nstd_timed_mutex_get_mut(NSTDTimedMutexGuard *const guard)
{
    return nstd_heap_ptr_get_mut(const_cast<NSTDHeapPtr *>(&guard->mutex->data));
}

/// Unlocks a timed mutex by consuming a mutex guard.
///
/// # Parameters:
///
/// - `NSTDTimedMutexGuard guard` - The mutex guard.
NSTDAPI void nstd_timed_mutex_unlock(const NSTDTimedMutexGuard guard)
{
    if (nstd_thread_is_panicking())
        const_cast<NSTDTimedMutex *>(guard.mutex)->poisoned = NSTD_TRUE;
    ((std::timed_mutex *)guard.mutex->inner)->unlock();
}

/// Frees an instance of `NSTDTimedMutex`.
///
/// # Parameters:
///
/// - `NSTDTimedMutex mutex` - The timed mutex to free.
///
/// # Panics
///
/// This operation will panic if freeing the timed mutex or it's data fails.
NSTDAPI void nstd_timed_mutex_free(const NSTDTimedMutex mutex)
{
    try
    {
        delete (std::timed_mutex *)mutex.inner;
        nstd_heap_ptr_free(mutex.data);
    }
    catch (...)
    {
        const NSTDStr msg{nstd_core_str_from_raw_cstr("failed to free a timed mutex")};
        nstd_core_panic_with_msg(&msg);
    }
}
