#ifndef NSTD_GL_BIND_GROUP_H
#define NSTD_GL_BIND_GROUP_H
#include "../core/slice.h"
#include "../nstd.h"
#include "buffer.h"
#include "gl.h"
#include "render_pass.h"
#include "sampler.h"
#include "texture.h"

/// Describes a read/write uniform buffer.
#define NSTD_GL_BUFFER_BINDING_TYPE_UNIFORM 0
/// Describes a possibly read-only storage buffer.
#define NSTD_GL_BUFFER_BINDING_TYPE_STORAGE 1

/// Describes a buffer's binding type.
typedef struct {
    NSTDUInt8 status;
    union {
        /// Determines whether or not the storage buffer is read-only.
        NSTDBool read_only;
    } value;
} NSTDGLBufferBindingType;

/// Describes a sampler's binding type.
typedef enum {
    /// The sampling result is based on a single color value from a texture.
    NSTD_GL_SAMPLER_BINDING_TYPE_UNFILTERED,
    /// The sampling result is based on more than a single color value from a texture.
    NSTD_GL_SAMPLER_BINDING_TYPE_FILTERING
} NSTDGLSamplerBindingType;

/// Sampling returns floats.
#define NSTD_GL_TEXTURE_SAMPLER_TYPE_FLOAT 0
/// Sampling returns signed integers.
#define NSTD_GL_TEXTURE_SAMPLER_TYPE_INT 1
/// Sampling returns unsigned integers.
#define NSTD_GL_TEXTURE_SAMPLER_TYPE_UINT 2

/// Describes a texture sampling type.
typedef struct {
    NSTDUInt8 status;
    union {
        /// Determines whether or not the texture is filterable.
        NSTDBool filterable;
    } value;
} NSTDGLTextureSamplerType;

/// Describes a binding for a GPU memory buffer.
#define NSTD_GL_BINDING_TYPE_BUFFER 0
/// Describes a binding for a texture sampler.
#define NSTD_GL_BINDING_TYPE_SAMPLER 1
/// Describes a binding for a texture.
#define NSTD_GL_BINDING_TYPE_TEXTURE 2

/// Describes a bind group entry's type.
typedef struct {
    NSTDUInt8 status;
    union {
        /// The buffer's binding type.
        NSTDGLBufferBindingType buffer_binding_type;
        /// The sampler's binding type.
        NSTDGLSamplerBindingType sampler_binding_type;
        /// The texture sampler return type.
        NSTDGLTextureSamplerType sample_type;
    } value;
} NSTDGLBindingType;

/// Represents a bind group resource.
typedef struct {
    enum {
        /// Represents a GPU memory buffer binding.
        NSTD_GL_BINDING_RESOURCE_BUFFER,
        /// Represents a texture sampler binding.
        NSTD_GL_BINDING_RESOURCE_SAMPLER,
        /// Represents a texture binding.
        NSTD_GL_BINDING_RESOURCE_TEXTURE
    } status;
    union {
        /// A reference to the buffer to use as a binding resource.
        const NSTDGLBuffer *buffer;
        /// A reference to the texture sampler.
        const NSTDGLSampler *sampler;
        /// A reference to the texture.
        const NSTDGLTexture *texture;
    } value;
} NSTDGLBindingResource;

/// Describes the creation of a bind group entry.
typedef struct {
    /// The entry's binding resource.
    NSTDGLBindingResource resource;
    /// The entry's binding type.
    NSTDGLBindingType binding_type;
    /// A bitset describing which parts of the render pipeline should be able to use the binding.
    NSTDUInt8 visibility;
} NSTDGLBindGroupEntry;

/// Represents group of bindings for a shader.
typedef struct {
    /// Heap data.
    NSTDAnyMut bind_group;
} NSTDGLBindGroup;

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
NSTDAPI NSTDGLBindGroup
nstd_gl_bind_group_new(const NSTDGLRenderer *renderer, const NSTDSlice *entries);

/// Makes a bind group active for the given render pass.
///
/// # Parameters:
///
/// - `const NSTDGLBindGroup *bind_group` - The group of bindings to use.
///
/// - `NSTDGLRenderPass *render_pass` - The render pass in use.
///
/// - `NSTDUInt32 index` - The index to bind the bind group to.
NSTDAPI void nstd_gl_bind_group_bind(
    const NSTDGLBindGroup *bind_group, NSTDGLRenderPass *render_pass, NSTDUInt32 index
);

/// Frees an instance of `NSTDGLBindGroup`.
///
/// # Parameters:
///
/// - `NSTDGLBindGroup bind_group` - The bind group to free.
NSTDAPI void nstd_gl_bind_group_free(NSTDGLBindGroup bind_group);

#endif
