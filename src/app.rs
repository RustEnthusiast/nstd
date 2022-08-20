//! An application event loop.
pub mod data;
pub mod events;
pub mod handle;
use self::{
    data::{NSTDAppControlFlow, NSTDAppData},
    events::{NSTDAppEvents, NSTDKey, NSTDMouseInput, NSTDScrollDelta, NSTDTouchState},
    handle::NSTDAppHandle,
};
use crate::NSTDAnyMut;
use winit::{
    event::{DeviceEvent, ElementState, Event, WindowEvent},
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
                // A mouse device was moved.
                DeviceEvent::MouseMotion { delta } => {
                    if let Some(mouse_moved) = app.events.mouse_moved {
                        mouse_moved(&app_data, &device_id, delta.0, -delta.1);
                    }
                }
                // A scroll wheel was scrolled.
                DeviceEvent::MouseWheel { delta } => {
                    if let Some(mouse_scrolled) = app.events.mouse_scrolled {
                        let (x, y, delta_t) = NSTDScrollDelta::from_winit(delta);
                        mouse_scrolled(&app_data, &device_id, x, y, delta_t);
                    }
                }
                // There was motion on some analog axis.
                DeviceEvent::Motion { axis, value } => {
                    if let Some(axis_motion) = app.events.axis_motion {
                        axis_motion(&app_data, &device_id, &axis, value);
                    }
                }
                // A button's state was changed.
                DeviceEvent::Button { button, state } => {
                    if let Some(button_input) = app.events.button_input {
                        let is_down = state == ElementState::Pressed;
                        button_input(&app_data, &device_id, &button, is_down.into());
                    }
                }
                // There was some keyboard input.
                DeviceEvent::Key(input) => {
                    if let Some(key_input) = app.events.key_input {
                        let key = NSTDKey::from_winit(input.virtual_keycode);
                        let is_down = input.state == ElementState::Pressed;
                        key_input(&app_data, &device_id, key, input.scancode, is_down.into());
                    }
                }
                _ => (),
            },
            // A window event was received.
            Event::WindowEvent { window_id, event } => match event {
                // A window's scale factor has changed.
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    if let Some(window_dpi_changed) = app.events.window_dpi_changed {
                        window_dpi_changed(
                            &app_data,
                            &window_id,
                            scale_factor,
                            &mut new_inner_size.width,
                            &mut new_inner_size.height,
                        );
                    }
                }
                // A window was resized.
                WindowEvent::Resized(size) => {
                    if let Some(window_resized) = app.events.window_resized {
                        window_resized(&app_data, &window_id, size.width, size.height);
                    }
                }
                // A window was moved.
                WindowEvent::Moved(pos) => {
                    if let Some(window_moved) = app.events.window_moved {
                        window_moved(&app_data, &window_id, pos.x, pos.y);
                    }
                }
                // A window's focus has changed.
                WindowEvent::Focused(is_focused) => {
                    if let Some(window_focus_changed) = app.events.window_focus_changed {
                        window_focus_changed(&app_data, &window_id, is_focused.into());
                    }
                }
                // A window received mouse button input.
                WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    ..
                } => {
                    if let Some(window_mouse_input) = app.events.window_mouse_input {
                        let input = NSTDMouseInput::from_winit(button);
                        let is_down = (state == ElementState::Pressed).into();
                        window_mouse_input(&app_data, &window_id, &device_id, &input, is_down);
                    }
                }
                // A window received key input.
                WindowEvent::KeyboardInput {
                    device_id, input, ..
                } => {
                    if let Some(window_key_input) = app.events.window_key_input {
                        let key = NSTDKey::from_winit(input.virtual_keycode);
                        let is_down = (input.state == ElementState::Pressed).into();
                        let scancode = input.scancode;
                        window_key_input(&app_data, &window_id, &device_id, key, scancode, is_down);
                    }
                }
                // A window received character input.
                WindowEvent::ReceivedCharacter(chr) => {
                    if let Some(window_received_char) = app.events.window_received_char {
                        window_received_char(&app_data, &window_id, chr.into());
                    }
                }
                // A window was scrolled.
                WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                    ..
                } => {
                    if let Some(window_scrolled) = app.events.window_scrolled {
                        let (x, y, delta_t) = NSTDScrollDelta::from_winit(delta);
                        let touch = NSTDTouchState::from_winit(phase);
                        window_scrolled(&app_data, &window_id, &device_id, x, y, delta_t, touch);
                    }
                }
                // The cursor was moved over a window.
                WindowEvent::CursorMoved {
                    device_id,
                    position: pos,
                    ..
                } => {
                    if let Some(window_cursor_moved) = app.events.window_cursor_moved {
                        window_cursor_moved(&app_data, &window_id, &device_id, pos.x, pos.y);
                    }
                }
                // The cursor entered a window.
                WindowEvent::CursorEntered { device_id } => {
                    if let Some(window_cursor_entered) = app.events.window_cursor_entered {
                        window_cursor_entered(&app_data, &window_id, &device_id);
                    }
                }
                // The cursor left a window.
                WindowEvent::CursorLeft { device_id } => {
                    if let Some(window_cursor_left) = app.events.window_cursor_left {
                        window_cursor_left(&app_data, &window_id, &device_id);
                    }
                }
                // A window requests closing.
                WindowEvent::CloseRequested => {
                    if let Some(window_close_requested) = app.events.window_close_requested {
                        window_close_requested(&app_data, &window_id);
                    }
                }
                // A window was permanently closed.
                WindowEvent::Destroyed => {
                    if let Some(window_closed) = app.events.window_closed {
                        window_closed(&app_data, &window_id);
                    }
                }
                _ => (),
            },
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

/// Signals an `NSTDApp`'s event loop to exit.
///
/// # Parameters:
///
/// - `const NSTDAppData *app` - The application data received from an event.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_exit(app: &NSTDAppData) {
    app.control_flow
        .set(NSTDAppControlFlow::NSTD_APP_CONTROL_FLOW_EXIT);
}
