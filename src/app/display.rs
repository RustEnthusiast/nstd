//! Provides access to physical displays.
use crate::{NSTDFloat64, NSTDInt32, NSTDUInt16, NSTDUInt32};
use winit::monitor::{MonitorHandle, VideoMode};

/// Represents a monitor/display.
pub type NSTDDisplay = Box<MonitorHandle>;

/// A handle to a display.
pub type NSTDDisplayHandle<'a> = &'a MonitorHandle;

/// Represents a display's video mode.
pub type NSTDDisplayMode<'a> = &'a VideoMode;

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

/// Invokes a callback function for each display mode detected for a display.
///
/// # Parameters:
///
/// - `NSTDDisplayHandle display` - A handle to the display.
///
/// - `void (*callback)(NSTDDisplayMode)` - The callback function.
///
/// # Safety
///
/// The user of this function must guarantee that `callback` is a valid C function pointer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_app_display_modes(
    display: NSTDDisplayHandle,
    callback: Option<unsafe extern "C" fn(NSTDDisplayMode)>,
) {
    if let Some(callback) = callback {
        for mode in display.video_modes() {
            callback(&mode);
        }
    }
}

/// Returns the size of a display mode.
///
/// # Parameters:
///
/// - `NSTDDisplayMode mode` - The display mode.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The display mode's size.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_mode_size(mode: NSTDDisplayMode) -> NSTDDisplaySize {
    let size = mode.size();
    NSTDDisplaySize {
        width: size.width,
        height: size.height,
    }
}

/// Returns the bit depth of a display mode.
///
/// # Parameters:
///
/// - `NSTDDisplayMode mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt16 bit_depth` - The display mode's bit depth.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_mode_bit_depth(mode: NSTDDisplayMode) -> NSTDUInt16 {
    mode.bit_depth()
}

/// Returns the refresh rate of a display mode in millihertz.
///
/// # Parameters:
///
/// - `NSTDDisplayMode mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_display_mode_refresh_rate(mode: NSTDDisplayMode) -> NSTDUInt32 {
    mode.refresh_rate_millihertz()
}