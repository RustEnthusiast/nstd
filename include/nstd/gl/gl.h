#ifndef NSTD_GL_GL_H
#define NSTD_GL_GL_H
#include "../core/result.h"
#include "../nstd.h"
#include "../window.h"

/// Describes an error returned by an `nstd.gl` function.
typedef enum {
    /// No error occurred.
    NSTD_GL_ERROR_NONE,
    /// A GPU device adapter could not be acquired.
    NSTD_GL_ERROR_ADAPTER_NOT_FOUND,
    /// A GPU device handle could not be acquired.
    NSTD_GL_ERROR_DEVICE_NOT_FOUND
} NSTDGLError;

/// Represents a rendering backend.
typedef enum {
    /// Unknown backend.
    ///
    /// When creating a rendering backend instance, this value will be useful for enabling use of
    /// all rendering backends.
    NSTD_GL_BACKEND_UNKNOWN,
    /// The [Vulkan](https://en.wikipedia.org/wiki/Vulkan) API.
    ///
    /// This backend should be supported on most modern platforms.
    NSTD_GL_BACKEND_VULKAN,
    /// The [OpenGL](https://en.wikipedia.org/wiki/OpenGL) API.
    ///
    /// This backend is currently "unsupported".
    NSTD_GL_BACKEND_OPENGL,
    /// The [Direct 3D 11](https://en.wikipedia.org/wiki/Direct3D#Direct3D_11) API.
    ///
    /// This backend is supported on Windows 7+.
    NSTD_GL_BACKEND_DX11,
    /// The [Direct 3D 12](https://en.wikipedia.org/wiki/Direct3D#Direct3D_12) API.
    ///
    /// This backend is supported on Windows 10+.
    NSTD_GL_BACKEND_DX12,
    /// The [Metal](https://en.wikipedia.org/wiki/Metal_(API)) API.
    ///
    /// This backend is supported on Apple systems.
    NSTD_GL_BACKEND_METAL,
    /// The [WebGPU](https://en.wikipedia.org/wiki/WebGPU) API.
    ///
    /// This backend is supported by the web through WebAssembly.
    NSTD_GL_BACKEND_WEBGPU
} NSTDGLBackend;

/// A power preference.
///
/// This type is used for querying drawing devices.
typedef enum {
    /// No power preference.
    NSTD_GL_POWER_PREFERENCE_NONE,
    /// A low power preference.
    NSTD_GL_POWER_PREFERENCE_LOW,
    /// A high power preference.
    NSTD_GL_POWER_PREFERENCE_HIGH
} NSTDGLPowerPreference;

/// Represents a surface's presentation mode.
typedef enum {
    /// Attempts to choose a non-VSync presentation mode automatically.
    ///
    /// Chooses immediate -> mailbox -> fifo based on availability.
    NSTD_GL_PRESENTATION_MODE_AUTO,
    /// Attempts to choose a VSync presentation mode automatically.
    ///
    /// Chooses fifo relaxed -> fifo based on availability.
    NSTD_GL_PRESENTATION_MODE_AUTO_VSYNC,
    /// Frames are kept in a first-in-first-out queue, every vertical blanking period a frame is
    /// popped off of this queue and displayed. If a frame is not ready to be displayed, it will
    /// present the same frame until the next vertical blanking period.
    ///
    /// Traditionally called "VSync".
    ///
    /// Supported on all platforms.
    NSTD_GL_PRESENTATION_MODE_FIFO,
    /// Frames are kept in a first-in-first-out queue, every vertical blanking period a frame is
    /// popped off of this queue and displayed. If a frame is not ready to be displayed, it will
    /// present the same frame until there is a frame in the queue, immediately popping the frame
    /// from the queue.
    ///
    /// Traditionally called "Adaptive VSync".
    ///
    /// Supported on AMD on Vulkan.
    NSTD_GL_PRESENTATION_MODE_FIFO_RELAXED,
    /// Frames are not queued at all. When a present command is executed, the presented image is
    /// immediately swapped onto the front buffer.
    ///
    /// Traditionally called "VSync off".
    ///
    /// Supported on most platforms apart from older Direct3D 12 and Wayland.
    NSTD_GL_PRESENTATION_MODE_IMMEDIATE,
    /// Frames are kept in a single-frame queue, every vertical blanking period a frame is popped
    /// off of this queue and displayed. If a frame is not ready to be displayed, it will present
    /// the same frame until the next vertical blanking period.
    ///
    /// Traditionally called "Fast VSync".
    ///
    /// Supported on Direct 3D 11/12, Nvidia on Vulkan, and Wayland on Vulkan.
    NSTD_GL_PRESENTATION_MODE_MAILBOX
} NSTDGLPresentationMode;

/// Describes the creation of an `NSTDGLRenderer`.
typedef struct {
    /// A reference to the window to create a renderer for.
    const NSTDWindow *window;
    /// The rendering backend to use.
    NSTDGLBackend backend;
    /// The power preference to use when querying for a drawing device.
    NSTDGLPowerPreference power_preference;
    /// The presentation mode to use for the renderer's surface.
    NSTDGLPresentationMode presentation_mode;
} NSTDGLRendererDescriptor;

/// `nstd.gl`'s renderer.
///
/// This type creates a rendering surface on an `NSTDWindow`.
typedef struct {
    /// The inner renderer.
    NSTDAnyMut renderer;
} NSTDGLRenderer;

/// The result type returned from `nstd_gl_renderer_new`.
NSTDResult(NSTDGLRenderer, NSTDGLError) NSTDGLRendererResult;

/// Creates a new rendering context with a rendering surface and a handle to a drawing device.
///
/// # Parameters:
///
/// - `const NSTDGLRendererDescriptor *desc` - The renderer descriptor.
///
/// # Returns
///
/// `NSTDGLRendererResult renderer` - The new `nstd.gl` renderer on success, or an error code on
/// failure.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - A default GPU adapter could not be found.
///
/// - A default device handle could not be made.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - This operation is called with the Metal backend while not on the "main" thread.
///
/// - In some situations when a default device handle could not be made.
///
/// # Safety
///
/// `desc.window` must remain alive while the returned object is alive.
NSTDAPI NSTDGLRendererResult nstd_gl_renderer_new(const NSTDGLRendererDescriptor *desc);

/// Resizes a renderer's surface.
///
/// This will have no effect if either `size.width` or `size.height` are zero.
///
/// # Parameters
///
/// - `NSTDGLRenderer *renderer` - The renderer.
///
/// - `NSTDUInt32 width` - The new width to give the renderer's surface.
///
/// - `NSTDUInt32 height` - The new height to give the renderer's surface.
NSTDAPI void nstd_gl_renderer_resize(NSTDGLRenderer *renderer, NSTDUInt32 width, NSTDUInt32 height);

/// Frees an instance of `NSTDGLRenderer`.
///
/// # Parameters:
///
/// - `NSTDGLRenderer renderer` - The renderer to free.
NSTDAPI void nstd_gl_renderer_free(NSTDGLRenderer renderer);

#endif
