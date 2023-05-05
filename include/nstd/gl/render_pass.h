#ifndef NSTD_GL_RENDER_PASS_H
#define NSTD_GL_RENDER_PASS_H
#include "../core/range.h"
#include "../nstd.h"

/// Represents a single render pass.
typedef NSTDAnyMut NSTDGLRenderPass;

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
