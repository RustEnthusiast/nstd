#ifndef NSTD_GL_SAMPLER_H
#define NSTD_GL_SAMPLER_H
#include "../core/optional.h"
#include "../nstd.h"
#include "gl.h"

/// Describes how a texture's edges should be handled by a sampler.
typedef enum {
    /// Repeats the texture.
    NSTD_GL_TEXTURE_WRAP_REPEAT,
    /// Same as `NSTD_GL_TEXTURE_WRAP_REPEAT` but this will mirror the texture for each repeat.
    NSTD_GL_TEXTURE_WRAP_MIRRORED_REPEAT,
    /// Stretches the edge of the texture.
    NSTD_GL_TEXTURE_WRAP_CLAMP_TO_EDGE,
    /// Clears non-textured fragments with `color`.
    NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER
} NSTDGLTextureWrap;

/// Describes how a sampler should filter/mix texture pixels.
typedef enum {
    /// Selects the texture pixel that is closest to the texture coordinate.
    NSTD_GL_SAMPLER_FILTER_NEAREST,
    /// Takes an interpolated value from the texture coordinate's neighboring texture pixels.
    NSTD_GL_SAMPLER_FILTER_LINEAR
} NSTDGLSamplerFilter;

/// Describes a valid color value that may be used with `NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER`.
typedef enum {
    /// Use the default border color.
    NSTD_GL_SAMPLER_BORDER_COLOR_NONE,
    /// An opaque black (0, 0, 0, 1).
    NSTD_GL_SAMPLER_BORDER_COLOR_BLACK,
    /// An opaque white (1, 1, 1, 1).
    NSTD_GL_SAMPLER_BORDER_COLOR_WHITE,
    /// A transparent black (0, 0, 0, 0).
    NSTD_GL_SAMPLER_BORDER_COLOR_TRANSPARENT_BLACK
} NSTDGLSamplerBorderColor;

/// Describes the creation of an `NSTDGLSampler`.
typedef struct {
    /// The texture's wrapping mode in the u/x direction.
    NSTDGLTextureWrap wrap_mode_u;
    /// The texture's wrapping mode in the v/y direction.
    NSTDGLTextureWrap wrap_mode_v;
    /// The texture's wrapping mode in the w/z direction.
    NSTDGLTextureWrap wrap_mode_w;
    /// The color value to use with `NSTD_GL_TEXTURE_WRAP_CLAMP_TO_BORDER`.
    NSTDGLSamplerBorderColor border_color;
    /// Describes how to filter the texture when it needs to be magnified.
    NSTDGLSamplerFilter mag_filter;
    /// Describes how to filter the texture when it needs to be minified.
    NSTDGLSamplerFilter min_filter;
    /// Describes how the sampler should filter between mip map levels.
    NSTDGLSamplerFilter mipmap_filter;
} NSTDGLSamplerDescriptor;

/// A shader's texture sampler.
typedef struct {
    /// The inner `Sampler`.
    NSTDAnyMut sampler;
} NSTDGLSampler;

/// Represents an optional value of type `NSTDGLSampler`.
NSTDOptional(NSTDGLSampler) NSTDGLOptionalSampler;

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
NSTDAPI NSTDGLOptionalSampler
nstd_gl_sampler_new(const NSTDGLRenderer *renderer, const NSTDGLSamplerDescriptor *desc);

/// Frees a texture sampler.
///
/// # Parameters:
///
/// - `NSTDGLSampler sampler` - The sampler to free.
NSTDAPI void nstd_gl_sampler_free(NSTDGLSampler sampler);

#endif
