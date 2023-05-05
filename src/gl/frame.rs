//! An individual window surface texture.
use super::{render_pass::NSTDGLRenderPass, NSTDGLRenderer};
use crate::core::result::NSTDResult;
use nstdapi::nstdapi;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    SurfaceError, SurfaceTexture, TextureView,
};

/// Describes an error returned from `nstd_gl_frame_new`.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLFrameError {
    /// A timeout was encountered while trying to acquire the next frame.
    NSTD_GL_FRAME_ERROR_TIMEOUT,
    /// The underlying surface has changed, and therefore the swap chain must be updated.
    NSTD_GL_FRAME_ERROR_OUTDATED,
    /// The swap chain has been lost and needs to be recreated.
    NSTD_GL_FRAME_ERROR_LOST,
    /// There is no memory left to allocate a new frame.
    NSTD_GL_FRAME_ERROR_OUT_OF_MEMORY,
}
impl From<SurfaceError> for NSTDGLFrameError {
    /// Converts a [SurfaceError] into an [NSTDGLFrameError].
    fn from(value: SurfaceError) -> Self {
        match value {
            SurfaceError::Timeout => Self::NSTD_GL_FRAME_ERROR_TIMEOUT,
            SurfaceError::Outdated => Self::NSTD_GL_FRAME_ERROR_OUTDATED,
            SurfaceError::Lost => Self::NSTD_GL_FRAME_ERROR_LOST,
            SurfaceError::OutOfMemory => Self::NSTD_GL_FRAME_ERROR_OUT_OF_MEMORY,
        }
    }
}

/// The frame.
struct Frame {
    /// The surface's texture.
    texture: SurfaceTexture,
    /// `texture`'s view.
    view: TextureView,
    /// The GPU command encoder.
    encoder: CommandEncoder,
}

/// An individual window surface texture.
#[nstdapi]
pub struct NSTDGLFrame {
    /// The inner frame.
    frame: Box<Frame>,
}

/// A result type returned from `nstd_gl_frame_new`.
pub type NSTDGLFrameResult = NSTDResult<NSTDGLFrame, NSTDGLFrameError>;

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
#[nstdapi]
pub fn nstd_gl_frame_new(renderer: &NSTDGLRenderer) -> NSTDGLFrameResult {
    // Get the swap chain's next texture.
    match renderer.renderer.surface.get_current_texture() {
        Ok(texture) => {
            let view = texture.texture.create_view(&Default::default());
            // Create the GPU command encoder.
            let encoder = renderer
                .renderer
                .device
                .create_command_encoder(&Default::default());
            // Construct the new frame.
            NSTDResult::Ok(NSTDGLFrame {
                frame: Box::new(Frame {
                    texture,
                    view,
                    encoder,
                }),
            })
        }
        Err(err) => NSTDResult::Err(err.into()),
    }
}

/// Creates a new render pass that may be used for drawing onto a frame.
///
/// # Parameters:
///
/// - `NSTDGLFrame *frame` - The frame to create a render pass for.
///
/// # Returns
///
/// `NSTDGLRenderPass render_pass` - The new render pass.
#[nstdapi]
pub fn nstd_gl_frame_render(frame: &mut NSTDGLFrame) -> NSTDGLRenderPass {
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

/// Draws `frame` onto the display.
///
/// # Parameters:
///
/// - `NSTDGLFrame frame` - The frame to display.
///
/// - `const NSTDGLRenderer *renderer` - The renderer used to create the frame.
#[inline]
#[nstdapi]
pub fn nstd_gl_frame_submit(frame: NSTDGLFrame, renderer: &NSTDGLRenderer) {
    // Submit the encoder's commands and output the next surface texture.
    renderer
        .renderer
        .device_handle
        .submit(Some(frame.frame.encoder.finish()));
    frame.frame.texture.present();
}
