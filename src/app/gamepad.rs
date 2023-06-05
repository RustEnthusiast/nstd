//! Gamepad access.
use crate::{
    app::events::{NSTDGamepadAxis, NSTDGamepadButton, NSTDGamepadID},
    core::str::NSTDStr,
    NSTDBool, NSTDFloat32,
};
use gilrs::Gamepad;
use nstdapi::nstdapi;

/// A handle to a gamepad.
pub type NSTDGamepad<'a> = Box<Gamepad<'a>>;

/// Returns a gamepad's unique ID.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDGamepadID id` - The gamepad's unique ID.
#[inline]
#[nstdapi]
pub fn nstd_app_gamepad_id(gamepad: &NSTDGamepad) -> NSTDGamepadID {
    Box::new(gamepad.id())
}

/// Returns the name of a gamepad.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDStr name` - The gamepad's name.
#[inline]
#[nstdapi]
pub fn nstd_app_gamepad_name(gamepad: &NSTDGamepad) -> NSTDStr {
    NSTDStr::from_str(gamepad.name())
}

/// Returns the operating system supplied name of a gamepad.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDStr name` - The gamepad's name as given by the operating system.
#[inline]
#[nstdapi]
pub fn nstd_app_gamepad_os_name(gamepad: &NSTDGamepad) -> NSTDStr {
    NSTDStr::from_str(gamepad.os_name())
}

/// Determines whether or not a gamepad is currently connected to the system.
///
/// # Parameters:
///
/// - `const NSTDGamepad *gamepad` - A handle to the gamepad.
///
/// # Returns
///
/// `NSTDBool is_connected` - Returns true if `gamepad` is currently connected to the system.
#[inline]
#[nstdapi]
pub fn nstd_app_gamepad_is_connected(gamepad: &NSTDGamepad) -> NSTDBool {
    gamepad.is_connected()
}

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
#[inline]
#[nstdapi]
pub fn nstd_app_gamepad_button_is_pressed(
    gamepad: &NSTDGamepad,
    button: NSTDGamepadButton,
) -> NSTDBool {
    gamepad.is_pressed(button.into_winit())
}

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
#[inline]
#[nstdapi]
pub fn nstd_app_gamepad_axis_value(gamepad: &NSTDGamepad, axis: NSTDGamepadAxis) -> NSTDFloat32 {
    gamepad.value(axis.into_winit())
}

/// Frees an instance of `NSTDGamepad`.
///
/// # Parameters:
///
/// - `NSTDGamepad gamepad` - The gamepad.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_app_gamepad_free(gamepad: NSTDGamepad) {}
