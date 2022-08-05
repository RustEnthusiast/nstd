#ifndef NSTD_APP_H
#define NSTD_APP_H
#include "nstd.h"
NSTDCPPSTART

/// An application event loop.
typedef struct {
    /// Application data that cannot live on the stack.
    NSTDAnyMut data;
} NSTDApp;

/// Creates a new `nstd` application.
///
/// # Note
///
/// An `NSTDApp` can only be created once on the "main" thread. Attempting to recreate an `NSTDApp`
/// after one has already been created will result in a panic.
///
/// # Returns
///
/// `NSTDApp app` - The new `NSTDApp`.
///
/// # Panics
///
/// This function must be called on the "main" thread, otherwise a panic may occurr.
NSTDAPI NSTDApp nstd_app_new();

/// Runs an `NSTDApp`'s event loop.
///
/// # Note
///
/// This will take full control of the current thread and never return.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application to run.
NSTDAPI void nstd_app_run(NSTDApp app);

/// Frees an instance of `NSTDApp`. The application's event loop must not be ran after this is
/// called.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application.
NSTDAPI void nstd_app_free(NSTDApp app);

NSTDCPPEND
#endif
