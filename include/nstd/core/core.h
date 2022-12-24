#ifndef NSTD_CORE_CORE_H
#define NSTD_CORE_CORE_H
#include "../nstd.h"
#include "str.h"

/// Invokes the runtime's panic handler.
///
/// This operation will never return.
///
/// # Panics
///
/// This function will always panic.
NSTDAPI void nstd_core_panic();

/// Invokes the runtime's panic handler with a UTF-8 encoded payload.
///
/// This operation will never return.
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
