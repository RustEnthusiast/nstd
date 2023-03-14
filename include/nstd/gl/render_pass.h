#ifndef NSTD_GL_RENDER_PASS_H
#define NSTD_GL_RENDER_PASS_H
#include "../core/range.h"
#include "../nstd.h"
#include "frame.h"

/// Represents a single render pass.
typedef NSTDAnyMut NSTDGLRenderPass;

/// Creates a new render pass that may be used for drawing onto `frame`.
///
/// # Parameters:
///
/// - `NSTDGLFrame *frame` - The frame to create a render pass for.
///
/// # Returns
///
/// `NSTDGLRenderPass render_pass` - The new render pass.
NSTDAPI NSTDGLRenderPass nstd_gl_render_pass_new(NSTDGLFrame *frame);

/// Draws primitives from active vertex buffers.
///
/// # Parameters:
///
/// - `NSTDGLRenderPass *render_pass` - The render pass.
///
/// - `NSTDRangeU32 vertices` - The range of vertices to draw.
///
/// - `NSTDRangeU32 instances` - The range of instances to draw.
NSTDAPI void nstd_gl_render_pass_draw(
    NSTDGLRenderPass *render_pass, NSTDRangeU32 vertices, NSTDRangeU32 instances
);

/// Frees an instance of `NSTDGLRenderPass`.
///
/// # Parameters:
///
/// - `NSTDGLRenderPass render_pass` - The render pass to free.
NSTDAPI void nstd_gl_render_pass_free(NSTDGLRenderPass render_pass);

#endif
