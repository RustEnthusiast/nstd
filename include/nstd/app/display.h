#ifndef NSTD_APP_DISPLAY_H
#define NSTD_APP_DISPLAY_H
#include "../nstd.h"

/// A handle to a display.
typedef NSTDAny NSTDDisplayHandle;

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

#endif
