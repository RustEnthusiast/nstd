#ifndef NSTD_GL_BUFFER_H
#define NSTD_GL_BUFFER_H
#include "../core/slice.h"
#include "../nstd.h"
#include "gl.h"
#include "render_pass.h"

/// Describes the type of a GPU buffer.
typedef enum {
    /// Represents a vertex buffer.
    NSTD_GL_BUFFER_TYPE_VERTEX,
    /// Represents a index buffer.
    NSTD_GL_BUFFER_TYPE_INDEX,
    /// Represents a uniform buffer.
    NSTD_GL_BUFFER_TYPE_UNIFORM
} NSTDGLBufferType;

/// GPU memory buffers.
typedef NSTDAnyMut NSTDGLBuffer;

/// Creates and initializes a new GPU buffer with `data`.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer to create the buffer with.
///
/// - `const NSTDSlice *data` - The data to send to the GPU.
///
/// - `NSTDGLBufferType buffer_type` - The type of buffer to create.
///
/// # Panics
///
/// This operation will panic if `data`'s stride is not 1.
///
/// # Safety
///
/// `data` must be valid for reads.
NSTDAPI NSTDGLBuffer nstd_gl_buffer_new(
    const NSTDGLRenderer *renderer, const NSTDSlice *data, NSTDGLBufferType buffer_type
);

/// Makes `buffer` active for `render_pass` at `index`.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to bind.
///
/// - `NSTDUInt32 index` - The index (or slot) to bind the buffer to.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
NSTDAPI void nstd_gl_buffer_bind_vertex(
    const NSTDGLBuffer *buffer, NSTDUInt32 index, NSTDGLRenderPass *render_pass
);

/// Frees a GPU buffer.
///
/// # Parameters:
///
/// - `NSTDGLBuffer buffer` - The buffer to free.
NSTDAPI void nstd_gl_buffer_free(NSTDGLBuffer buffer);

#endif
