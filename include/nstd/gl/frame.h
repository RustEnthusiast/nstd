#ifndef NSTD_GL_FRAME_H
#define NSTD_GL_FRAME_H
#include "../core/optional.h"
#include "../nstd.h"
#include "gl.h"

/// An individual window surface texture.
typedef struct {
    /// The inner frame.
    NSTDAnyMut frame;
} NSTDGLFrame;

/// Represents an optional value of type `NSTDGLFrame`.
NSTDOptional(NSTDGLFrame) NSTDGLOptionalFrame;

/// Gets `renderer`'s swap chain's next frame.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer.
///
/// # Returns
///
/// `NSTDGLOptionalFrame frame` - Renderer's next frame.
///
/// # Panics
///
/// This operation will panic if another frame is alive.
NSTDAPI NSTDGLOptionalFrame nstd_gl_frame_new(const NSTDGLRenderer *renderer);

/// Draws `frame` onto the display.
///
/// # Parameters:
///
/// - `NSTDGLFrame frame` - The frame to display.
///
/// - `const NSTDGLRenderer *renderer` - The renderer used to create the frame.
NSTDAPI void nstd_gl_frame_submit(NSTDGLFrame frame, const NSTDGLRenderer *renderer);

#endif
