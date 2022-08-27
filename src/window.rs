//! An `nstd` application window.
use crate::{
    app::handle::NSTDAppHandle,
    core::str::NSTDStr,
    image::{nstd_image_as_bytes, nstd_image_height, nstd_image_width, NSTDImage},
    NSTDInt32, NSTDUInt32,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::{Icon, Window},
};

/// An `nstd` application window.
pub type NSTDWindow = Box<Window>;

/// Describes the position of a window.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDWindowPosition {
    /// The position of the window from the left of the screen.
    pub x: NSTDInt32,
    /// The position of the window from the top of the screen.
    pub y: NSTDInt32,
}

/// Describes the size of a window.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDWindowSize {
    /// The width of the window.
    pub width: NSTDUInt32,
    /// The height of the window.
    pub height: NSTDUInt32,
}

/// Creates a new window attached to `app`'s event loop.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to an `nstd` application.
///
/// # Returns
///
/// `NSTDWindow window` - A handle to the newly created window.
///
/// # Panics
///
/// This operation will panic if creating the new window fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_new(app: NSTDAppHandle) -> NSTDWindow {
    let window = Window::new(app).expect("Failed to create an nstd application window.");
    window.set_title("");
    Box::new(window)
}

/// Sets the title of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `const NSTDStr *title` - The new title of the window.
///
/// # Safety
///
/// This function can cause undefined behavior if `title`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_window_set_title(window: &NSTDWindow, title: &NSTDStr) {
    window.set_title(title.as_str())
}

/// Sets a window's icon to an RGBA image.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `const NSTDImage *icon` - The image to set as the window icon.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_set_icon(window: &NSTDWindow, icon: &NSTDImage) {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_set_position(window: &NSTDWindow, pos: NSTDWindowPosition) {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_get_position(window: &NSTDWindow) -> NSTDWindowPosition {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_get_inner_position(window: &NSTDWindow) -> NSTDWindowPosition {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_set_size(window: &NSTDWindow, size: NSTDWindowSize) {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_get_size(window: &NSTDWindow) -> NSTDWindowSize {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_window_get_outer_size(window: &NSTDWindow) -> NSTDWindowSize {
    let size = window.outer_size();
    NSTDWindowSize {
        width: size.width,
        height: size.height,
    }
}

/// Permanently closes & frees a window and it's data.
///
/// # Parameters:
///
/// - `NSTDWindow window` - The window to close.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_window_close(window: NSTDWindow) {}
