#ifndef NSTD_WINDOW_H
#define NSTD_WINDOW_H
#include "app/handle.h"
#include "nstd.h"
NSTDCPPSTART

/// An `nstd` application window.
typedef NSTDAnyMut NSTDWindow;

/// Creates a new window attached to `app`'s event loop.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to an `nstd` application.
///
/// # Returns
///
/// `NSTDWindow window` - A handle to the newly created window.
///
/// # Panics
///
/// This operation will panic if creating the new window fails.
NSTDAPI NSTDWindow nstd_window_new(NSTDAppHandle app);

/// Permanently closes & frees a window and it's data.
///
/// # Parameters:
///
/// - `NSTDWindow window` - The window to close.
NSTDAPI void nstd_window_close(NSTDWindow window);

NSTDCPPEND
#endif
