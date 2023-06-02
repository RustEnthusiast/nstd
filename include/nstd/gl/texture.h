#ifndef NSTD_GL_TEXTURE_H
#define NSTD_GL_TEXTURE_H
#include "../image.h"
#include "../nstd.h"
#include "gl.h"

/// A shader's texture.
typedef struct {
    /// The `wgpu` texture.
    NSTDAnyMut texture;
} NSTDGLTexture;

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
/// `NSTDGLTexture texture` - The new texture.
NSTDAPI NSTDGLTexture nstd_gl_texture_new(const NSTDGLRenderer *renderer, const NSTDImage *image);

/// Frees an instance of `NSTDGLTexture`.
///
/// # Parameters:
///
/// - `NSTDGLTexture texture` - The texture to free.
NSTDAPI void nstd_gl_texture_free(NSTDGLTexture texture);

#endif
