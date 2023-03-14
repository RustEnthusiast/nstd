#ifndef NSTD_GL_SHADER_H
#define NSTD_GL_SHADER_H
#include "../core/slice.h"
#include "../core/str.h"
#include "../nstd.h"
#include "gl.h"
#include "render_pass.h"

/// An enumeration of each programmable stage of the rendering pipeline.
typedef enum {
    /// The vertex shader.
    NSTD_GL_SHADER_STAGE_VERTEX,
    /// The fragment shader.
    NSTD_GL_SHADER_STAGE_FRAGMENT,
    /// The compute shader.
    NSTD_GL_SHADER_STAGE_COMPUTE
} NSTDGLShaderStage;

/// Describes an `NSTDGLShaderSource` variant.
typedef enum {
    /// WebGPU WGSL shader source.
    NSTD_GL_SHADER_SOURCE_STATUS_WGSL,
    /// Khronos SPIR-V binary.
    NSTD_GL_SHADER_SOURCE_STATUS_SPIRV,
    /// Khronos OpenGL GLSL source.
    NSTD_GL_SHADER_SOURCE_STATUS_GLSL
} NSTDGLShaderSourceStatus;

/// A structure containing a shader's source.
typedef struct {
    /// The source language variant.
    NSTDGLShaderSourceStatus status;
    union {
        /// WebGPU WGSL shader source.
        const NSTDStr *wgsl;
        /// Khronos SPIR-V binary.
        const NSTDSlice *spirv;
        /// Khronos OpenGL GLSL source.
        struct {
            /// The GLSL source.
            const NSTDStr *glsl;
            /// The shader's stage in the rendering pipeline.
            NSTDGLShaderStage stage;
        };
    };
} NSTDGLShaderSource;

/// A GPU shader module.
typedef NSTDAnyMut NSTDGLShaderModule;

/// Describes the stepping mode of a vertex buffer within a shader.
typedef enum {
    /// Vertex data is indexed by vertex.
    NSTD_GL_VERTEX_STEP_MODE_VERTEX,
    /// Vertex data is indexed by instance.
    NSTD_GL_VERTEX_STEP_MODE_INSTANCE
} NSTDGLVertexStepMode;

/// Represents a vertex attribute format within a shader.
typedef enum {
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
    NSTD_GL_VERTEX_FORMAT_FLOAT64X4
} NSTDGLVertexFormat;

/// Represents a single vertex attribute.
typedef struct {
    /// The vertex attribute format.
    NSTDGLVertexFormat format;
    /// The offset of the vertex attribute.
    NSTDUInt64 offset;
    /// The location of the vertex attribute.
    NSTDUInt32 location;
} NSTDGLVertexAttribute;

/// Describes how a shader's vertex buffer is interpreted.
typedef struct {
    /// The number of bytes between elements of this buffer.
    NSTDUInt64 stride;
    /// The vertex buffer's step mode.
    NSTDGLVertexStepMode step_mode;
    /// The buffer's attributes.
    ///
    /// This is a slice of [NSTDGLVertexAttribute].
    const NSTDSlice *attributes;
} NSTDGLVertexBufferLayout;

/// Describes the creation of a GPU shader program.
typedef struct {
    /// The vertex shader module.
    const NSTDGLShaderModule *vertex;
    /// The fragment shader module.
    const NSTDGLShaderModule *fragment;
    /// The shader's vertex buffer layouts.
    ///
    /// A slice of [NSTDGLVertexBufferLayout].
    const NSTDSlice *buffers;
} NSTDGLShaderDescriptor;

/// A GPU shader program.
typedef NSTDAnyMut NSTDGLShader;

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
NSTDAPI NSTDGLShaderModule
nstd_gl_shader_module_new(const NSTDGLRenderer *renderer, const NSTDGLShaderSource *source);

/// Frees an instance of [NSTDGLShaderModule].
///
/// # Parameters:
///
/// - `NSTDGLShaderModule module` - The shader module.
NSTDAPI void nstd_gl_shader_module_free(NSTDGLShaderModule module);

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
NSTDAPI NSTDGLShader
nstd_gl_shader_new(const NSTDGLRenderer *renderer, const NSTDGLShaderDescriptor *desc);

/// Makes a shader program active for the given render pass.
///
/// # Parameters:
///
/// - `const NSTDGLShader *shader` - The shader to bind.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass.
NSTDAPI void nstd_gl_shader_bind(const NSTDGLShader *shader, NSTDGLRenderPass *render_pass);

/// Frees an instance of `NSTDGLShader`.
///
/// # Parameters:
///
/// - `NSTDGLShader shader` - The GPU shader program to free.
NSTDAPI void nstd_gl_shader_free(NSTDGLShader shader);

#endif
