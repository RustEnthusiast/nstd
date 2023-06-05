#ifndef NSTD_APP_APP_H
#define NSTD_APP_APP_H
#include "../core/def.h"
#include "../core/optional.h"
#include "../heap_ptr.h"
#include "../nstd.h"
#include "../vec.h"
#include "data.h"
#include "display.h"
#include "events.h"
#include "gamepad.h"

/// An application event loop.
typedef struct {
    /// The application event callback function pointers.
    NSTDAppEvents events;
    /// Private app data.
    NSTDAnyMut inner;
} NSTDApp;

/// Represents an optional value of type `NSTDApp`.
NSTDOptional(NSTDApp) NSTDOptionalApp;

/// Creates a new `nstd` application.
///
/// An `NSTDApp` can only be created once on the "main" thread. Attempting to recreate an `NSTDApp`
/// after one has already been created will result in a panic.
///
/// # Returns
///
/// `NSTDOptionalApp app` - The new `NSTDApp` or an uninitialized "none" variant if creating the
/// `nstd` application fails.
///
/// # Panics
///
/// This function may panic in the following situations:
///
/// - This function was not called on the "main" thread.
///
/// - An `NSTDApp` has already been created.
NSTDAPI NSTDOptionalApp nstd_app_new(void);

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
/// - `NSTDOptionalHeapPtr data` - Custom user data to pass to each app event.
///
/// # Safety
///
/// This function's caller must guarantee validity of the `app`'s event callbacks.
NSTDAPI void nstd_app_run(NSTDApp app, NSTDOptionalHeapPtr data);

/// Frees an instance of `NSTDApp`. The application's event loop must not be ran after this is
/// called.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application.
NSTDAPI void nstd_app_free(NSTDApp app);

/// Returns a vector of display handles detected by an `nstd` application.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to the `nstd` application.
///
/// # Returns
///
/// `NSTDVec displays` - A vector of `NSTDDisplay` handles.
NSTDAPI NSTDVec nstd_app_displays(NSTDAppHandle app);

/// Returns a handle to the primary display.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to the `nstd` application.
///
/// # Returns
///
/// `NSTDDisplay display` - A handle to the primary display, null on error.
NSTDAPI NSTDDisplay nstd_app_primary_display(NSTDAppHandle app);

/// Sets the `nstd` application's device filtering mode.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to the `nstd` application.
///
/// - `NSTDDeviceEventFilter filter` - The device event filtering mode to use.
NSTDAPI void nstd_app_set_device_event_filter(NSTDAppHandle app, NSTDDeviceEventFilter filter);

/// Returns a handle to a gamepad that matches `id`.
///
/// # Parameters:
///
/// - `const NSTDAppData *app` - The app.
///
/// - `const NSTDGamepadID *id` - The gamepad ID.
///
/// # Returns
///
/// `NSTDGamepad gamepad` - A handle to the gamepad with ID `id`.
NSTDAPI NSTDGamepad nstd_app_gamepad(const NSTDAppData *app, const NSTDGamepadID *id);

/// Returns a vector of all connected gamepad handles detected by `app`.
///
/// # Parameters:
///
/// - `const NSTDAppData *app` - The app.
///
/// # Returns
///
/// `NSTDVec gamepads` - A vector of `NSTDGamepad` handles.
NSTDAPI NSTDVec nstd_app_gamepads(const NSTDAppData *app);

/// Signals an `NSTDApp`'s event loop to exit.
///
/// # Parameters:
///
/// - `NSTDAppData *app` - The application data received from an event.
NSTDAPI void nstd_app_exit(NSTDAppData *app);

/// Signals an `NSTDApp`'s event loop to exit with a specific error code.
///
/// # Parameters:
///
/// - `NSTDAppData *app` - The application data received from an event.
///
/// - `NSTDErrorCode errc` - The error code to exit the application event loop with.
NSTDAPI void nstd_app_exit_with_code(NSTDAppData *app, NSTDErrorCode errc);

/// Checks if two `NSTDWindowID`s refer to the same window.
///
/// # Parameters:
///
/// - `const NSTDWindowID *id1` - The first ID.
///
/// - `const NSTDWindowID *id2` - The second ID.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two window IDs compare equal.
NSTDAPI NSTDBool nstd_app_window_id_compare(const NSTDWindowID *id1, const NSTDWindowID *id2);

/// Frees an instance of `NSTDWindowID`.
///
/// # Parameters:
///
/// - `NSTDWindowID id` - The window ID to free.
NSTDAPI void nstd_app_window_id_free(NSTDWindowID id);

/// Checks if two `NSTDDeviceID`s refer to the same device.
///
/// # Parameters:
///
/// - `const NSTDDeviceID *id1` - The first ID.
///
/// - `const NSTDDeviceID *id2` - The second ID.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two device IDs compare equal.
NSTDAPI NSTDBool nstd_app_device_id_compare(const NSTDDeviceID *id1, const NSTDDeviceID *id2);

/// Frees an instance of `NSTDDeviceID`.
///
/// # Parameters:
///
/// - `NSTDDeviceID id` - The device ID to free.
NSTDAPI void nstd_app_device_id_free(NSTDDeviceID id);

/// Checks if two `NSTDGamepadID`s refer to the same gamepad.
///
/// # Parameters:
///
/// - `const NSTDGamepadID *id1` - The first ID.
///
/// - `const NSTDGamepadID *id2` - The second ID.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two gamepad IDs compare equal.
NSTDAPI NSTDBool nstd_app_gamepad_id_compare(const NSTDGamepadID *id1, const NSTDGamepadID *id2);

/// Frees an instance of `NSTDGamepadID`.
///
/// # Parameters:
///
/// - `NSTDGamepadID id` - The gamepad ID to free.
NSTDAPI void nstd_app_gamepad_id_free(NSTDGamepadID id);

#endif
