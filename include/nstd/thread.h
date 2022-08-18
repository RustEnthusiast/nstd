#ifndef NSTD_THREAD_H
#define NSTD_THREAD_H
#include "core/def.h"
#include "core/str.h"
#include "nstd.h"
NSTDCPPSTART

/// A handle to a running thread.
typedef NSTDAnyMut NSTDThread;

/// Describes the creation of a new thread.
///
/// This type is passed to the `nstd_thread_spawn_with_desc` function.
typedef struct {
    /// The name of the thread.
    NSTDStrConst name;
    /// A pointer to the data to be passed to the thread.
    ///
    /// # Note
    ///
    /// This will only be passed to the thread function on platforms that support atomic pointers,
    /// on other platforms `NSTD_NULL` will be passed.
    NSTDAnyMut data;
    /// The number of bytes that the thread's stack should have.
    NSTDUSize stack_size;
} NSTDThreadDescriptor;

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
///
/// # Safety
///
/// The caller of this function must guarantee that `thread_fn` is a valid function pointer.
NSTDAPI NSTDThread nstd_thread_spawn(NSTDErrorCode (*thread_fn)(NSTDAnyMut), NSTDAnyMut data);

/// Spawns a new thread configured with a descriptor.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDAnyMut)` - The thread function.
///
/// - `const NSTDThreadDescriptor *desc` - The thread descriptor.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread.
///
/// # Safety
///
/// This operation can cause undefined behavior if `desc`'s data is invalid.
NSTDAPI NSTDThread nstd_thread_spawn_with_desc(NSTDErrorCode (*thread_fn)(NSTDAnyMut),
const NSTDThreadDescriptor *desc);

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
