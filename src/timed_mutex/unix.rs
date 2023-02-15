//! Contains Unix specific definitions for `NSTDTimedMutex`.
use crate::os::unix::mutex::{
    NSTDUnixMutex, NSTDUnixMutexGuard, NSTDUnixMutexLockResult, NSTDUnixOptionalMutexLockResult,
};

/// A mutual exclusion primitive with a timed locking mechanism.
pub type NSTDTimedMutex = NSTDUnixMutex;

/// A handle to a timed mutex's data.
pub type NSTDTimedMutexGuard<'a> = NSTDUnixMutexGuard<'a>;

/// A result type containing a timed mutex lock whether or not the mutex is poisoned.
pub type NSTDTimedMutexLockResult<'a> = NSTDUnixMutexLockResult<'a>;

/// An optional value of type `NSTDTimedMutexLockResult`.
///
/// This type is returned from `nstd_timed_mutex_try_lock` where the uninitialized variant means
/// that the function would block.
pub type NSTDOptionalTimedMutexLockResult<'a> = NSTDUnixOptionalMutexLockResult<'a>;
