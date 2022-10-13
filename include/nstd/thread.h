#ifndef NSTD_THREAD_H
#define NSTD_THREAD_H
#include "core/def.h"
#include "core/str.h"
#include "nstd.h"

/// A handle to a running thread.
typedef NSTDAnyMut NSTDThread;

/// Describes the creation of a new thread.
///
/// This type is passed to the `nstd_thread_spawn_with_desc` function.
typedef struct {
    /// The name of the thread.
    NSTDStr name;
    /// The number of bytes that the thread's stack should have.
    ///
    /// Set this to 0 to let the host decide how much stack memory should be allocated.
    NSTDUInt stack_size;
} NSTDThreadDescriptor;

/// Spawns a new thread and returns a handle to it.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)()` - The thread function.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread, null on error.
///
/// # Safety
///
/// The caller of this function must guarantee that `thread_fn` is a valid function pointer.
NSTDAPI NSTDThread nstd_thread_spawn(NSTDErrorCode (*thread_fn)());

/// Spawns a new thread configured with a descriptor.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)()` - The thread function.
///
/// - `const NSTDThreadDescriptor *desc` - The thread descriptor.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread, null on error.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `desc.name` contains null bytes.
///
/// - `desc.name`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// - The caller of this function must guarantee that `thread_fn` is a valid function pointer.
///
/// - This operation can cause undefined behavior if `desc`'s data is invalid.
NSTDAPI NSTDThread nstd_thread_spawn_with_desc(NSTDErrorCode (*thread_fn)(),
const NSTDThreadDescriptor *desc);

/// Puts the current thread to sleep for a specified number of seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 secs` - The number of seconds to put the thread to sleep for.
///
/// # Panics
///
/// Panics if `secs` is negative, overflows Rust's `Duration` structure, or is non-finite.
NSTDAPI void nstd_thread_sleep(NSTDFloat64 secs);

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
///
/// # Returns
///
/// `NSTDErrorCode errc` - The thread function's return code.
///
/// # Panics
///
/// Panics if joining the thread fails.
NSTDAPI NSTDErrorCode nstd_thread_join(NSTDThread thread);

/// Detaches a thread from it's handle, allowing it to run in the background.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
NSTDAPI void nstd_thread_detach(NSTDThread thread);

#endif
