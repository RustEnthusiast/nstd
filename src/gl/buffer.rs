//! GPU memory buffers.
use super::{render_pass::NSTDGLRenderPass, NSTDGLRenderer};
use crate::{core::slice::NSTDSlice, NSTDUInt32, NSTDUInt8};
use nstdapi::nstdapi;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, IndexFormat,
};

/// Create a vertex buffer.
pub const NSTD_GL_BUFFER_TYPE_VERTEX: NSTDUInt8 = 1;
/// Create a index buffer.
pub const NSTD_GL_BUFFER_TYPE_INDEX: NSTDUInt8 = 1 << 1;
/// Create a uniform buffer.
pub const NSTD_GL_BUFFER_TYPE_UNIFORM: NSTDUInt8 = 1 << 2;

/// GPU memory buffers.
pub type NSTDGLBuffer = Box<Buffer>;

/// Creates and initializes a new GPU buffer with `data`.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer to create the buffer with.
///
/// - `const NSTDSlice *data` - The data to send to the GPU.
///
/// - `NSTDUInt8 buffer_type` - A bit mask describing what type of buffer to create.
///
/// # Panics
///
/// This operation will panic if `data`'s stride is not 1.
///
/// # Safety
///
/// `data` must be valid for reads.
#[nstdapi]
pub unsafe fn nstd_gl_buffer_new(
    renderer: &NSTDGLRenderer,
    data: &NSTDSlice,
    buffer_type: NSTDUInt8,
) -> NSTDGLBuffer {
    let mut usage = BufferUsages::empty();
    (buffer_type & NSTD_GL_BUFFER_TYPE_VERTEX != 0).then(|| usage |= BufferUsages::VERTEX);
    (buffer_type & NSTD_GL_BUFFER_TYPE_INDEX != 0).then(|| usage |= BufferUsages::INDEX);
    (buffer_type & NSTD_GL_BUFFER_TYPE_UNIFORM != 0).then(|| usage |= BufferUsages::UNIFORM);
    let buffer_desc = BufferInitDescriptor {
        label: None,
        contents: data.as_slice(),
        usage,
    };
    Box::new(renderer.renderer.device.create_buffer_init(&buffer_desc))
}

/// Makes `buffer` an active vertex buffer for `render_pass` at `index`.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to bind.
///
/// - `NSTDUInt32 index` - The index (or slot) to bind the buffer to.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
#[inline]
#[nstdapi]
pub fn nstd_gl_buffer_bind_vertex<'a: 'b, 'b>(
    buffer: &'a NSTDGLBuffer,
    index: NSTDUInt32,
    render_pass: &mut NSTDGLRenderPass<'b>,
) {
    render_pass.set_vertex_buffer(index, buffer.slice(..));
}

/// Makes `buffer` an active index buffer for `render_pass`.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to bind.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
#[inline]
#[nstdapi]
pub fn nstd_gl_buffer_bind_index<'a: 'b, 'b>(
    buffer: &'a NSTDGLBuffer,
    render_pass: &mut NSTDGLRenderPass<'b>,
) {
    render_pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint32);
}

/// Frees a GPU buffer.
///
/// # Parameters:
///
/// - `NSTDGLBuffer buffer` - The buffer to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_buffer_free(buffer: NSTDGLBuffer) {}
