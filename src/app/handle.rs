//! A handle to the application event loop.
use winit::event_loop::EventLoopWindowTarget;

/// A handle to the application event loop.
pub type NSTDAppHandle<'a> = &'a EventLoopWindowTarget<()>;
