//! Provides access to physical displays.
use crate::{
    alloc::{CBox, NSTD_ALLOCATOR},
    core::optional::{gen_optional, NSTDOptional},
    string::{NSTDOptionalString, NSTDString},
    vec::{nstd_vec_new, nstd_vec_push, NSTDVec},
    NSTDFloat64, NSTDInt32, NSTDUInt16, NSTDUInt32,
};
use nstdapi::nstdapi;
use std::ptr::addr_of;
use winit::monitor::{MonitorHandle, VideoMode};

/// Represents a monitor/display.
#[nstdapi]
pub struct NSTDDisplay {
    /// The inner [MonitorHandle].
    pub(super) handle: CBox<MonitorHandle>,
}
gen_optional!(NSTDOptionalDisplay, NSTDDisplay);

/// An owned display mode handle.
#[nstdapi]
pub struct NSTDDisplayMode {
    /// The inner [VideoMode].
    mode: CBox<VideoMode>,
}

/// Represents the size of a display.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDDisplaySize {
    /// The width of the display.
    pub width: NSTDUInt32,
    /// The height of the display.
    pub height: NSTDUInt32,
}

/// Represents the position of a display.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDDisplayPosition {
    /// The position of the display on the x-axis.
    pub x: NSTDInt32,
    /// The position of the display on the y-axis.
    pub y: NSTDInt32,
}

/// Attempts to retrieve the name of a display.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDOptionalString name` - The name of the display if it could be obtained.
#[inline]
#[nstdapi]
pub fn nstd_app_display_name(display: &NSTDDisplay) -> NSTDOptionalString {
    match display.handle.name() {
        Some(name) => NSTDOptional::Some(NSTDString::from_string(name)),
        _ => NSTDOptional::None,
    }
}

/// Returns the size of a display.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The size of the display.
#[inline]
#[nstdapi]
pub fn nstd_app_display_size(display: &NSTDDisplay) -> NSTDDisplaySize {
    let size = display.handle.size();
    NSTDDisplaySize {
        width: size.width,
        height: size.height,
    }
}

/// Returns the position of a display relative to the full-screen area.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDDisplayPosition position` - The position of the display.
#[inline]
#[nstdapi]
pub fn nstd_app_display_position(display: &NSTDDisplay) -> NSTDDisplayPosition {
    let position = display.handle.position();
    NSTDDisplayPosition {
        x: position.x,
        y: position.y,
    }
}

/// Returns the refresh rate of a display in millihertz.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate, possibly 0 on error.
#[inline]
#[nstdapi]
pub fn nstd_app_display_refresh_rate(display: &NSTDDisplay) -> NSTDUInt32 {
    display.handle.refresh_rate_millihertz().unwrap_or_default()
}

/// Returns the scale factor of a display.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The display's scale factor.
#[inline]
#[nstdapi]
pub fn nstd_app_display_scale_factor(display: &NSTDDisplay) -> NSTDFloat64 {
    display.handle.scale_factor()
}

/// Returns a display's valid display configurations.
///
/// # Parameters:
///
/// - `const NSTDDisplay *display` - A handle to the display.
///
/// # Returns
///
/// `NSTDVec modes` - A vector of `display`'s `NSTDDisplayMode`s.
#[nstdapi]
pub unsafe fn nstd_app_display_modes(display: &NSTDDisplay) -> NSTDVec {
    let mut modes = nstd_vec_new(&NSTD_ALLOCATOR, std::mem::size_of::<NSTDDisplayMode>());
    for mode in display.handle.video_modes() {
        if let Some(mode) = CBox::new(mode) {
            let m = NSTDDisplayMode { mode };
            nstd_vec_push(&mut modes, addr_of!(m) as _);
        }
    }
    modes
}

/// Frees an instance of `NSTDDisplay`.
///
/// # Parameters:
///
/// - `NSTDDisplay display` - The display.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_app_display_free(display: NSTDDisplay) {}

/// Returns the size of a display mode.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDDisplaySize size` - The display mode's size.
#[inline]
#[nstdapi]
pub fn nstd_app_display_mode_size(mode: &NSTDDisplayMode) -> NSTDDisplaySize {
    let size = mode.mode.size();
    NSTDDisplaySize {
        width: size.width,
        height: size.height,
    }
}

/// Returns the bit depth of a display mode.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt16 bit_depth` - The display mode's bit depth.
#[inline]
#[nstdapi]
pub fn nstd_app_display_mode_bit_depth(mode: &NSTDDisplayMode) -> NSTDUInt16 {
    mode.mode.bit_depth()
}

/// Returns the refresh rate of a display mode in millihertz.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDUInt32 refresh_rate` - The display's refresh rate.
#[inline]
#[nstdapi]
pub fn nstd_app_display_mode_refresh_rate(mode: &NSTDDisplayMode) -> NSTDUInt32 {
    mode.mode.refresh_rate_millihertz()
}

/// Returns a handle to a display mode's display.
///
/// # Parameters:
///
/// - `const NSTDDisplayMode *mode` - The display mode.
///
/// # Returns
///
/// `NSTDOptionalDisplay display` - A handle to the display that `mode` is valid for on success, or
/// an uninitialized "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_app_display_mode_handle(mode: &NSTDDisplayMode) -> NSTDOptionalDisplay {
    match CBox::new(mode.mode.monitor()) {
        Some(handle) => NSTDOptional::Some(NSTDDisplay { handle }),
        _ => NSTDOptional::None,
    }
}

/// Frees an instance of `NSTDDisplayMode`.
///
/// # Parameters:
///
/// - `NSTDDisplayMode mode` - The display mode.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_app_display_mode_free(mode: NSTDDisplayMode) {}
