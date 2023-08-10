#ifndef NSTD_GL_TEXTURE_H
#define NSTD_GL_TEXTURE_H
#include "../core/optional.h"
#include "../image.h"
#include "../nstd.h"
#include "gl.h"

/// A shader's texture.
typedef struct {
    /// The `wgpu` texture.
    NSTDAnyMut texture;
} NSTDGLTexture;

/// Represents an optional value of type `NSTDGLTexture`.
NSTDOptional(NSTDGLTexture) NSTDGLOptionalTexture;

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
NSTDAPI NSTDGLOptionalTexture
nstd_gl_texture_new(const NSTDGLRenderer *renderer, const NSTDImage *image);

/// Frees an instance of `NSTDGLTexture`.
///
/// # Parameters:
///
/// - `NSTDGLTexture texture` - The texture to free.
NSTDAPI void nstd_gl_texture_free(NSTDGLTexture texture);

#endif
