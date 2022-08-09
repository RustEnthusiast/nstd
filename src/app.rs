//! An application event loop.
pub mod data;
pub mod events;
pub mod handle;
use self::{data::NSTDAppData, events::NSTDAppEvents, handle::NSTDAppHandle};
use crate::NSTDAnyMut;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::EventLoop,
};

/// An application event loop.
#[repr(C)]
pub struct NSTDApp {
    /// The application event callback function pointers.
    events: NSTDAppEvents,
    /// The underlying event loop.
    event_loop: Box<EventLoop<()>>,
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
/// This function must be called on the "main" thread, otherwise a panic may occur.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_new() -> NSTDApp {
    NSTDApp {
        events: NSTDAppEvents::default(),
        event_loop: Box::default(),
    }
}

/// Returns a handle to an `NSTDApp`'s event loop.
///
/// # Parameters:
///
/// - `const NSTDApp *app` - The `nstd` application.
///
/// # Returns
///
/// `NSTDAppHandle handle` - A handle to the application's event loop.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_handle(app: &NSTDApp) -> NSTDAppHandle {
    &app.event_loop
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
/// - `NSTDAnyMut data` - Custom user data to pass to each app event.
///
/// # Safety
///
/// This function's caller must guarantee validity of the `app`'s event callbacks.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_app_run(app: NSTDApp, data: NSTDAnyMut) -> ! {
    // Dispatch the `start` event.
    if let Some(start) = app.events.start {
        let start_app_data = NSTDAppData::new(&app.event_loop, data);
        start(&start_app_data);
    }
    // Run the winit event loop.
    app.event_loop.run(move |event, handle, control_flow| {
        // Instantiate a new instance of `NSTDAppData`.
        let app_data = NSTDAppData::new(handle, data);
        // Dispatch events.
        match event {
            // All other events have been processed.
            Event::MainEventsCleared => {
                if let Some(update) = app.events.update {
                    update(&app_data);
                }
            }
            // A device event was received.
            Event::DeviceEvent { device_id, event } => match event {
                // A device was connected to the system.
                DeviceEvent::Added => {
                    if let Some(device_added) = app.events.device_added {
                        device_added(&app_data, &device_id);
                    }
                }
                // A device was disconnected from the system.
                DeviceEvent::Removed => {
                    if let Some(device_removed) = app.events.device_removed {
                        device_removed(&app_data, &device_id);
                    }
                }
                _ => (),
            },
            // A window requests closing.
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } => {
                if let Some(window_close_requested) = app.events.window_close_requested {
                    window_close_requested(&app_data, &window_id);
                }
            }
            // The event loop is being exited.
            Event::LoopDestroyed => {
                if let Some(exit) = app.events.exit {
                    exit(&app_data);
                }
            }
            _ => (),
        }
        // Set the `winit` event loop's control flow.
        *control_flow = app_data.control_flow.into_inner().into();
    })
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
