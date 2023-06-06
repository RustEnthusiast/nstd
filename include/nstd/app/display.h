#ifndef NSTD_APP_DISPLAY_H
#define NSTD_APP_DISPLAY_H
#include "../nstd.h"
#include "../string.h"
#include "../vec.h"

/// Represents a monitor/display.
typedef NSTDAnyMut NSTDDisplay;

/// An owned display mode handle.
typedef NSTDAnyMut NSTDDisplayMode;

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

/// Attempts to retrieve the name of a display.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDOptionalString name` - The name of the display if it could be obtained.
NSTDAPI NSTDOptionalString nstd_app_display_name(const NSTDDisplay *display);

/// Returns the size of a display.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The size of the display.
NSTDAPI NSTDDisplaySize nstd_app_display_size(const NSTDDisplay *display);

/// Returns the position of a display relative to the full-screen area.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplayPosition position` - The position of the display.
NSTDAPI NSTDDisplayPosition nstd_app_display_position(const NSTDDisplay *display);

/// Returns the refresh rate of a display in millihertz.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate, possibly 0 on error.
NSTDAPI NSTDUInt32 nstd_app_display_refresh_rate(const NSTDDisplay *display);

/// Returns the scale factor of a display.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The display's scale factor.
NSTDAPI NSTDFloat64 nstd_app_display_scale_factor(const NSTDDisplay *display);

/// Returns a display's valid display configurations.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDVec modes` - A vector of `display`'s `NSTDDisplayMode`s.
NSTDAPI void nstd_app_display_modes(const NSTDDisplay *display);

/// Frees an instance of `NSTDDisplay`.
///
/// # Parameters:
///
/// - `NSTDDisplay display` - The display.
NSTDAPI void nstd_app_display_free(NSTDDisplay display);

/// Returns the size of a display mode.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The display mode's size.
NSTDAPI NSTDDisplaySize nstd_app_display_mode_size(const NSTDDisplayMode *mode);

/// Returns the bit depth of a display mode.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt16 bit_depth` - The display mode's bit depth.
NSTDAPI NSTDUInt16 nstd_app_display_mode_bit_depth(const NSTDDisplayMode *mode);

/// Returns the refresh rate of a display mode in millihertz.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate.
NSTDAPI NSTDUInt32 nstd_app_display_mode_refresh_rate(const NSTDDisplayMode *mode);

/// Returns a handle to a display mode's display.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDDisplay display` - A handle to the display that `mode` is valid for.
NSTDAPI NSTDDisplay nstd_app_display_mode_handle(const NSTDDisplayMode *mode);

/// Frees an instance of `NSTDDisplayMode`.
///
/// # Parameters:
///
/// - `NSTDDisplayMode mode` - The display mode.
NSTDAPI void nstd_app_display_mode_free(NSTDDisplayMode mode);

#endif
