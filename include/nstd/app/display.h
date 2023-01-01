#ifndef NSTD_APP_DISPLAY_H
#define NSTD_APP_DISPLAY_H
#include "../nstd.h"

/// Represents a monitor/display.
typedef NSTDAnyMut NSTDDisplay;

/// A handle to a display.
typedef NSTDAny NSTDDisplayHandle;

/// An owned display mode handle.
typedef NSTDAnyMut NSTDDisplayMode;

/// Represents a display's video mode.
typedef NSTDAny NSTDDisplayModeHandle;

/// Represents the size of a display.
typedef struct {
    /// The width of the display.
    NSTDUInt32 width;
    /// The height of the display.
    NSTDUInt32 height;
} NSTDDisplaySize;

/// Represents the position of a display.
typedef struct {
    /// The position of the display on the x-axis.
    NSTDInt32 x;
    /// The position of the display on the y-axis.
    NSTDInt32 y;
} NSTDDisplayPosition;

/// Creates a new `NSTDDisplay` from it's handle.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle handle` - A borrowed handle to a display.
///
/// # Returns
///
/// `NSTDDisplay display` - An owned handle to the display.
NSTDAPI NSTDDisplay nstd_app_display_new(NSTDDisplayHandle handle);

/// Immutably borrows an `NSTDDisplay`.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - The display handle to borrow.
///
/// # Returns
///
/// `NSTDDisplayHandle handle` - A borrowed handle to the display.
NSTDAPI NSTDDisplayHandle nstd_app_display_handle(const NSTDDisplay *display);

/// Returns the size of a display.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The size of the display.
NSTDAPI NSTDDisplaySize nstd_app_display_size(NSTDDisplayHandle display);

/// Returns the position of a display relative to the full-screen area.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplayPosition position` - The position of the display.
NSTDAPI NSTDDisplayPosition nstd_app_display_position(NSTDDisplayHandle display);

/// Returns the refresh rate of a display in millihertz.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate, possibly 0 on error.
NSTDAPI NSTDUInt32 nstd_app_display_refresh_rate(NSTDDisplayHandle display);

/// Returns the scale factor of a display.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The display's scale factor.
NSTDAPI NSTDFloat64 nstd_app_display_scale_factor(NSTDDisplayHandle display);

/// Invokes a callback function for each display mode detected for a display.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// - `void (*callback)(NSTDDisplayModeHandle)` - The callback function.
///
/// # Safety
///
/// The user of this function must guarantee that `callback` is a valid C function pointer.
NSTDAPI void nstd_app_display_modes(NSTDDisplayHandle display,
void (*callback)(NSTDDisplayModeHandle));

/// Creates a new `NSTDDisplayMode` from it's handle.
///
/// # Parameters:
///
/// - `NSTDDisplayModeHandle handle` - A borrowed handle to a display mode.
///
/// # Returns
///
/// `NSTDDisplayMode mode` - An owned representation of the display mode.
NSTDAPI NSTDDisplayMode nstd_app_display_mode_new(NSTDDisplayModeHandle handle);

/// Immutably borrows an `NSTDDisplayMode`.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode to borrow.
///
/// # Returns
///
/// `NSTDDisplayModeHandle handle` - A borrowed handle to the display mode.
NSTDAPI NSTDDisplayModeHandle nstd_app_display_mode_handle(const NSTDDisplayMode *mode);

/// Returns the size of a display mode.
///
/// # Parameters:
///
/// - `NSTDDisplayModeHandle mode` - The display mode.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The display mode's size.
NSTDAPI NSTDDisplaySize nstd_app_display_mode_size(NSTDDisplayModeHandle mode);

/// Returns the bit depth of a display mode.
///
/// # Parameters:
///
/// - `NSTDDisplayModeHandle mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt16 bit_depth` - The display mode's bit depth.
NSTDAPI NSTDUInt16 nstd_app_display_mode_bit_depth(NSTDDisplayModeHandle mode);

/// Returns the refresh rate of a display mode in millihertz.
///
/// # Parameters:
///
/// - `NSTDDisplayModeHandle mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate.
NSTDAPI NSTDUInt32 nstd_app_display_mode_refresh_rate(NSTDDisplayModeHandle mode);

#endif
