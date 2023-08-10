//! A shader's texture.
use super::NSTDGLRenderer;
use crate::{
    alloc::CBox,
    core::optional::{gen_optional, NSTDOptional},
    image::NSTDImage,
};
use image::GenericImageView;
use nstdapi::nstdapi;
use std::num::NonZeroU32;
use wgpu::{
    Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Texture as WgpuTexture, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
};

/// A texture.
struct Texture {
    /// The [wgpu] [WgpuTexture].
    #[allow(dead_code)]
    texture: WgpuTexture,
    /// The texture view.
    view: TextureView,
}

/// A shader's texture.
#[nstdapi]
pub struct NSTDGLTexture {
    /// The `wgpu` texture.
    texture: CBox<Texture>,
}
impl NSTDGLTexture {
    /// Returns an immutable reference to the texture view.
    #[inline]
    pub(super) fn view(&self) -> &TextureView {
        &self.texture.view
    }
}
gen_optional!(NSTDGLOptionalTexture, NSTDGLTexture);

/// Creates a new `NSTDGLTexture` from an `NSTDImage`.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer.
///
/// - `const NSTDImage *image` - The image to initialize the texture with.
///
/// # Returns
///
/// `NSTDGLOptionalTexture texture` - The new texture on success, or an uninitialized "none"
/// variant on error.
#[nstdapi]
pub fn nstd_gl_texture_new(renderer: &NSTDGLRenderer, image: &NSTDImage) -> NSTDGLOptionalTexture {
    // Create the texture.
    let dimensions = image.dimensions();
    let size = Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    let desc = TextureDescriptor {
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        label: None,
        view_formats: &[],
    };
    let texture = renderer.renderer.device.create_texture(&desc);
    // Write the image to the texture.
    let copy_view = ImageCopyTexture {
        texture: &texture,
        aspect: TextureAspect::All,
        origin: Origin3d::ZERO,
        mip_level: 0,
    };
    let image_layout = ImageDataLayout {
        offset: 0,
        bytes_per_row: NonZeroU32::new(dimensions.0 * 4).map(|n| n.get()),
        rows_per_image: NonZeroU32::new(dimensions.1).map(|n| n.get()),
    };
    let rgba = image.to_rgba8();
    renderer
        .renderer
        .device_handle
        .write_texture(copy_view, &rgba, image_layout, size);
    // Create the texture view.
    let view = texture.create_view(&Default::default());
    match CBox::new(Texture { texture, view }) {
        Some(texture) => NSTDOptional::Some(NSTDGLTexture { texture }),
        _ => NSTDOptional::None,
    }
}

/// Frees an instance of `NSTDGLTexture`.
///
/// # Parameters:
///
/// - `NSTDGLTexture texture` - The texture to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_texture_free(texture: NSTDGLTexture) {}
