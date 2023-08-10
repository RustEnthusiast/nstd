#ifndef NSTD_GL_RENDER_PASS_H
#define NSTD_GL_RENDER_PASS_H
#include "../core/optional.h"
#include "../core/range.h"
#include "../nstd.h"

/// Represents a single render pass.
typedef struct {
    /// The inner `RenderPass`.
    NSTDAnyMut pass;
} NSTDGLRenderPass;

/// Represents an optional value of type `NSTDGLRenderPass`.
NSTDOptional(NSTDGLRenderPass) NSTDGLOptionalRenderPass;

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

/// Draws indexed primitives from active vertex and index buffers.
///
/// # Parameters:
///
/// - `NSTDGLRenderPass *render_pass` - The render pass.
///
/// - `NSTDRangeU32 indices` - The range of indices to draw.
///
/// - `NSTDInt32 base` - The index of the first vertex to draw.
///
/// - `NSTDRangeU32 instances` - The range of instances to draw.
NSTDAPI void nstd_gl_render_pass_draw_indexed(
    NSTDGLRenderPass *render_pass, NSTDRangeU32 indices, NSTDInt32 base, NSTDRangeU32 instances
);

/// Frees an instance of `NSTDGLRenderPass`.
///
/// # Parameters:
///
/// - `NSTDGLRenderPass render_pass` - The render pass to free.
NSTDAPI void nstd_gl_render_pass_free(NSTDGLRenderPass render_pass);

#endif
