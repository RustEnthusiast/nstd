#ifndef NSTD_GL_FRAME_H
#define NSTD_GL_FRAME_H
#include "../core/result.h"
#include "../nstd.h"
#include "gl.h"
#include "render_pass.h"

/// Describes an error returned from `nstd_gl_frame_new`.
typedef enum {
    /// A timeout was encountered while trying to acquire the next frame.
    NSTD_GL_FRAME_ERROR_TIMEOUT,
    /// The underlying surface has changed, and therefore the swap chain must be updated.
    NSTD_GL_FRAME_ERROR_OUTDATED,
    /// The swap chain has been lost and needs to be recreated.
    NSTD_GL_FRAME_ERROR_LOST,
    /// There is no memory left to allocate a new frame.
    NSTD_GL_FRAME_ERROR_OUT_OF_MEMORY
} NSTDGLFrameError;

/// An individual window surface texture.
typedef struct {
    /// The inner frame.
    NSTDAnyMut frame;
} NSTDGLFrame;

/// A result type returned from `nstd_gl_frame_new`.
NSTDResult(NSTDGLFrame, NSTDGLFrameError) NSTDGLFrameResult;

/// Gets `renderer`'s swap chain's next frame.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer.
///
/// # Returns
///
/// `NSTDGLFrameResult frame` - Renderer's next frame on success, or a value indicating an error on
/// failure.
///
/// # Panics
///
/// This operation will panic if another frame is alive.
NSTDAPI NSTDGLFrameResult nstd_gl_frame_new(const NSTDGLRenderer *renderer);

/// Creates a new render pass that may be used for drawing onto a frame.
///
/// # Parameters:
///
/// - `NSTDGLFrame *frame` - The frame to create a render pass for.
///
/// # Returns
///
/// `NSTDGLRenderPass render_pass` - The new render pass.
NSTDAPI NSTDGLRenderPass nstd_gl_frame_render(NSTDGLFrame *frame);

/// Draws `frame` onto the display.
///
/// # Parameters:
///
/// - `NSTDGLFrame frame` - The frame to display.
///
/// - `const NSTDGLRenderer *renderer` - The renderer used to create the frame.
NSTDAPI void nstd_gl_frame_submit(NSTDGLFrame frame, const NSTDGLRenderer *renderer);

#endif
