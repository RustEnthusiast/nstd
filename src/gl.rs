//! The low level graphics library.
pub mod bind_group;
pub mod buffer;
pub mod frame;
pub mod render_pass;
pub mod sampler;
pub mod shader;
pub mod texture;
use crate::{alloc::CBox, core::result::NSTDResult, window::NSTDWindow, NSTDFloat64, NSTDUInt32};
use nstdapi::nstdapi;
use pollster::FutureExt;
use wgpu::{
    Backends, Color, Device, DeviceDescriptor, Instance, InstanceDescriptor, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

/// Represents an RGBA color value.
#[nstdapi]
#[derive(Clone, Copy, PartialEq)]
pub struct NSTDGLColor {
    /// The red color value.
    pub r: NSTDFloat64,
    /// The green color value.
    pub g: NSTDFloat64,
    /// The blue color value.
    pub b: NSTDFloat64,
    /// The alpha color value.
    pub a: NSTDFloat64,
}
impl NSTDGLColor {
    /// Converts an [NSTDGLColor] into a `wgpu` [Color].
    #[inline]
    const fn as_wgpu(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

/// Describes an error returned by an `nstd.gl` function.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLError {
    /// No error occurred.
    NSTD_GL_ERROR_NONE,
    /// Allocating memory failed.
    NSTD_GL_ERROR_OUT_OF_MEMORY,
    /// A canvas could not be created for a web window.
    NSTD_GL_ERROR_CANVAS_NOT_CREATED,
    /// A rendering surface could not be created.
    NSTD_GL_ERROR_SURFACE_NOT_CREATED,
    /// A GPU device adapter could not be acquired.
    NSTD_GL_ERROR_ADAPTER_NOT_FOUND,
    /// A GPU device handle could not be acquired.
    NSTD_GL_ERROR_DEVICE_NOT_FOUND,
}

/// Represents a rendering backend.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLBackend {
    /// Unknown backend.
    ///
    /// When creating a rendering backend instance, this value will be useful for enabling use of
    /// all rendering backends.
    NSTD_GL_BACKEND_UNKNOWN,
    /// The [Vulkan](https://en.wikipedia.org/wiki/Vulkan) API.
    ///
    /// This backend should be supported on most modern platforms.
    NSTD_GL_BACKEND_VULKAN,
    /// The [OpenGL](https://en.wikipedia.org/wiki/OpenGL) API.
    ///
    /// This backend is currently "unsupported".
    NSTD_GL_BACKEND_OPENGL,
    /// The [Direct 3D 11](https://en.wikipedia.org/wiki/Direct3D#Direct3D_11) API.
    ///
    /// This backend is supported on Windows 7+.
    NSTD_GL_BACKEND_DX11,
    /// The [Direct 3D 12](https://en.wikipedia.org/wiki/Direct3D#Direct3D_12) API.
    ///
    /// This backend is supported on Windows 10+.
    NSTD_GL_BACKEND_DX12,
    /// The [Metal](https://en.wikipedia.org/wiki/Metal_(API)) API.
    ///
    /// This backend is supported on Apple systems.
    NSTD_GL_BACKEND_METAL,
    /// The [WebGPU](https://en.wikipedia.org/wiki/WebGPU) API.
    ///
    /// This backend is supported by the web through WebAssembly.
    NSTD_GL_BACKEND_WEBGPU,
}
impl From<NSTDGLBackend> for Backends {
    /// Converts an [NSTDGLBackend] into a [Backends] object.
    fn from(value: NSTDGLBackend) -> Self {
        match value {
            NSTDGLBackend::NSTD_GL_BACKEND_UNKNOWN => Self::all(),
            NSTDGLBackend::NSTD_GL_BACKEND_VULKAN => Self::VULKAN,
            NSTDGLBackend::NSTD_GL_BACKEND_OPENGL => Self::GL,
            NSTDGLBackend::NSTD_GL_BACKEND_DX11 => Self::DX11,
            NSTDGLBackend::NSTD_GL_BACKEND_DX12 => Self::DX12,
            NSTDGLBackend::NSTD_GL_BACKEND_METAL => Self::METAL,
            NSTDGLBackend::NSTD_GL_BACKEND_WEBGPU => Self::BROWSER_WEBGPU,
        }
    }
}

/// A power preference.
///
/// This type is used for querying drawing devices.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLPowerPreference {
    /// No power preference.
    NSTD_GL_POWER_PREFERENCE_NONE,
    /// A low power preference.
    NSTD_GL_POWER_PREFERENCE_LOW,
    /// A high power preference.
    NSTD_GL_POWER_PREFERENCE_HIGH,
}
impl From<NSTDGLPowerPreference> for PowerPreference {
    /// Converts an [NSTDGLPowerPreference] into a [PowerPreference].
    #[inline]
    fn from(value: NSTDGLPowerPreference) -> Self {
        match value {
            NSTDGLPowerPreference::NSTD_GL_POWER_PREFERENCE_NONE => Self::default(),
            NSTDGLPowerPreference::NSTD_GL_POWER_PREFERENCE_LOW => Self::LowPower,
            NSTDGLPowerPreference::NSTD_GL_POWER_PREFERENCE_HIGH => Self::HighPerformance,
        }
    }
}

/// Represents a surface's presentation mode.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDGLPresentationMode {
    /// Attempts to choose a non-VSync presentation mode automatically.
    ///
    /// Chooses immediate -> mailbox -> fifo based on availability.
    NSTD_GL_PRESENTATION_MODE_AUTO,
    /// Attempts to choose a VSync presentation mode automatically.
    ///
    /// Chooses fifo relaxed -> fifo based on availability.
    NSTD_GL_PRESENTATION_MODE_AUTO_VSYNC,
    /// Frames are kept in a first-in-first-out queue, every vertical blanking period a frame is
    /// popped off of this queue and displayed. If a frame is not ready to be displayed, it will
    /// present the same frame until the next vertical blanking period.
    ///
    /// Traditionally called "VSync".
    ///
    /// Supported on all platforms.
    NSTD_GL_PRESENTATION_MODE_FIFO,
    /// Frames are kept in a first-in-first-out queue, every vertical blanking period a frame is
    /// popped off of this queue and displayed. If a frame is not ready to be displayed, it will
    /// present the same frame until there is a frame in the queue, immediately popping the frame
    /// from the queue.
    ///
    /// Traditionally called "Adaptive VSync".
    ///
    /// Supported on AMD on Vulkan.
    NSTD_GL_PRESENTATION_MODE_FIFO_RELAXED,
    /// Frames are not queued at all. When a present command is executed, the presented image is
    /// immediately swapped onto the front buffer.
    ///
    /// Traditionally called "VSync off".
    ///
    /// Supported on most platforms apart from older Direct3D 12 and Wayland.
    NSTD_GL_PRESENTATION_MODE_IMMEDIATE,
    /// Frames are kept in a single-frame queue, every vertical blanking period a frame is popped
    /// off of this queue and displayed. If a frame is not ready to be displayed, it will present
    /// the same frame until the next vertical blanking period.
    ///
    /// Traditionally called "Fast VSync".
    ///
    /// Supported on Direct3D 11/12, Nvidia on Vulkan, and Wayland on Vulkan.
    NSTD_GL_PRESENTATION_MODE_MAILBOX,
}
impl From<NSTDGLPresentationMode> for PresentMode {
    /// Converts an [NSTDGLPresentationMode] into a [PresentMode].
    fn from(value: NSTDGLPresentationMode) -> Self {
        match value {
            NSTDGLPresentationMode::NSTD_GL_PRESENTATION_MODE_AUTO => Self::AutoNoVsync,
            NSTDGLPresentationMode::NSTD_GL_PRESENTATION_MODE_AUTO_VSYNC => Self::AutoVsync,
            NSTDGLPresentationMode::NSTD_GL_PRESENTATION_MODE_FIFO => Self::Fifo,
            NSTDGLPresentationMode::NSTD_GL_PRESENTATION_MODE_FIFO_RELAXED => Self::FifoRelaxed,
            NSTDGLPresentationMode::NSTD_GL_PRESENTATION_MODE_IMMEDIATE => Self::Immediate,
            NSTDGLPresentationMode::NSTD_GL_PRESENTATION_MODE_MAILBOX => Self::Mailbox,
        }
    }
}

/// Describes the creation of an `NSTDGLRenderer`.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDGLRendererDescriptor<'a> {
    /// A reference to the window to create a renderer for.
    pub window: &'a NSTDWindow,
    /// The rendering backend to use.
    pub backend: NSTDGLBackend,
    /// The power preference to use when querying for a drawing device.
    pub power_preference: NSTDGLPowerPreference,
    /// The presentation mode to use for the renderer's surface.
    pub presentation_mode: NSTDGLPresentationMode,
}

/// The renderer.
struct Renderer {
    /// The rendering surface.
    surface: Surface,
    /// The surface configuration object.
    surface_config: SurfaceConfiguration,
    /// The GPU used for drawing.
    device: Device,
    /// A handle to the drawing device.
    device_handle: Queue,
}

/// `nstd.gl`'s renderer.
///
/// This type creates a rendering surface on an `NSTDWindow`.
#[nstdapi]
pub struct NSTDGLRenderer {
    /// The inner renderer.
    renderer: CBox<Renderer>,
}

/// The result type returned from `nstd_gl_renderer_new`.
pub type NSTDGLRendererResult = NSTDResult<NSTDGLRenderer, NSTDGLError>;

/// Creates a new rendering context with a rendering surface and a handle to a drawing device.
///
/// # Parameters:
///
/// - `const NSTDGLRendererDescriptor *desc` - The renderer descriptor.
///
/// # Returns
///
/// `NSTDGLRendererResult renderer` - The new `nstd.gl` renderer on success, or an error code on
/// failure.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - A default GPU adapter could not be found.
///
/// - A default device handle could not be made.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - This operation is called with the Metal backend while not on the "main" thread.
///
/// - In some situations when a default device handle could not be made.
///
/// # Safety
///
/// `desc.window` must remain alive while the returned object is alive.
#[nstdapi]
pub unsafe fn nstd_gl_renderer_new(desc: &NSTDGLRendererDescriptor) -> NSTDGLRendererResult {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::Element;
        use winit::platform::web::WindowExtWebSys;
        // Create a rendering canvas on the web window's body.
        if web_sys::window()
            .and_then(|w| {
                let body = w.document()?.body()?;
                let canvas = Element::from(desc.window.canvas());
                body.append_child(&canvas).ok()
            })
            .is_none()
        {
            return NSTDResult::Err(NSTDGLError::NSTD_GL_ERROR_CANVAS_NOT_CREATED);
        }
    }
    // Create an instance of the rendering backend.
    let instance_desc = InstanceDescriptor {
        backends: desc.backend.into(),
        ..Default::default()
    };
    let instance = Instance::new(instance_desc);
    // Create the rendering surface.
    let surface = match instance.create_surface(&**desc.window) {
        Ok(surface) => surface,
        _ => return NSTDResult::Err(NSTDGLError::NSTD_GL_ERROR_SURFACE_NOT_CREATED),
    };
    // Create the GPU device adapter.
    let adapter_desc = RequestAdapterOptions {
        compatible_surface: Some(&surface),
        power_preference: desc.power_preference.into(),
        force_fallback_adapter: false,
    };
    let adapter = match instance.request_adapter(&adapter_desc).block_on() {
        Some(adapter) => adapter,
        _ => return NSTDResult::Err(NSTDGLError::NSTD_GL_ERROR_ADAPTER_NOT_FOUND),
    };
    // Create a handle to the GPU.
    let device_desc = DeviceDescriptor {
        label: None,
        features: adapter.features(),
        limits: adapter.limits(),
    };
    let (device, device_handle) = match adapter.request_device(&device_desc, None).block_on() {
        Ok(handle) => handle,
        _ => return NSTDResult::Err(NSTDGLError::NSTD_GL_ERROR_DEVICE_NOT_FOUND),
    };
    // Configure the surface.
    let window_size = desc.window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let formats = surface_caps.formats;
    let format = *formats.iter().find(|f| f.is_srgb()).unwrap_or(&formats[0]);
    let surface_config = SurfaceConfiguration {
        width: window_size.width,
        height: window_size.height,
        present_mode: desc.presentation_mode.into(),
        format,
        usage: TextureUsages::RENDER_ATTACHMENT,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: Vec::new(),
    };
    surface.configure(&device, &surface_config);
    // Construct the renderer.
    let renderer = Renderer {
        surface,
        surface_config,
        device,
        device_handle,
    };
    match CBox::new(renderer) {
        Some(renderer) => NSTDResult::Ok(NSTDGLRenderer { renderer }),
        _ => NSTDResult::Err(NSTDGLError::NSTD_GL_ERROR_OUT_OF_MEMORY),
    }
}

/// Resizes a renderer's surface.
///
/// This will have no effect if either `size.width` or `size.height` are zero.
///
/// # Parameters
///
/// - `NSTDGLRenderer *renderer` - The renderer.
///
/// - `NSTDUInt32 width` - The new width to give the renderer's surface.
///
/// - `NSTDUInt32 height` - The new height to give the renderer's surface.
#[nstdapi]
pub fn nstd_gl_renderer_resize(
    renderer: &mut NSTDGLRenderer,
    width: NSTDUInt32,
    height: NSTDUInt32,
) {
    // Make sure neither width or height are 0.
    if width != 0 && height != 0 {
        let renderer = &mut renderer.renderer;
        renderer.surface_config.width = width;
        renderer.surface_config.height = height;
        renderer
            .surface
            .configure(&renderer.device, &renderer.surface_config);
    }
}

/// Frees an instance of `NSTDGLRenderer`.
///
/// # Parameters:
///
/// - `NSTDGLRenderer renderer` - The renderer to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_gl_renderer_free(renderer: NSTDGLRenderer) {}
