//! An application event loop.
pub mod events;
pub mod handle;
use self::events::NSTDAppEvents;
use winit::event_loop::EventLoop;

/// Application data that cannot be stored on the stack.
#[derive(Debug, Default)]
struct App {
    /// The underlying [winit] event loop.
    event_loop: EventLoop<()>,
}

/// An application event loop.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDApp {
    /// The application event callback function pointers.
    events: NSTDAppEvents,
    /// Application data that cannot live on the stack.
    data: Box<App>,
}

/// Creates a new `nstd` application.
///
/// # Note
///
/// An `NSTDApp` can only be created once on the "main" thread. Attempting to recreate an `NSTDApp`
/// after one has already been created will result in a panic.
///
/// # Returns
///
/// `NSTDApp app` - The new `NSTDApp`.
///
/// # Panics
///
/// This function must be called on the "main" thread, otherwise a panic may occurr.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_new() -> NSTDApp {
    NSTDApp {
        events: NSTDAppEvents::new(),
        data: Box::default(),
    }
}

/// Returns a mutable reference to an `NSTDApp`'s event table.
///
/// # Parameters:
///
/// - `NSTDApp *app` - A pointer to the `nstd` app.
///
/// # Returns
///
/// `NSTDAppEvents *events` - A pointer to the application event table.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_events(app: &mut NSTDApp) -> &mut NSTDAppEvents {
    &mut app.events
}

/// Runs an `NSTDApp`'s event loop.
///
/// # Note
///
/// This function will take full control of the current thread and never return.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application to run.
///
/// # Safety
///
/// This function's caller must guarantee validity of the `app`'s event callbacks.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_app_run(app: NSTDApp) -> ! {
    // Dispatch the `start` event.
    if let Some(start) = app.events.start {
        start(&*app.data.event_loop);
    }
    // Run the winit event loop.
    app.data.event_loop.run(|_, _, _| {})
}

/// Frees an instance of `NSTDApp`. The application's event loop must not be ran after this is
/// called.
///
/// # Parameters:
///
/// - `NSTDApp app` - The `nstd` application.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_app_free(app: NSTDApp) {}
