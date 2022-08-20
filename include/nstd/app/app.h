#ifndef NSTD_APP_APP_H
#define NSTD_APP_APP_H
#include "../nstd.h"
#include "data.h"
#include "events.h"
#include "handle.h"
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
/// This function must be called on the "main" thread, otherwise a panic may occur.
NSTDAPI NSTDApp nstd_app_new();

/// Returns a handle to an `NSTDApp`'s event loop.
///
/// # Parameters:
///
/// - `const NSTDApp *app` - The `nstd` application.
///
/// # Returns
///
/// `NSTDAppHandle handle` - A handle to the application's event loop.
NSTDAPI NSTDAppHandle nstd_app_handle(const NSTDApp *app);

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
/// - `NSTDAnyMut data` - Custom user data to pass to each app event.
///
/// # Safety
///
/// This function's caller must guarantee validity of the `app`'s event callbacks.
NSTDAPI void nstd_app_run(NSTDApp app, NSTDAnyMut data);

/// Frees an instance of `NSTDApp`. The application's event loop must not be ran after this is
/// called.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application.
NSTDAPI void nstd_app_free(NSTDApp app);

/// Signals an `NSTDApp`'s event loop to exit.
///
/// # Parameters:
///
/// - `const NSTDAppData *app` - The application data received from an event.
NSTDAPI void nstd_app_exit(const NSTDAppData *app);

NSTDCPPEND
#endif
