//! Represents group of bindings for a shader.
use super::{
    buffer::NSTDGLBuffer, render_pass::NSTDGLRenderPass, shader::NSTDGLShaderStage::*,
    NSTDGLRenderer,
};
use crate::{core::slice::NSTDSlice, NSTDBool, NSTDUInt32, NSTDUInt8};
use nstdapi::nstdapi;
use wgpu::{
    BindGroup as WgpuBindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBindingType, ShaderStages,
};

/// Describes a buffer's binding type.
#[nstdapi]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum NSTDGLBufferBindingType {
    /// Describes a read/write uniform buffer.
    Uniform,
    /// Describes a possibly read-only storage buffer.
    Storage {
        /// Determines whether or not the storage buffer is read-only.
        read_only: NSTDBool,
    },
}
impl From<NSTDGLBufferBindingType> for BufferBindingType {
    /// Converts an [NSTDGLBufferBindingType] into a `wgpu` [BufferBindingType].
    #[inline]
    fn from(value: NSTDGLBufferBindingType) -> Self {
        match value {
            NSTDGLBufferBindingType::Uniform => Self::Uniform,
            NSTDGLBufferBindingType::Storage { read_only } => Self::Storage { read_only },
        }
    }
}

/// Describes a bind group entry's type.
#[nstdapi]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum NSTDGLBindingType {
    /// Describes a binding for a GPU memory buffer.
    Buffer {
        /// The buffer's binding type.
        buffer_binding_type: NSTDGLBufferBindingType,
    },
}
impl From<NSTDGLBindingType> for BindingType {
    /// Converts an [NSTDGLBindingType] into a `wgpu` [BindingType].
    fn from(value: NSTDGLBindingType) -> Self {
        match value {
            NSTDGLBindingType::Buffer {
                buffer_binding_type,
            } => Self::Buffer {
                ty: buffer_binding_type.into(),
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }
    }
}

/// Represents a bind group resource.
#[nstdapi]
#[derive(Clone, Copy)]
pub enum NSTDGLBindingResource<'a> {
    /// Represents a GPU memory buffer binding.
    Buffer {
        /// A reference to the buffer to use as a binding resource.
        buffer: &'a NSTDGLBuffer,
    },
}
impl<'a> From<NSTDGLBindingResource<'a>> for BindingResource<'a> {
    /// Converts an [NSTDGLBindingResource] into a `wgpu` [BindingResource].
    fn from(value: NSTDGLBindingResource<'a>) -> Self {
        match value {
            NSTDGLBindingResource::Buffer { buffer } => {
                Self::Buffer(buffer.as_entire_buffer_binding())
            }
        }
    }
}

/// Describes the creation of a bind group entry.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDGLBindGroupEntry<'a> {
    /// The entry's binding resource.
    pub resource: NSTDGLBindingResource<'a>,
    /// The entry's binding type.
    pub binding_type: NSTDGLBindingType,
    /// A bitset describing which parts of the render pipeline should be able to use the binding.
    pub visibility: NSTDUInt8,
}

/// Bind group data.
struct BindGroup {
    /// The `wgpu` bind group.
    bind_group: WgpuBindGroup,
    /// The bind group layout.
    layout: BindGroupLayout,
}

/// Represents group of bindings for a shader.
#[nstdapi]
pub struct NSTDGLBindGroup {
    /// Heap data.
    bind_group: Box<BindGroup>,
}
impl NSTDGLBindGroup {
    /// Returns a reference to the bind group's layout.
    #[inline]
    pub(super) fn layout(&self) -> &BindGroupLayout {
        &self.bind_group.layout
    }
}

/// Creates a new shader bind group.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer to create the bind group for.
///
/// - `const NSTDSlice *entries` - The bind group entries.
///
/// # Returns
///
/// `NSTDGLBindGroup bind_group` - The new bind group.
///
/// # Panics
///
/// This operation may panic in the following situations:
///
/// - `entries`'s stride does not match `NSTDGLBindGroupEntry`'s size in bytes.
///
/// - Memory allocation fails.
///
/// # Safety
///
/// `entries` must be valid for reads.
#[nstdapi]
pub unsafe fn nstd_gl_bind_group_new(
    renderer: &NSTDGLRenderer,
    entries: &NSTDSlice,
) -> NSTDGLBindGroup {
    // Create the entries.
    let entries = entries.as_slice::<NSTDGLBindGroupEntry>();
    let mut layout_entries = Vec::with_capacity(entries.len());
    let mut bind_group_entries = Vec::with_capacity(entries.len());
    for (i, entry) in entries.iter().enumerate() {
        layout_entries.push(BindGroupLayoutEntry {
            binding: i as _,
            visibility: {
                let mut stages = ShaderStages::NONE;
                if entry.visibility & NSTD_GL_SHADER_STAGE_VERTEX as NSTDUInt8 != 0 {
                    stages |= ShaderStages::VERTEX;
                }
                if entry.visibility & NSTD_GL_SHADER_STAGE_FRAGMENT as NSTDUInt8 != 0 {
                    stages |= ShaderStages::FRAGMENT;
                }
                if entry.visibility & NSTD_GL_SHADER_STAGE_COMPUTE as NSTDUInt8 != 0 {
                    stages |= ShaderStages::COMPUTE;
                }
                stages
            },
            ty: entry.binding_type.into(),
            count: None,
        });
        bind_group_entries.push(BindGroupEntry {
            binding: i as _,
            resource: entry.resource.into(),
        });
    }
    // Create the bind group layout.
    let layout_desc = BindGroupLayoutDescriptor {
        label: None,
        entries: &layout_entries,
    };
    let layout = renderer
        .renderer
        .device
        .create_bind_group_layout(&layout_desc);
    // Create the bind group.
    let bind_group_desc = BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &bind_group_entries,
    };
    let bind_group = renderer.renderer.device.create_bind_group(&bind_group_desc);
    // Construct the bind group.
    NSTDGLBindGroup {
        bind_group: Box::new(BindGroup { bind_group, layout }),
    }
}

/// Makes a bind group active for the given render pass.
///
/// # Parameters:
///
/// - `const NSTDGLBindGroup *bind_group` - The group of bindings to use.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
///
/// - `NSTDUInt32 index` - The index to bind the bind group to.
#[inline]
#[nstdapi]
pub fn nstd_gl_bind_group_bind<'a: 'b, 'b>(
    bind_group: &'a NSTDGLBindGroup,
    render_pass: &mut NSTDGLRenderPass<'b>,
    index: NSTDUInt32,
) {
    render_pass.set_bind_group(index, &bind_group.bind_group.bind_group, &[]);
}

/// Frees an instance of `NSTDGLBindGroup`.
///
/// # Parameters:
///
/// - `NSTDGLBindGroup bind_group` - The bind group to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_bind_group_free(bind_group: NSTDGLBindGroup) {}