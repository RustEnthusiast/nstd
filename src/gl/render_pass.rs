//! Represents a single render pass.
use super::frame::NSTDGLFrame;
use crate::core::range::NSTDRangeU32;
use nstdapi::nstdapi;
use wgpu::{
    Color, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
};

/// Represents a single render pass.
pub type NSTDGLRenderPass<'a> = Box<RenderPass<'a>>;

/// Creates a new render pass that may be used for drawing onto `frame`.
///
/// # Parameters:
///
/// - `NSTDGLFrame *frame` - The frame to create a render pass for.
///
/// # Returns
///
/// `NSTDGLRenderPass render_pass` - The new render pass.
#[nstdapi]
pub fn nstd_gl_render_pass_new(frame: &mut NSTDGLFrame) -> NSTDGLRenderPass {
    let render_pass_desc = RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(RenderPassColorAttachment {
            view: &frame.frame.view,
            ops: Operations {
                load: LoadOp::Clear(Color::BLACK),
                store: true,
            },
            resolve_target: None,
        })],
        depth_stencil_attachment: None,
    };
    Box::new(frame.frame.encoder.begin_render_pass(&render_pass_desc))
}

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
    render_pass.draw(vertices.start..vertices.end, instances.start..instances.end);
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
