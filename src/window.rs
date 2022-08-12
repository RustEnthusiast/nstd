//! An `nstd` application window.
use crate::{app::handle::NSTDAppHandle, core::str::NSTDStrConst};
use winit::window::Window;

/// An `nstd` application window.
pub type NSTDWindow = Box<Window>;

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
    Box::new(Window::new(app).expect("Failed to create an nstd application window."))
}

/// Sets the title of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `const NSTDStrConst *title` - The new title of the window.
///
/// # Safety
///
/// This function can cause undefined behavior if `title`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_window_set_title(window: &NSTDWindow, title: &NSTDStrConst) {
    window.set_title(title.as_str())
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
