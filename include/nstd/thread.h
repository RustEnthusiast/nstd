#ifndef NSTD_THREAD_H
#define NSTD_THREAD_H
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// A handle to a running thread.
typedef NSTDAnyMut NSTDThread;

/// Spawns a new thread and returns a handle to it.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDAnyMut)` - The thread function.
///
/// - `NSTDAnyMut data` - Data to pass to the thread. This will only be passed to `thread_fn` on
/// platforms that support atomic pointers, on other platforms `NSTD_NULL` will be passed.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread.
NSTDAPI NSTDThread nstd_thread_spawn(NSTDErrorCode (*thread_fn)(NSTDAnyMut), NSTDAnyMut data);

/// Puts the current thread to sleep for a specified number of seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 secs` - The number of seconds to put the thread to sleep for.
NSTDAPI void nstd_thread_sleep(NSTDFloat64 secs);

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
///
/// # Returns
///
/// `NSTDErrorCode errc` - The thread functions return code. This can also be non-zero if joining
/// the thread fails.
NSTDAPI NSTDErrorCode nstd_thread_join(NSTDThread thread);

/// Detaches a thread from it's handle, allowing it to run in the background.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
NSTDAPI void nstd_thread_detach(NSTDThread thread);

NSTDCPPEND
#endif
