//! GPU shader programs.
use super::{render_pass::NSTDGLRenderPass, NSTDGLRenderer};
use crate::{
    core::{slice::NSTDSlice, str::NSTDStr},
    NSTDUInt32, NSTDUInt64,
};
use naga::ShaderStage;
use nstdapi::nstdapi;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, MultisampleState,
    PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModule, ShaderModuleDescriptor, ShaderSource, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexState, VertexStepMode,
};

/// An enumeration of each programmable stage of the rendering pipeline.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLShaderStage {
    /// The vertex shader.
    NSTD_GL_SHADER_STAGE_VERTEX,
    /// The fragment shader.
    NSTD_GL_SHADER_STAGE_FRAGMENT,
    /// The compute shader.
    NSTD_GL_SHADER_STAGE_COMPUTE,
}
impl From<&NSTDGLShaderStage> for ShaderStage {
    /// Converts an [NSTDGLShaderStage] into a [ShaderStage].
    #[inline]
    fn from(value: &NSTDGLShaderStage) -> Self {
        match *value {
            NSTDGLShaderStage::NSTD_GL_SHADER_STAGE_VERTEX => Self::Vertex,
            NSTDGLShaderStage::NSTD_GL_SHADER_STAGE_FRAGMENT => Self::Fragment,
            NSTDGLShaderStage::NSTD_GL_SHADER_STAGE_COMPUTE => Self::Compute,
        }
    }
}

/// Describes an `NSTDGLShaderSource` variant.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLShaderSourceStatus {
    /// WebGPU WGSL shader source.
    NSTD_GL_SHADER_SOURCE_STATUS_WGSL,
    /// Khronos SPIR-V binary.
    NSTD_GL_SHADER_SOURCE_STATUS_SPIRV,
    /// Khronos OpenGL GLSL source.
    NSTD_GL_SHADER_SOURCE_STATUS_GLSL,
}

/// A structure containing a shader's source.
#[nstdapi]
#[derive(Clone, Copy)]
pub enum NSTDGLShaderSource<'a> {
    /// WebGPU WGSL shader source.
    WGSL(&'a NSTDStr),
    /// Khronos SPIR-V binary.
    SPIRV(&'a NSTDSlice),
    /// Khronos OpenGL GLSL source.
    GLSL {
        /// The GLSL source.
        glsl: &'a NSTDStr,
        /// The shader's stage in the rendering pipeline.
        stage: NSTDGLShaderStage,
    },
}
impl NSTDGLShaderSource<'_> {
    /// Converts [NSTDGLShaderSource] into [wgpu] [ShaderSource].
    ///
    /// # Panics
    ///
    /// This operation will panic in the following situations:
    ///
    /// - `wgsl`'s source length in bytes exceeds `NSTDInt`'s max value.
    ///
    /// - `spirv`'s stride is not 4.
    ///
    /// - `spirv`'s length in bytes exceeds `NSTDInt`'s max value.
    ///
    /// - `glsl`'s source length in bytes exceeds `NSTDInt`'s max value.
    ///
    /// # Safety
    ///
    /// `wgsl`, `spirv`, and `glsl`'s data must be valid.
    unsafe fn as_wgpu(&self) -> ShaderSource {
        match self {
            Self::WGSL(wgsl) => ShaderSource::Wgsl(wgsl.as_str().into()),
            Self::SPIRV(spirv) => ShaderSource::SpirV(spirv.as_slice().into()),
            Self::GLSL { glsl, stage } => ShaderSource::Glsl {
                shader: glsl.as_str().into(),
                stage: stage.into(),
                defines: Default::default(),
            },
        }
    }
}

/// A GPU shader module.
pub type NSTDGLShaderModule = Box<ShaderModule>;

/// Describes the stepping mode of a vertex buffer within a shader.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLVertexStepMode {
    /// Vertex data is indexed by vertex.
    NSTD_GL_VERTEX_STEP_MODE_VERTEX,
    /// Vertex data is indexed by instance.
    NSTD_GL_VERTEX_STEP_MODE_INSTANCE,
}
impl From<NSTDGLVertexStepMode> for VertexStepMode {
    /// Converts an [NSTDGLVertexStepMode] into a [VertexStepMode].
    #[inline]
    fn from(value: NSTDGLVertexStepMode) -> Self {
        match value {
            NSTDGLVertexStepMode::NSTD_GL_VERTEX_STEP_MODE_VERTEX => Self::Vertex,
            NSTDGLVertexStepMode::NSTD_GL_VERTEX_STEP_MODE_INSTANCE => Self::Instance,
        }
    }
}

/// Represents a vertex attribute format within a shader.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLVertexFormat {
    /// Two unsigned bytes.
    NSTD_GL_VERTEX_FORMAT_UINT8X2,
    /// Four unsigned bytes.
    NSTD_GL_VERTEX_FORMAT_UINT8X4,
    /// Two signed bytes.
    NSTD_GL_VERTEX_FORMAT_INT8X2,
    /// Four signed bytes.
    NSTD_GL_VERTEX_FORMAT_INT8X4,
    /// Two unsigned bytes converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_UNORM8X2,
    /// Four unsigned bytes converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_UNORM8X4,
    /// Two signed bytes converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_NORM8X2,
    /// Four signed bytes converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_NORM8X4,
    /// Two unsigned shorts.
    NSTD_GL_VERTEX_FORMAT_UINT16X2,
    /// Four unsigned shorts.
    NSTD_GL_VERTEX_FORMAT_UINT16X4,
    /// Two signed shorts.
    NSTD_GL_VERTEX_FORMAT_INT16X2,
    /// Four signed shorts.
    NSTD_GL_VERTEX_FORMAT_INT16X4,
    /// Two unsigned shorts converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_UNORM16X2,
    /// Four unsigned shorts converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_UNORM16X4,
    /// Two signed shorts converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_NORM16X2,
    /// Four signed shorts converted to a normalized float.
    NSTD_GL_VERTEX_FORMAT_NORM16X4,
    /// A single unsigned integer.
    NSTD_GL_VERTEX_FORMAT_UINT32,
    /// Two unsigned integers.
    NSTD_GL_VERTEX_FORMAT_UINT32X2,
    /// Three unsigned integers.
    NSTD_GL_VERTEX_FORMAT_UINT32X3,
    /// Four unsigned integers.
    NSTD_GL_VERTEX_FORMAT_UINT32X4,
    /// A single signed integer.
    NSTD_GL_VERTEX_FORMAT_INT32,
    /// Two signed integers.
    NSTD_GL_VERTEX_FORMAT_INT32X2,
    /// Three signed integers.
    NSTD_GL_VERTEX_FORMAT_INT32X3,
    /// Four signed integers.
    NSTD_GL_VERTEX_FORMAT_INT32X4,
    /// Two 16-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT16X2,
    /// Four 16-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT16X4,
    /// A single 32-bit float.
    NSTD_GL_VERTEX_FORMAT_FLOAT32,
    /// Two 32-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT32X2,
    /// Three 32-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT32X3,
    /// Four 32-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT32X4,
    /// A single 64-bit float.
    NSTD_GL_VERTEX_FORMAT_FLOAT64,
    /// Two 64-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT64X2,
    /// Three 64-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT64X3,
    /// Four 64-bit floats.
    NSTD_GL_VERTEX_FORMAT_FLOAT64X4,
}
impl From<NSTDGLVertexFormat> for VertexFormat {
    /// Converts an [NSTDGLVertexFormat] into a [VertexFormat].
    fn from(value: NSTDGLVertexFormat) -> Self {
        match value {
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT8X2 => VertexFormat::Uint8x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT8X4 => VertexFormat::Uint8x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT8X2 => VertexFormat::Sint8x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT8X4 => VertexFormat::Sint8x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UNORM8X2 => VertexFormat::Unorm8x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UNORM8X4 => VertexFormat::Unorm8x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_NORM8X2 => VertexFormat::Snorm8x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_NORM8X4 => VertexFormat::Snorm8x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT16X2 => VertexFormat::Uint16x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT16X4 => VertexFormat::Uint16x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT16X2 => VertexFormat::Sint16x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT16X4 => VertexFormat::Sint16x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UNORM16X2 => VertexFormat::Unorm16x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UNORM16X4 => VertexFormat::Unorm16x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_NORM16X2 => VertexFormat::Snorm16x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_NORM16X4 => VertexFormat::Snorm16x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT32 => VertexFormat::Uint32,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT32X2 => VertexFormat::Uint32x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT32X3 => VertexFormat::Uint32x3,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_UINT32X4 => VertexFormat::Uint32x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT32 => VertexFormat::Sint32,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT32X2 => VertexFormat::Sint32x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT32X3 => VertexFormat::Sint32x3,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_INT32X4 => VertexFormat::Sint32x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT16X2 => VertexFormat::Float16x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT16X4 => VertexFormat::Float16x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT32 => VertexFormat::Float32,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT32X2 => VertexFormat::Float32x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT32X3 => VertexFormat::Float32x3,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT32X4 => VertexFormat::Float32x4,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT64 => VertexFormat::Float64,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT64X2 => VertexFormat::Float64x2,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT64X3 => VertexFormat::Float64x3,
            NSTDGLVertexFormat::NSTD_GL_VERTEX_FORMAT_FLOAT64X4 => VertexFormat::Float64x4,
        }
    }
}

/// Represents a single vertex attribute.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NSTDGLVertexAttribute {
    /// The vertex attribute format.
    pub format: NSTDGLVertexFormat,
    /// The offset of the vertex attribute.
    pub offset: NSTDUInt64,
    /// The location of the vertex attribute.
    pub location: NSTDUInt32,
}
impl From<&NSTDGLVertexAttribute> for VertexAttribute {
    /// Converts an [NSTDGLVertexAttribute] into a [VertexAttribute].
    #[inline]
    fn from(value: &NSTDGLVertexAttribute) -> Self {
        Self {
            format: value.format.into(),
            offset: value.offset as _,
            shader_location: value.location as _,
        }
    }
}

/// Describes how a shader's vertex buffer is interpreted.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDGLVertexBufferLayout<'a> {
    /// The number of bytes between elements of this buffer.
    pub stride: NSTDUInt64,
    /// The vertex buffer's step mode.
    pub step_mode: NSTDGLVertexStepMode,
    /// The buffer's attributes.
    ///
    /// This is a slice of [NSTDGLVertexAttribute].
    pub attributes: &'a NSTDSlice,
}

/// Describes the creation of a GPU shader program.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDGLShaderDescriptor<'a> {
    /// The vertex shader module.
    pub vertex: &'a NSTDGLShaderModule,
    /// The fragment shader module.
    pub fragment: Option<&'a NSTDGLShaderModule>,
    /// The shader's vertex buffer layouts.
    ///
    /// A slice of [NSTDGLVertexBufferLayout].
    pub buffers: &'a NSTDSlice,
}

/// A GPU shader program.
pub type NSTDGLShader = Box<RenderPipeline>;

/// Creates a new compiled shader object from a shader source object.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer to use to create the shader.
///
/// - `const NSTDGLShaderSource *source` - The shader source object.
///
/// # Returns
///
/// `NSTDGLShaderModule module` - The new compiled shader object.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `source.wgsl`'s source length in bytes exceeds `NSTDInt`'s max value.
///
/// - `source.spirv`'s stride is not 4.
///
/// - `source.spirv`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// - `source.glsl`'s source length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// `source.wgsl`, `spirv`, and `glsl`'s data must be valid.
#[inline]
#[nstdapi]
pub unsafe fn nstd_gl_shader_module_new(
    renderer: &NSTDGLRenderer,
    source: &NSTDGLShaderSource,
) -> NSTDGLShaderModule {
    let module_desc = ShaderModuleDescriptor {
        label: None,
        source: source.as_wgpu(),
    };
    Box::new(renderer.renderer.device.create_shader_module(module_desc))
}

/// Frees an instance of [NSTDGLShaderModule].
///
/// # Parameters:
///
/// - `NSTDGLShaderModule module` - The shader module.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_shader_module_free(module: NSTDGLShaderModule) {}

/// Creates a new GPU shader program using `renderer`.
///
/// # Parameters:
///
/// - `const NSTDGLRenderer *renderer` - The renderer to use to create the shader program.
///
/// - `const NSTDGLShaderDescriptor *desc` - The GPU shader program descriptor.
///
/// # Returns
///
/// `NSTDGLShader shader` - The new GPU shader program.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `desc.buffers`'s stride does not match `NSTDGLVertexBufferLayout`'s size in bytes.
///
/// - `desc.buffers`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// - `desc.buffers.attributes`'s stride does not match `NSTDGLVertexAttribute`'s size in bytes.
///
/// - `desc.buffers.attributes`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// - `desc.buffers`'s data must be properly aligned and valid for reads.
///
/// - `desc.buffers.attributes`'s data must be properly aligned and valid for reads.
#[nstdapi]
pub unsafe fn nstd_gl_shader_new(
    renderer: &NSTDGLRenderer,
    desc: &NSTDGLShaderDescriptor,
) -> NSTDGLShader {
    let renderer = &renderer.renderer;
    // Create the vertex buffer layouts.
    let desc_buffers = desc.buffers.as_slice::<NSTDGLVertexBufferLayout>();
    let mut buffers = Vec::with_capacity(desc_buffers.len());
    let mut attributes = Vec::with_capacity(desc_buffers.len());
    for buffer_layout in desc_buffers {
        let mut vertex_attributes = Vec::new();
        for attribute in buffer_layout.attributes.as_slice::<NSTDGLVertexAttribute>() {
            vertex_attributes.push(attribute.into());
        }
        attributes.push(vertex_attributes);
    }
    for (i, buffer_layout) in desc_buffers.iter().enumerate() {
        buffers.push(VertexBufferLayout {
            array_stride: buffer_layout.stride,
            step_mode: buffer_layout.step_mode.into(),
            attributes: &attributes[i],
        });
    }
    // Create the pipeline layout.
    let pipeline_layout = renderer.device.create_pipeline_layout(&Default::default());
    // Create the render pipeline.
    let targets = [Some(ColorTargetState {
        format: renderer.surface_config.format,
        blend: Some(BlendState::REPLACE),
        write_mask: ColorWrites::ALL,
    })];
    let pipeline_desc = RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: desc.vertex,
            entry_point: "vertex",
            buffers: &buffers,
        },
        fragment: desc.fragment.map(|fragment| FragmentState {
            module: fragment,
            entry_point: "fragment",
            targets: &targets,
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            strip_index_format: None,
            unclipped_depth: false,
            conservative: false,
        },
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        depth_stencil: None,
        multiview: None,
    };
    Box::new(renderer.device.create_render_pipeline(&pipeline_desc))
}

/// Makes a shader program active for the given render pass.
///
/// # Parameters:
///
/// - `const NSTDGLShader *shader` - The shader to bind.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass.
#[inline]
#[nstdapi]
pub fn nstd_gl_shader_bind<'a: 'b, 'b>(
    shader: &'a NSTDGLShader,
    render_pass: &mut NSTDGLRenderPass<'b>,
) {
    render_pass.set_pipeline(shader);
}

/// Frees an instance of `NSTDGLShader`.
///
/// # Parameters:
///
/// - `NSTDGLShader shader` - The GPU shader program to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_shader_free(shader: NSTDGLShader) {}
