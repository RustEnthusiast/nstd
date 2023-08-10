//! GPU memory buffers.
use super::{render_pass::NSTDGLRenderPass, NSTDGLRenderer};
use crate::{
    alloc::CBox,
    core::{
        optional::{gen_optional, NSTDOptional},
        slice::NSTDSlice,
    },
    NSTDUInt32, NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, IndexFormat,
};

/// A bit flag that instructs [nstd_gl_buffer_new] to create a vertex buffer.
pub const NSTD_GL_VERTEX_BUFFER: NSTDUInt8 = 1;
/// A bit flag that instructs [nstd_gl_buffer_new] to create an index buffer.
pub const NSTD_GL_INDEX_BUFFER: NSTDUInt8 = 1 << 1;
/// A bit flag that instructs [nstd_gl_buffer_new] to create a uniform buffer.
pub const NSTD_GL_UNIFORM_BUFFER: NSTDUInt8 = 1 << 2;
/// A bit flag that instructs [nstd_gl_buffer_new] to create a readable buffer.
pub const NSTD_GL_SRC_BUFFER: NSTDUInt8 = 1 << 3;
/// A bit flag that instructs [nstd_gl_buffer_new] to create a writable buffer.
pub const NSTD_GL_DEST_BUFFER: NSTDUInt8 = 1 << 4;

/// GPU memory buffers.
#[nstdapi]
pub struct NSTDGLBuffer {
    /// The inner `Buffer`.
    buffer: CBox<Buffer>,
}
impl NSTDGLBuffer {
    /// Returns an immutable reference to the inner buffer.
    #[inline]
    pub(super) fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}
gen_optional!(NSTDGLOptionalBuffer, NSTDGLBuffer);

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
/// # Returns
///
/// `NSTDGLOptionalBuffer buffer` - The new buffer on success, or an uninitialized "none" variant
/// on error.
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
    usages: NSTDUInt8,
) -> NSTDGLOptionalBuffer {
    let mut usage = BufferUsages::empty();
    (usages & NSTD_GL_VERTEX_BUFFER != 0).then(|| usage |= BufferUsages::VERTEX);
    (usages & NSTD_GL_INDEX_BUFFER != 0).then(|| usage |= BufferUsages::INDEX);
    (usages & NSTD_GL_UNIFORM_BUFFER != 0).then(|| usage |= BufferUsages::UNIFORM);
    (usages & NSTD_GL_SRC_BUFFER != 0).then(|| usage |= BufferUsages::COPY_SRC);
    (usages & NSTD_GL_DEST_BUFFER != 0).then(|| usage |= BufferUsages::COPY_DST);
    let buffer_desc = BufferInitDescriptor {
        label: None,
        contents: data.as_slice(),
        usage,
    };
    match CBox::new(renderer.renderer.device.create_buffer_init(&buffer_desc)) {
        Some(buffer) => NSTDOptional::Some(NSTDGLBuffer { buffer }),
        _ => NSTDOptional::None,
    }
}

/// Makes `buffer` an active vertex buffer for `render_pass` at `index`.
///
/// # Parameters:
///
/// - `const NSTDGLBuffer *buffer` - The buffer to bind.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
///
/// - `NSTDUInt32 index` - The index (or slot) to bind the buffer to.
#[inline]
#[nstdapi]
pub fn nstd_gl_buffer_bind_vertex<'a: 'b, 'b>(
    buffer: &'a NSTDGLBuffer,
    render_pass: &mut NSTDGLRenderPass<'b>,
    index: NSTDUInt32,
) {
    render_pass
        .pass
        .set_vertex_buffer(index, buffer.buffer.slice(..));
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
    render_pass
        .pass
        .set_index_buffer(buffer.buffer.slice(..), IndexFormat::Uint32);
}

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
#[inline]
#[nstdapi]
pub unsafe fn nstd_gl_buffer_write(
    buffer: &NSTDGLBuffer,
    renderer: &NSTDGLRenderer,
    data: &NSTDSlice,
    offset: NSTDUInt64,
) {
    renderer
        .renderer
        .device_handle
        .write_buffer(&buffer.buffer, offset as _, data.as_slice());
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
