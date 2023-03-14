//! An individual window surface texture.
use super::NSTDGLRenderer;
use crate::core::optional::{gen_optional, NSTDOptional};
use nstdapi::nstdapi;
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

/// The frame.
pub(super) struct Frame {
    /// `texture`'s view.
    pub(super) view: TextureView,
    /// The GPU command encoder.
    pub(super) encoder: CommandEncoder,
    /// The surface's texture.
    texture: SurfaceTexture,
}

/// An individual window surface texture.
#[nstdapi]
pub struct NSTDGLFrame {
    /// The inner frame.
    pub(super) frame: Box<Frame>,
}
gen_optional!(NSTDGLOptionalFrame, NSTDGLFrame);

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
#[nstdapi]
pub fn nstd_gl_frame_new(renderer: &NSTDGLRenderer) -> NSTDGLOptionalFrame {
    // Get the swap chain's next texture.
    if let Ok(texture) = renderer.renderer.surface.get_current_texture() {
        let view = texture.texture.create_view(&Default::default());
        // Create the GPU command encoder.
        let encoder = renderer
            .renderer
            .device
            .create_command_encoder(&Default::default());
        // Construct the new frame.
        return NSTDOptional::Some(NSTDGLFrame {
            frame: Box::new(Frame {
                view,
                encoder,
                texture,
            }),
        });
    }
    NSTDOptional::None
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
