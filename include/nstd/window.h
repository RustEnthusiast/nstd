#ifndef NSTD_WINDOW_H
#define NSTD_WINDOW_H
#include "app/data.h"
#include "core/str.h"
#include "image.h"
#include "nstd.h"

/// An `nstd` application window.
typedef NSTDAnyMut NSTDWindow;

/// Describes the position of a window.
typedef struct {
    /// The position of the window from the left of the screen.
    NSTDInt32 x;
    /// The position of the window from the top of the screen.
    NSTDInt32 y;
} NSTDWindowPosition;

/// Describes the size of a window.
typedef struct {
    /// The width of the window.
    NSTDUInt32 width;
    /// The height of the window.
    NSTDUInt32 height;
} NSTDWindowSize;

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
NSTDAPI NSTDWindow nstd_window_new(NSTDAppHandle app);

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
NSTDAPI void nstd_window_set_title(const NSTDWindow *window, const NSTDStr *title);

/// Sets a window's icon to an RGBA image.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `const NSTDImage *icon` - The image to set as the window icon.
NSTDAPI void nstd_window_set_icon(const NSTDWindow *window, const NSTDImage *icon);

/// Sets the position of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDWindowPosition pos` - The position of the window.
NSTDAPI void nstd_window_set_position(const NSTDWindow *window, NSTDWindowPosition pos);

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
NSTDAPI NSTDWindowPosition nstd_window_get_position(const NSTDWindow *window);

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
NSTDAPI NSTDWindowPosition nstd_window_get_inner_position(const NSTDWindow *window);

/// Sets the size of a window's client area.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// - `NSTDWindowSize size` - The new size of the window.
NSTDAPI void nstd_window_set_size(const NSTDWindow *window, NSTDWindowSize size);

/// Gets the size of a window's client area.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowSize size` - The size of the window.
NSTDAPI NSTDWindowSize nstd_window_get_size(const NSTDWindow *window);

/// Gets the full size of a window.
///
/// # Parameters:
///
/// - `const NSTDWindow *window` - The window.
///
/// # Returns
///
/// `NSTDWindowSize size` - The size of the window.
NSTDAPI NSTDWindowSize nstd_window_get_outer_size(const NSTDWindow *window);

/// Permanently closes & frees a window and it's data.
///
/// # Parameters:
///
/// - `NSTDWindow window` - The window to close.
NSTDAPI void nstd_window_close(NSTDWindow window);

#endif
