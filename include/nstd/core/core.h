#ifndef NSTD_CORE_CORE_H
#define NSTD_CORE_CORE_H
#include "../nstd.h"

/// Invokes the runtime's panic handler.
///
/// This operation will never return.
///
/// # Panics
///
/// This function will always panic.
NSTDAPI void nstd_core_panic();

#endif
