//! Represents a single render pass.
use crate::{
    alloc::CBox,
    core::{optional::NSTDOptional, range::NSTDRangeU32},
    NSTDInt32,
};
use nstdapi::nstdapi;
use wgpu::RenderPass;

/// Represents a single render pass.
#[nstdapi]
pub struct NSTDGLRenderPass<'a> {
    /// The inner `RenderPass`.
    pub(super) pass: CBox<RenderPass<'a>>,
}

/// Represents an optional value of type `NSTDGLRenderPass`.
pub type NSTDGLOptionalRenderPass<'a> = NSTDOptional<NSTDGLRenderPass<'a>>;

/// Draws primitives from active vertex buffers.
///
/// # Parameters:
///
/// - `NSTDGLRenderPass *render_pass` - The render pass.
///
/// - `NSTDRangeU32 vertices` - The range of vertices to draw.
///
/// - `NSTDRangeU32 instances` - The range of instances to draw.
#[inline]
#[nstdapi]
pub fn nstd_gl_render_pass_draw(
    render_pass: &mut NSTDGLRenderPass,
    vertices: NSTDRangeU32,
    instances: NSTDRangeU32,
) {
    render_pass
        .pass
        .draw(vertices.start..vertices.end, instances.start..instances.end);
}

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
#[inline]
#[nstdapi]
pub fn nstd_gl_render_pass_draw_indexed(
    render_pass: &mut NSTDGLRenderPass,
    indices: NSTDRangeU32,
    base: NSTDInt32,
    instances: NSTDRangeU32,
) {
    render_pass.pass.draw_indexed(
        indices.start..indices.end,
        base,
        instances.start..instances.end,
    );
}

/// Frees an instance of `NSTDGLRenderPass`.
///
/// # Parameters:
///
/// - `NSTDGLRenderPass render_pass` - The render pass to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_render_pass_free(render_pass: NSTDGLRenderPass) {}
