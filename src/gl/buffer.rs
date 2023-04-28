//! GPU memory buffers.
use super::{render_pass::NSTDGLRenderPass, NSTDGLRenderer};
use crate::{core::slice::NSTDSlice, NSTDUInt32};
use nstdapi::nstdapi;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};

/// Describes the type of a GPU buffer.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLBufferType {
    /// Represents a vertex buffer.
    NSTD_GL_BUFFER_TYPE_VERTEX,
    /// Represents a index buffer.
    NSTD_GL_BUFFER_TYPE_INDEX,
    /// Represents a uniform buffer.
    NSTD_GL_BUFFER_TYPE_UNIFORM,
}
impl From<NSTDGLBufferType> for BufferUsages {
    /// Converts an [NSTDGLBufferType] into [BufferUsages].
    #[inline]
    fn from(value: NSTDGLBufferType) -> Self {
        match value {
            NSTDGLBufferType::NSTD_GL_BUFFER_TYPE_VERTEX => Self::VERTEX,
            NSTDGLBufferType::NSTD_GL_BUFFER_TYPE_INDEX => Self::INDEX,
            NSTDGLBufferType::NSTD_GL_BUFFER_TYPE_UNIFORM => Self::UNIFORM,
        }
    }
}

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
/// - `NSTDGLBufferType buffer_type` - The type of buffer to create.
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
    buffer_type: NSTDGLBufferType,
) -> NSTDGLBuffer {
    let buffer_desc = BufferInitDescriptor {
        label: None,
        contents: data.as_slice(),
        usage: buffer_type.into(),
    };
    Box::new(renderer.renderer.device.create_buffer_init(&buffer_desc))
}

/// Makes `buffer` active for `render_pass` at `index`.
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

/// Frees a GPU buffer.
///
/// # Parameters:
///
/// - `NSTDGLBuffer buffer` - The buffer to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_buffer_free(buffer: NSTDGLBuffer) {}
