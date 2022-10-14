//! Provides access to physical displays.
use crate::{NSTDFloat64, NSTDInt32, NSTDUInt32};
use winit::monitor::MonitorHandle;

/// A handle to a display.
pub type NSTDDisplayHandle<'a> = &'a MonitorHandle;

/// Represents the size of a display.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NSTDDisplaySize {
    /// The width of the display.
    pub width: NSTDUInt32,
    /// The height of the display.
    pub height: NSTDUInt32,
}

/// Represents the position of a display.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NSTDDisplayPosition {
    /// The position of the display on the x-axis.
    pub x: NSTDInt32,
    /// The position of the display on the y-axis.
    pub y: NSTDInt32,
}

/// Returns the size of a display.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The size of the display.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_size(display: NSTDDisplayHandle) -> NSTDDisplaySize {
    let size = display.size();
    NSTDDisplaySize {
        width: size.width,
        height: size.height,
    }
}

/// Returns the position of a display relative to the full-screen area.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplayPosition position` - The position of the display.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_position(display: NSTDDisplayHandle) -> NSTDDisplayPosition {
    let position = display.position();
    NSTDDisplayPosition {
        x: position.x,
        y: position.y,
    }
}

/// Returns the refresh rate of a display in millihertz.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate, possibly 0 on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_refresh_rate(display: NSTDDisplayHandle) -> NSTDUInt32 {
    display.refresh_rate_millihertz().unwrap_or_default()
}

/// Returns the scale factor of a display.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The display's scale factor.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_scale_factor(display: NSTDDisplayHandle) -> NSTDFloat64 {
    display.scale_factor()
}
