#ifndef NSTD_APP_APP_H
#define NSTD_APP_APP_H
#include "../nstd.h"
#include "events.h"
NSTDCPPSTART

/// An application event loop.
typedef struct {
    /// The application event callback function pointers.
    NSTDAppEvents events;
    /// The underlying event loop.
    NSTDAnyMut event_loop;
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

/// Returns a mutable reference to an `NSTDApp`'s event table.
///
/// # Parameters:
///
/// - `NSTDApp *app` - A pointer to the `nstd` app.
///
/// # Returns
///
/// `NSTDAppEvents *events` - A pointer to the application event table.
NSTDAPI NSTDAppEvents *nstd_app_events(NSTDApp *app);

/// Runs an `NSTDApp`'s event loop.
///
/// # Note
///
/// This function will take full control of the current thread and never return.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application to run.
///
/// # Safety
///
/// This function's caller must guarantee validity of the `app`'s event callbacks.
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
