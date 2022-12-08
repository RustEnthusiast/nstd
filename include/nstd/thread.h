#ifndef NSTD_THREAD_H
#define NSTD_THREAD_H
#include "core/def.h"
#include "core/optional.h"
#include "core/result.h"
#include "core/str.h"
#include "heap_ptr.h"
#include "io/io.h"
#include "nstd.h"

/// Represents a running thread.
typedef NSTDAnyMut NSTDThread;

/// A handle to a running thread.
typedef NSTDAnyMut NSTDThreadHandle;

/// A thread's unique identifier.
typedef NSTDAnyMut NSTDThreadID;

/// Describes the creation of a new thread.
///
/// This type is passed to the `nstd_thread_spawn_with_desc` function.
typedef struct {
    /// The name of the thread.
    ///
    /// If present, this must not contain any null bytes.
    NSTDOptionalStr name;
    /// The number of bytes that the thread's stack should have.
    ///
    /// Set this to 0 to let the host decide how much stack memory should be allocated.
    NSTDUInt stack_size;
} NSTDThreadDescriptor;

/// Returned from `nstd_thread_join`, contains the thread function's return value on success.
NSTDOptional(NSTDErrorCode) NSTDOptionalThreadResult;

/// Returned from `nstd_thread_count`, contains the number of threads detected on the system on
/// success.
NSTDResult(NSTDUInt, NSTDIOError) NSTDThreadCountResult;

/// Spawns a new thread and returns a handle to it.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDHeapPtr)` - The thread function.
///
/// - `NSTDHeapPtr data` - Data to pass to the thread.
///
/// # Returns
///
/// `NSTDThread thread` - A handle to the new thread, null on error.
///
/// # Safety
///
/// - The caller of this function must guarantee that `thread_fn` is a valid function pointer.
///
/// - The data type that `data` holds must be able to be safely sent between threads.
NSTDAPI NSTDThread nstd_thread_spawn(NSTDErrorCode (*thread_fn)(NSTDHeapPtr), NSTDHeapPtr data);

/// Spawns a new thread configured with a descriptor.
///
/// # Parameters:
///
/// - `NSTDErrorCode (*thread_fn)(NSTDHeapPtr)` - The thread function.
///
/// - `NSTDHeapPtr data` - Data to pass to the thread.
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
///
/// - The data type that `data` holds must be able to be safely sent between threads.
NSTDAPI NSTDThread nstd_thread_spawn_with_desc(NSTDErrorCode (*thread_fn)(NSTDHeapPtr),
NSTDHeapPtr data, const NSTDThreadDescriptor *desc);

/// Returns a handle to the calling thread.
///
/// # Returns
///
/// `NSTDThreadHandle handle` - A handle to the current thread.
NSTDAPI NSTDThreadHandle nstd_thread_current();

/// Retrieves a raw handle to a thread.
///
/// # Parameters:
///
/// - `const NSTDThread *thread` - A handle to the thread.
///
/// # Returns
///
/// `NSTDThreadHandle handle` - A raw handle to the thread.
NSTDAPI NSTDThreadHandle nstd_thread_handle(const NSTDThread *thread);

/// Checks if a thread has finished running.
///
/// # Parameters:
///
/// - `const NSTDThread *thread` - A handle to the thread.
///
/// # Returns
///
/// `NSTDBool is_finished` - True if the thread associated with the handle has finished executing.
NSTDAPI NSTDBool nstd_thread_is_finished(const NSTDThread *thread);

/// Joins a thread by it's handle.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
///
/// # Returns
///
/// `NSTDOptionalThreadResult errc` - The thread function's return code, or none if joining the
/// thread fails.
NSTDAPI NSTDOptionalThreadResult nstd_thread_join(NSTDThread thread);

/// Detaches a thread from it's handle, allowing it to run in the background.
///
/// # Parameters:
///
/// - `NSTDThread thread` - The thread handle.
NSTDAPI void nstd_thread_detach(NSTDThread thread);

/// Returns the name of a thread.
///
/// # Parameters:
///
/// - `const NSTDThreadHandle *handle` - A handle to the thread.
///
/// # Returns
///
/// `NSTDOptionalStr name` - The name of the thread, or none if the thread is unnamed.
NSTDAPI NSTDOptionalStr nstd_thread_name(const NSTDThreadHandle *handle);

/// Returns a thread's unique identifier.
///
/// # Parameters:
///
/// - `const NSTDThreadHandle *handle` - A handle to the thread.
///
/// # Returns
///
/// `NSTDThreadID id` - The thread's unique ID.
NSTDAPI NSTDThreadID nstd_thread_id(const NSTDThreadHandle *handle);

/// Frees an instance of `NSTDThreadHandle`.
///
/// # Parameters:
///
/// - `NSTDThreadHandle handle` - The handle to free.
NSTDAPI void nstd_thread_handle_free(NSTDThreadHandle handle);

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

/// Returns the number of recommended threads that a program should use.
///
/// # Returns
///
/// `NSTDThreadCountResult threads` - The estimated default amount of parallelism a program should
/// use on success, or the I/O error code on failure.
NSTDAPI NSTDThreadCountResult nstd_thread_count();

/// Compares two thread identifiers.
///
/// # Parameters:
///
/// - `const NSTDThreadID *xid` - The first identifier.
///
/// - `const NSTDThreadID *yid` - The second identifier.
///
/// # Returns
///
/// `NSTDBool is_eq` - True if the two identifiers refer to the same thread.
NSTDAPI NSTDBool nstd_thread_id_compare(const NSTDThreadID *xid, const NSTDThreadID *yid);

/// Frees an instance of `NSTDThreadID`.
///
/// # Parameters:
///
/// - `NSTDThreadID id` - A thread identifier.
NSTDAPI void nstd_thread_id_free(NSTDThreadID id);

#endif
