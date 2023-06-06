#ifndef NSTD_APP_GAMEPAD_H
#define NSTD_APP_GAMEPAD_H
#include "../core/str.h"
#include "../nstd.h"
#include "events.h"

/// A handle to a gamepad.
typedef NSTDAnyMut NSTDGamepad;

/// Returns a gamepad's unique ID.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDGamepadID id` - The gamepad's unique ID.
NSTDAPI NSTDGamepadID nstd_app_gamepad_id(const NSTDGamepad *gamepad);

/// Returns the name of a gamepad.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDStr name` - The gamepad's name.
NSTDAPI NSTDStr nstd_app_gamepad_name(const NSTDGamepad *gamepad);

/// Returns the operating system supplied name of a gamepad.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDStr name` - The gamepad's name as given by the operating system.
NSTDAPI NSTDStr nstd_app_gamepad_os_name(const NSTDGamepad *gamepad);

/// Determines whether or not a gamepad is currently connected to the system.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDBool is_connected` - Returns true if `gamepad` is currently connected to the system.
NSTDAPI NSTDBool nstd_app_gamepad_is_connected(const NSTDGamepad *gamepad);

/// Determines whether or not a gamepad's `button` is currently pressed.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// - `NSTDGamepadButton button` - The button to check.
///
/// # Returns
///
/// `NSTDBool is_pressed` - Returns true if `gamepad`'s `button` is currently pressed.
NSTDAPI NSTDBool
nstd_app_gamepad_button_is_pressed(const NSTDGamepad *gamepad, NSTDGamepadButton button);

/// Gets the current value of a gamepad axis.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// - `NSTDGamepadAxis axis` - The gamepad axis to check.
///
/// # Returns
///
/// `NSTDFloat32 value` - `axis`'s current value.
NSTDAPI NSTDFloat32 nstd_app_gamepad_axis_value(const NSTDGamepad *gamepad, NSTDGamepadAxis axis);

/// Frees an instance of `NSTDGamepad`.
///
/// # Parameters:
///
/// - `NSTDGamepad gamepad` - The gamepad.
NSTDAPI void nstd_app_gamepad_free(NSTDGamepad gamepad);

#endif
