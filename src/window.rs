//! An `nstd` application window.
use crate::{
    app::{data::NSTDAppHandle, events::NSTDWindowID},
    core::{
        optional::{gen_optional, NSTDOptional},
        str::NSTDStr,
    },
    image::{nstd_image_as_bytes, nstd_image_height, nstd_image_width, NSTDImage},
    NSTDBool, NSTDFloat64, NSTDInt32, NSTDUInt32,
};
use nstdapi::nstdapi;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::{Icon, Window},
};

/// An `nstd` application window.
pub type NSTDWindow = Box<Window>;

/// Describes the position of a window.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDWindowPosition {
    /// The position of the window from the left of the screen.
    pub x: NSTDInt32,
    /// The position of the window from the top of the screen.
    pub y: NSTDInt32,
}

/// Describes the size of a window.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDWindowSize {
    /// The width of the window.
    pub width: NSTDUInt32,
    /// The height of the window.
    pub height: NSTDUInt32,
}
gen_optional!(NSTDOptionalWindowSize, NSTDWindowSize);

/// Creates a new window attached to `app`'s event loop.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to an `nstd` application.
///
/// # Returns
///
/// `NSTDWindow window` - A handle to the newly created window, or null on error.
#[nstdapi]
pub fn nstd_window_new(app: NSTDAppHandle) -> Option<NSTDWindow> {
    let window = match Window::new(app) {
        Ok(window) => window,
        _ => return None,
    };
    window.set_title("");
    Some(Box::new(window))
}

/// Returns a window's unique identifier.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowID window_id` - The window's unique identifier.
#[inline]
#[nstdapi]
pub fn nstd_window_id(window: &NSTDWindow) -> NSTDWindowID {
    Box::new(window.id())
}

/// Sets the title of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `const NSTDStr *title` - The new title of the window.
///
/// # Panics
///
/// Panics if `title`'s length in bytes is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// This function can cause undefined behavior if `title`'s data is invalid.
#[inline]
#[nstdapi]
pub unsafe fn nstd_window_set_title(window: &NSTDWindow, title: &NSTDStr) {
    window.set_title(title.as_str())
}

/// Sets a window's icon to an RGBA image.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `const NSTDImage *icon` - The image to set as the window icon.
///
/// # Panics
///
/// Panics if the image's length in bytes exceeds `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_window_set_icon(window: &NSTDWindow, icon: &NSTDImage) {
    let width = nstd_image_width(icon);
    let height = nstd_image_height(icon);
    let bytes = nstd_image_as_bytes(icon);
    // SAFETY: `icon` owns the data.
    let rgba = Vec::from(unsafe { bytes.as_slice() });
    window.set_window_icon(Icon::from_rgba(rgba, width, height).ok());
}

/// Sets the position of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDWindowPosition pos` - The position of the window.
#[inline]
#[nstdapi]
pub fn nstd_window_set_position(window: &NSTDWindow, pos: NSTDWindowPosition) {
    window.set_outer_position(PhysicalPosition::new(pos.x, pos.y));
}

/// Gets the position of a window.
///
/// This always returns an x and y value of 0 on unsupported platforms.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowPosition pos` - The position of the window.
#[inline]
#[nstdapi]
pub fn nstd_window_get_position(window: &NSTDWindow) -> NSTDWindowPosition {
    if let Ok(pos) = window.outer_position() {
        return NSTDWindowPosition { x: pos.x, y: pos.y };
    }
    NSTDWindowPosition { x: 0, y: 0 }
}

/// Gets the position of a window's client area on the display.
///
/// This always returns an x and y value of 0 on unsupported platforms.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowPosition pos` - The position of the window's client area.
#[inline]
#[nstdapi]
pub fn nstd_window_get_inner_position(window: &NSTDWindow) -> NSTDWindowPosition {
    if let Ok(pos) = window.inner_position() {
        return NSTDWindowPosition { x: pos.x, y: pos.y };
    }
    NSTDWindowPosition { x: 0, y: 0 }
}

/// Sets the size of a window's client area.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDWindowSize size` - The new size of the window.
#[inline]
#[nstdapi]
pub fn nstd_window_set_size(window: &NSTDWindow, size: NSTDWindowSize) {
    window.set_inner_size(PhysicalSize::new(size.width, size.height));
}

/// Gets the size of a window's client area.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowSize size` - The size of the window.
#[inline]
#[nstdapi]
pub fn nstd_window_get_size(window: &NSTDWindow) -> NSTDWindowSize {
    let size = window.inner_size();
    NSTDWindowSize {
        width: size.width,
        height: size.height,
    }
}

/// Gets the full size of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowSize size` - The size of the window.
#[inline]
#[nstdapi]
pub fn nstd_window_get_outer_size(window: &NSTDWindow) -> NSTDWindowSize {
    let size = window.outer_size();
    NSTDWindowSize {
        width: size.width,
        height: size.height,
    }
}

/// Sets a window's minimum size.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDOptionalWindowSize size` - The minimum window size, pass none to disable clamping.
#[inline]
#[nstdapi]
pub fn nstd_window_set_min_size(window: &NSTDWindow, size: NSTDOptionalWindowSize) {
    let size = match size {
        NSTDOptional::Some(size) => Some(PhysicalSize::new(size.width, size.height)),
        _ => None,
    };
    window.set_min_inner_size(size);
}

/// Sets a window's maximum size.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDOptionalWindowSize size` - The maximum window size, pass none to disable clamping.
#[inline]
#[nstdapi]
pub fn nstd_window_set_max_size(window: &NSTDWindow, size: NSTDOptionalWindowSize) {
    let size = match size {
        NSTDOptional::Some(size) => Some(PhysicalSize::new(size.width, size.height)),
        _ => None,
    };
    window.set_max_inner_size(size);
}

/// Returns the scale factor of a window.
///
/// # Parameter:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The window's scale factor.
#[inline]
#[nstdapi]
pub fn nstd_window_scale_factor(window: &NSTDWindow) -> NSTDFloat64 {
    window.scale_factor()
}

/// Sets whether or not a window is resizable.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDBool resizable` - True if the window should be resizable.
#[inline]
#[nstdapi]
pub fn nstd_window_set_resizable(window: &NSTDWindow, resizable: NSTDBool) {
    window.set_resizable(resizable);
}

/// Checks if a window is resizable.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDBool is_resizable` - Returns true if the window is resizable.
#[inline]
#[nstdapi]
pub fn nstd_window_is_resizable(window: &NSTDWindow) -> NSTDBool {
    window.is_resizable()
}

/// Permanently closes & frees a window and it's data.
///
/// # Parameters:
///
/// - `NSTDWindow window` - The window to close.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_window_close(window: NSTDWindow) {}
