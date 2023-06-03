#ifndef NSTD_GL_BUFFER_H
#define NSTD_GL_BUFFER_H
#include "../core/slice.h"
#include "../nstd.h"
#include "gl.h"
#include "render_pass.h"

/// A bit flag that instructs [nstd_gl_buffer_new] to create a vertex buffer.
#define NSTD_GL_VERTEX_BUFFER 1
/// A bit flag that instructs [nstd_gl_buffer_new] to create an index buffer.
#define NSTD_GL_INDEX_BUFFER (1 << 1)
/// A bit flag that instructs [nstd_gl_buffer_new] to create a uniform buffer.
#define NSTD_GL_UNIFORM_BUFFER (1 << 2)
/// A bit flag that instructs [nstd_gl_buffer_new] to create a readable buffer.
#define NSTD_GL_SRC_BUFFER (1 << 3)
/// A bit flag that instructs [nstd_gl_buffer_new] to create a writable buffer.
#define NSTD_GL_DEST_BUFFER (1 << 4)

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
/// - `NSTDUInt8 usages` - A bit mask describing what type of buffer to create.
///
/// # Panics
///
/// This operation will panic if `data`'s stride is not 1.
///
/// # Safety
///
/// `data` must be valid for reads.
NSTDAPI NSTDGLBuffer
nstd_gl_buffer_new(const NSTDGLRenderer *renderer, const NSTDSlice *data, NSTDUInt8 usages);

/// Makes `buffer` an active vertex buffer for `render_pass` at `index`.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to bind.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
///
/// - `NSTDUInt32 index` - The index (or slot) to bind the buffer to.
NSTDAPI void nstd_gl_buffer_bind_vertex(
    const NSTDGLBuffer *buffer, NSTDGLRenderPass *render_pass, NSTDUInt32 index
);

/// Makes `buffer` an active index buffer for `render_pass`.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to bind.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
NSTDAPI void nstd_gl_buffer_bind_index(const NSTDGLBuffer *buffer, NSTDGLRenderPass *render_pass);

/// Writes data into a GPU buffer.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to write to.
///
/// - `const NSTDGLRenderer *renderer` - The renderer.
///
/// - `const NSTDSlice *data` - The data to write to the buffer.
///
/// - `NSTDUInt64 offset` - The offset to use for the write operation.
///
/// # Panics
///
/// This operation will panic if `data`'s stride is not 1.
///
/// # Safety
///
/// `data` must be valid for reads.
NSTDAPI void nstd_gl_buffer_write(
    const NSTDGLBuffer *buffer, const NSTDGLRenderer *renderer, const NSTDSlice *data,
    NSTDUInt64 offset
);

/// Frees a GPU buffer.
///
/// # Parameters:
///
/// - `NSTDGLBuffer buffer` - The buffer to free.
NSTDAPI void nstd_gl_buffer_free(NSTDGLBuffer buffer);

#endif
