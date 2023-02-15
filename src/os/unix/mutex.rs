//! A mutual exclusion primitive useful for protecting shared data.
use crate::{
    core::{optional::NSTDOptional, result::NSTDResult},
    heap_ptr::{nstd_heap_ptr_get, nstd_heap_ptr_get_mut, NSTDHeapPtr},
    time::NSTDDuration,
    NSTDAny, NSTDAnyMut, NSTDBool, NSTD_FALSE, NSTD_TRUE,
};
use cfg_if::cfg_if;
use libc::{
    pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_trylock, pthread_mutex_unlock, pthread_mutexattr_destroy, pthread_mutexattr_init,
    pthread_mutexattr_settype, pthread_mutexattr_t, EBUSY, PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_NORMAL,
};
use nstdapi::nstdapi;
use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    mem::MaybeUninit,
};

/// A raw mutex wrapping `pthread_mutex_t`.
///
/// This type has the same in-memory representation as `pthread_mutex_t`.
#[repr(transparent)]
struct RawMutex(UnsafeCell<pthread_mutex_t>);
impl Drop for RawMutex {
    /// [RawMutex]'s destructor.
    fn drop(&mut self) {
        // SAFETY: Destroying a locked mutex results in undefined behavior, so here we check if the
        // mutex is locked. If the mutex *is* locked then it's guard must have been leaked, in this
        // case we will leak the raw mutex data as well.
        unsafe {
            if pthread_mutex_trylock(self.0.get()) == 0 {
                // This shall only fail if the mutex is either robust,
                // `PTHREAD_MUTEX_ERRORCHECK`, or `PTHREAD_MUTEX_RECURSIVE` and the thread does
                // not own the mutex.
                pthread_mutex_unlock(self.0.get());
                pthread_mutex_destroy(self.0.get());
            }
        }
    }
}

/// A mutex attribute builder.
struct MutexAttrs(pthread_mutexattr_t);
impl MutexAttrs {
    /// Creates a new instance of [MutexAttrs].
    ///
    /// # Panics
    ///
    /// This operation will panic if creating the mutex attributes object fails.
    fn new() -> Self {
        let mut attr = MaybeUninit::uninit();
        // SAFETY: All operations are thread-safe, errors are checked.
        unsafe {
            assert!(pthread_mutexattr_init(attr.as_mut_ptr()) == 0);
            // This shall never fail, PTHREAD_MUTEX_NORMAL is a valid type.
            pthread_mutexattr_settype(attr.as_mut_ptr(), PTHREAD_MUTEX_NORMAL);
            Self(attr.assume_init())
        }
    }
}
impl Drop for MutexAttrs {
    /// [MutexAttrs] destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Rust's type system will ensure that `Self` is properly initialized.
        unsafe { pthread_mutexattr_destroy(&mut self.0) };
    }
}

/// A mutual exclusion primitive useful for protecting shared data.
#[nstdapi]
pub struct NSTDUnixMutex {
    /// The underlying mutex.
    inner: RawMutex,
    /// The protected data.
    data: UnsafeCell<NSTDHeapPtr>,
    /// Determines whether or not the mutex is poisoned.
    poisoned: Cell<NSTDBool>,
}
/// # Safety
///
/// The data that the mutex is protecting must be able to be safely sent between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Send for NSTDUnixMutex {}
/// # Safety
///
/// The data that the mutex is protecting must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDUnixMutex {}

/// A handle to a mutex's protected data.
#[nstdapi]
pub struct NSTDUnixMutexGuard<'a> {
    /// A reference to the mutex.
    mutex: &'a NSTDUnixMutex,
    /// Ensures that the guard is not [Send].
    pd: PhantomData<*const ()>,
}
impl<'a> NSTDUnixMutexGuard<'a> {
    /// Constructs a new mutex guard.
    #[inline]
    fn new(mutex: &'a NSTDUnixMutex) -> Self {
        Self {
            mutex,
            pd: Default::default(),
        }
    }
}
impl Drop for NSTDUnixMutexGuard<'_> {
    /// Drops the guard, releasing the lock for the mutex.
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.mutex.poisoned.set(NSTD_TRUE);
        }
        // SAFETY: `self` has a valid reference to the mutex.
        // This shall only fail if the mutex is either robust, `PTHREAD_MUTEX_ERRORCHECK`, or
        // `PTHREAD_MUTEX_RECURSIVE` and the thread does not own the mutex.
        unsafe { pthread_mutex_unlock(self.mutex.inner.0.get()) };
    }
}
/// # Safety
///
/// The data that the guard is protecting must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDUnixMutexGuard<'_> {}

/// A result type returned from `nstd_os_unix_mutex_lock` containing the mutex guard whether or not
/// the data is poisoned.
pub type NSTDUnixMutexLockResult<'a> = NSTDResult<NSTDUnixMutexGuard<'a>, NSTDUnixMutexGuard<'a>>;

/// An optional value of type `NSTDUnixMutexLockResult`.
///
/// This type is returned from the `nstd_os_unix_mutex_try_lock` where the uninitialized variant
/// means that the function would block.
pub type NSTDUnixOptionalMutexLockResult<'a> = NSTDOptional<NSTDUnixMutexLockResult<'a>>;

/// Creates a new mutex in an unlocked state.
///
/// # Parameters:
///
/// - `NSTDHeapPtr data` - The data to be protected by the mutex.
///
/// # Returns
///
/// `NSTDUnixMutex mutex` - The new initialized mutex.
///
/// # Panics
///
/// This operation will panic if creating the mutex fails.
#[nstdapi]
pub fn nstd_os_unix_mutex_new(data: NSTDHeapPtr) -> NSTDUnixMutex {
    let mutex = RawMutex(UnsafeCell::new(PTHREAD_MUTEX_INITIALIZER));
    let attrs = MutexAttrs::new();
    // SAFETY: `attrs` is properly initialized.
    unsafe { assert!(pthread_mutex_init(mutex.0.get(), &attrs.0) == 0) };
    NSTDUnixMutex {
        inner: mutex,
        data: UnsafeCell::new(data),
        poisoned: Cell::new(NSTD_FALSE),
    }
}

/// Returns a Unix mutex's native OS handle.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex.
///
/// # Returns
///
/// `pthread_mutex_t raw` - The native mutex handle.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_mutex_handle(mutex: &NSTDUnixMutex) -> pthread_mutex_t {
    // SAFETY: `mutex` is behind an initialized reference.
    unsafe { *mutex.inner.0.get() }
}

/// Determines whether or not a mutex's data is poisoned.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to check.
///
/// # Returns
///
/// `NSTDBool is_poisoned` - `NSTD_TRUE` if the mutex's data is poisoned.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_mutex_is_poisoned(mutex: &NSTDUnixMutex) -> NSTDBool {
    mutex.poisoned.get()
}

/// Waits for a mutex lock to become acquired, returning a guard wrapping the protected data.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to lock.
///
/// # Returns
///
/// `NSTDUnixMutexLockResult guard` - A handle to the mutex's protected data.
///
/// # Panics
///
/// This operation will panic if locking the mutex fails.
#[nstdapi]
pub fn nstd_os_unix_mutex_lock(mutex: &NSTDUnixMutex) -> NSTDUnixMutexLockResult {
    // SAFETY: `mutex` is behind an initialized reference.
    unsafe { assert!(pthread_mutex_lock(mutex.inner.0.get()) == 0) };
    let guard = NSTDUnixMutexGuard::new(mutex);
    match mutex.poisoned.get() {
        true => NSTDResult::Err(guard),
        false => NSTDResult::Ok(guard),
    }
}

/// The non-blocking variant of `nstd_os_unix_mutex_lock`. This will return immediately with an
/// uninitialized "none" value if the mutex is locked.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to lock.
///
/// # Returns
///
/// `NSTDUnixOptionalMutexLockResult guard` - A handle to the mutex's data, or "none" if the mutex
/// is locked.
///
/// # Panics
///
/// This operation will panic if locking the mutex fails.
#[nstdapi]
pub fn nstd_os_unix_mutex_try_lock(mutex: &NSTDUnixMutex) -> NSTDUnixOptionalMutexLockResult {
    // SAFETY: `mutex` is behind an initialized reference.
    match unsafe { pthread_mutex_trylock(mutex.inner.0.get()) } {
        0 => {
            let guard = NSTDUnixMutexGuard::new(mutex);
            NSTDOptional::Some(match mutex.poisoned.get() {
                true => NSTDResult::Err(guard),
                false => NSTDResult::Ok(guard),
            })
        }
        err => {
            assert!(err == EBUSY);
            NSTDOptional::None
        }
    }
}

/// The timed variant of `nstd_os_unix_mutex_lock`. This will return with an uninitialized "none"
/// value if the mutex remains locked for the time span of `duration`.
///
/// # Note
///
/// This function will return immediately with a "none" value on unsupported platforms.
/// Supported platforms include Android, DragonFly BSD, FreeBSD, NetBSD, OpenBSD, Haiku, illumos,
/// Linux, QNX Neutrino, and Oracle Solaris.
///
/// # Parameters:
///
/// - `const NSTDUnixMutex *mutex` - The mutex to lock.
///
/// - `const NSTDDuration *duration` - The amount of time to wait for the mutex to become available.
///
/// # Returns
///
/// `NSTDUnixOptionalMutexLockResult guard` - A handle to the mutex's data, or "none" if the mutex
/// remains locked for the time span of `duration`.
///
/// # Panics
///
/// This operation will panic if locking the mutex fails.
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_os_unix_mutex_timed_lock<'a>(
    mutex: &'a NSTDUnixMutex,
    duration: &'_ NSTDDuration,
) -> NSTDUnixOptionalMutexLockResult<'a> {
    cfg_if! {
        if #[cfg(any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "haiku",
            target_os = "illumos",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "nto",
            target_os = "openbsd",
            target_os = "solaris"
        ))] {
            use crate::time::{
                nstd_time_duration_nanoseconds, nstd_time_duration_seconds, nstd_time_nanoseconds,
                nstd_time_seconds, NSTDTime,
            };
            use libc::{pthread_mutex_timedlock, timespec, ETIMEDOUT};
            use std::time::{Duration, SystemTime};
            let mut time = SystemTime::now();
            time += Duration::new(
                nstd_time_duration_seconds(&duration),
                nstd_time_duration_nanoseconds(&duration),
            );
            let time = NSTDTime::from(time);
            let duration = timespec {
                tv_sec: nstd_time_seconds(&time) as _,
                tv_nsec: nstd_time_nanoseconds(&time) as _,
            };
            // SAFETY: `mutex` is behind an initialized reference.
            match unsafe { pthread_mutex_timedlock(mutex.inner.0.get(), &duration) } {
                0 => {
                    let guard = NSTDUnixMutexGuard::new(mutex);
                    NSTDOptional::Some(match mutex.poisoned.get() {
                        true => NSTDResult::Err(guard),
                        false => NSTDResult::Ok(guard),
                    })
                }
                err => {
                    assert!(err == ETIMEDOUT);
                    NSTDOptional::None
                }
            }
        } else {
            return NSTDOptional::None;
        }
    }
}

/// Returns a pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `const NSTDUnixMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAny data` - A pointer to the mutex's data.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_mutex_get(guard: &NSTDUnixMutexGuard) -> NSTDAny {
    // SAFETY: `mutex` is behind a valid reference.
    nstd_heap_ptr_get(unsafe { &*guard.mutex.data.get() })
}

/// Returns a mutable pointer to a mutex's raw data.
///
/// # Parameters:
///
/// - `NSTDUnixMutexGuard *guard` - A handle to the mutex's protected data.
///
/// # Returns
///
/// `NSTDAnyMut data` - A pointer to the mutex's data.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_mutex_get_mut(guard: &mut NSTDUnixMutexGuard) -> NSTDAnyMut {
    // SAFETY: `mutex` is behind a valid reference.
    nstd_heap_ptr_get_mut(unsafe { &mut *guard.mutex.data.get() })
}

/// Unlocks a mutex by consuming it's guard.
///
/// # Parameters:
///
/// - `NSTDUnixMutexGuard guard` - The mutex guard to take ownership of.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_os_unix_mutex_unlock(guard: NSTDUnixMutexGuard) {}

/// Frees a mutex and the data it is protecting.
///
/// # Parameters:
///
/// - `NSTDUnixMutex mutex` - The mutex to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_os_unix_mutex_free(mutex: NSTDUnixMutex) {}
