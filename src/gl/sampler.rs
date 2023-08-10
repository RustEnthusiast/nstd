//! A shader's texture sampler.
use self::NSTDGLSamplerBorderColor::*;
use super::NSTDGLRenderer;
use crate::{
    alloc::CBox,
    core::optional::{gen_optional, NSTDOptional},
};
use nstdapi::nstdapi;
use wgpu::{AddressMode, FilterMode, Sampler, SamplerBorderColor, SamplerDescriptor};

/// Describes how a texture's edges should be handled by a sampler.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLTextureWrap {
    /// Repeats the texture.
    NSTD_GL_TEXTURE_WRAP_REPEAT,
    /// Same as `NSTD_GL_TEXTURE_WRAP_REPEAT` but this will mirror the texture for each repeat.
    NSTD_GL_TEXTURE_WRAP_MIRRORED_REPEAT,
    /// Stretches the edge of the texture.
    NSTD_GL_TEXTURE_WRAP_CLAMP_TO_EDGE,
    /// Clears non-textured fragments with `color`.
    NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER,
}
impl From<NSTDGLTextureWrap> for AddressMode {
    /// Converts an [NSTDGLTextureWrap] into an [AddressMode].
    fn from(value: NSTDGLTextureWrap) -> Self {
        match value {
            NSTDGLTextureWrap::NSTD_GL_TEXTURE_WRAP_REPEAT => Self::Repeat,
            NSTDGLTextureWrap::NSTD_GL_TEXTURE_WRAP_MIRRORED_REPEAT => Self::MirrorRepeat,
            NSTDGLTextureWrap::NSTD_GL_TEXTURE_WRAP_CLAMP_TO_EDGE => Self::ClampToEdge,
            NSTDGLTextureWrap::NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER => Self::ClampToBorder,
        }
    }
}

/// Describes how a sampler should filter/mix texture pixels.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLSamplerFilter {
    /// Selects the texture pixel that is closest to the texture coordinate.
    NSTD_GL_SAMPLER_FILTER_NEAREST,
    /// Takes an interpolated value from the texture coordinate's neighboring texture pixels.
    NSTD_GL_SAMPLER_FILTER_LINEAR,
}
impl From<NSTDGLSamplerFilter> for FilterMode {
    /// Converts an [NSTDGLSamplerFilter] into a [FilterMode].
    #[inline]
    fn from(value: NSTDGLSamplerFilter) -> Self {
        match value {
            NSTDGLSamplerFilter::NSTD_GL_SAMPLER_FILTER_NEAREST => Self::Nearest,
            NSTDGLSamplerFilter::NSTD_GL_SAMPLER_FILTER_LINEAR => Self::Linear,
        }
    }
}

/// Describes a valid color value that may be used with `NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER`.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLSamplerBorderColor {
    /// Use the default border color.
    NSTD_GL_SAMPLER_BORDER_COLOR_NONE,
    /// An opaque black (0, 0, 0, 1).
    NSTD_GL_SAMPLER_BORDER_COLOR_BLACK,
    /// An opaque white (1, 1, 1, 1).
    NSTD_GL_SAMPLER_BORDER_COLOR_WHITE,
    /// A transparent black (0, 0, 0, 0).
    NSTD_GL_SAMPLER_BORDER_COLOR_TRANSPARENT_BLACK,
}
impl From<NSTDGLSamplerBorderColor> for Option<SamplerBorderColor> {
    /// Converts an [NSTDGLSamplerBorderColor] into a [SamplerBorderColor].
    fn from(value: NSTDGLSamplerBorderColor) -> Self {
        match value {
            NSTD_GL_SAMPLER_BORDER_COLOR_NONE => None,
            NSTD_GL_SAMPLER_BORDER_COLOR_BLACK => Some(SamplerBorderColor::OpaqueBlack),
            NSTD_GL_SAMPLER_BORDER_COLOR_WHITE => Some(SamplerBorderColor::OpaqueWhite),
            NSTD_GL_SAMPLER_BORDER_COLOR_TRANSPARENT_BLACK => {
                Some(SamplerBorderColor::TransparentBlack)
            }
        }
    }
}

/// Describes the creation of an `NSTDGLSampler`.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NSTDGLSamplerDescriptor {
    /// The texture's wrapping mode in the u/x direction.
    pub wrap_mode_u: NSTDGLTextureWrap,
    /// The texture's wrapping mode in the v/y direction.
    pub wrap_mode_v: NSTDGLTextureWrap,
    /// The texture's wrapping mode in the w/z direction.
    pub wrap_mode_w: NSTDGLTextureWrap,
    /// The color value to use with `NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER`.
    pub border_color: NSTDGLSamplerBorderColor,
    /// Describes how to filter the texture when it needs to be magnified.
    pub mag_filter: NSTDGLSamplerFilter,
    /// Describes how to filter the texture when it needs to be minified.
    pub min_filter: NSTDGLSamplerFilter,
    /// Describes how the sampler should filter between mip map levels.
    pub mipmap_filter: NSTDGLSamplerFilter,
}
impl From<&NSTDGLSamplerDescriptor> for SamplerDescriptor<'_> {
    /// Converts an [NSTDGLSamplerDescriptor] into a [SamplerDescriptor].
    fn from(value: &NSTDGLSamplerDescriptor) -> Self {
        Self {
            address_mode_u: value.wrap_mode_u.into(),
            address_mode_v: value.wrap_mode_v.into(),
            address_mode_w: value.wrap_mode_w.into(),
            border_color: value.border_color.into(),
            mag_filter: value.mag_filter.into(),
            min_filter: value.min_filter.into(),
            mipmap_filter: value.mipmap_filter.into(),
            ..Default::default()
        }
    }
}

/// A shader's texture sampler.
#[nstdapi]
pub struct NSTDGLSampler {
    /// The inner `Sampler`.
    sampler: CBox<Sampler>,
}
impl NSTDGLSampler {
    /// Returns an immutable reference to the inner sampler.
    #[inline]
    pub(super) fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}
gen_optional!(NSTDGLOptionalSampler, NSTDGLSampler);

/// Creates a new texture sampler.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer.
///
/// - `const NSTDGLSamplerDescriptor *desc` - The sampler descriptor.
///
/// # Returns
///
/// `NSTDGLOptionalSampler sampler` - The new texture sampler on success, or an uninitialized
/// "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_gl_sampler_new(
    renderer: &NSTDGLRenderer,
    desc: &NSTDGLSamplerDescriptor,
) -> NSTDGLOptionalSampler {
    match CBox::new(renderer.renderer.device.create_sampler(&desc.into())) {
        Some(sampler) => NSTDOptional::Some(NSTDGLSampler { sampler }),
        _ => NSTDOptional::None,
    }
}

/// Frees a texture sampler.
///
/// # Parameters:
///
/// - `NSTDGLSampler sampler` - The sampler to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_sampler_free(sampler: NSTDGLSampler) {}
