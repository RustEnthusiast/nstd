#ifndef NSTD_CORE_CORE_H
#define NSTD_CORE_CORE_H
#include "../nstd.h"
#include "str.h"

/// Terminates the program immediately in an abnormal fashion.
///
/// This operation will never return.
///
/// # Panics
///
/// This operation will always panic.
NSTDAPI void nstd_core_abort(void);

/// Invokes the runtime's panic handler.
///
/// This operation will never return.
///
/// In contrast to `nstd_core_abort`, which will terminate the program immediately, this method of
/// abortion will begin unwinding the stack (when panic = "unwind"). This can be useful for Rust
/// programs that don't unwind through call frames from foreign languages.
///
/// # Panics
///
/// This function will always panic.
NSTDAPI void nstd_core_panic(void);

/// Invokes the runtime's panic handler with a UTF-8 encoded payload.
///
/// This operation will never return.
///
/// In contrast to `nstd_core_abort_with_msg`, which will terminate the program immediately, this
/// method of abortion will begin unwinding the stack (when panic = "unwind"). This can be useful
/// for Rust programs that don't unwind through call frames from foreign languages.
///
/// # Parameters:
///
/// - `const NSTDStr *msg` - The message to panic with.
///
/// # Panics
///
/// This function will always panic.
///
/// # Safety
///
/// `msg`'s data must be valid for reads.
NSTDAPI void nstd_core_panic_with_msg(const NSTDStr *msg);

#endif
