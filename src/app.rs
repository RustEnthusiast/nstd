//! An application event loop.
pub mod data;
pub mod display;
pub mod events;
use self::{
    data::{NSTDAppData, NSTDAppHandle},
    display::{NSTDDisplay, NSTDDisplayHandle},
    events::{
        NSTDAppEvents, NSTDDeviceEventFilter, NSTDGamepadAxis, NSTDGamepadButton, NSTDKey,
        NSTDMouseInput, NSTDScrollDelta, NSTDTouchState,
    },
};
use crate::{
    core::{def::NSTDErrorCode, str::NSTDStr},
    heap_ptr::NSTDHeapPtr,
};
use gilrs::{Error::NotImplemented, EventType as GamepadEvent, Gilrs};
use winit::{
    event::{DeviceEvent, ElementState, Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, DeviceEventFilter, EventLoop},
};

/// An application event loop.
#[repr(C)]
pub struct NSTDApp {
    /// The application event callback function pointers.
    events: NSTDAppEvents,
    /// Private app data.
    inner: Box<AppData>,
}

/// Private application data.
struct AppData {
    /// The [winit] event loop.
    event_loop: EventLoop<()>,
    /// The gamepad input handler.
    gil: Gilrs,
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
/// This function may panic in the following situations:
///
/// - This function was not called on the "main" thread.
///
/// - Creating the gamepad input handler fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_new() -> NSTDApp {
    let event_loop = EventLoop::new();
    event_loop.set_device_event_filter(DeviceEventFilter::Never);
    NSTDApp {
        events: NSTDAppEvents::default(),
        inner: Box::new(AppData {
            event_loop,
            gil: match Gilrs::new() {
                Ok(gil) => gil,
                Err(NotImplemented(gil)) => gil,
                _ => panic!("failed to create gamepad event listener"),
            },
        }),
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
    &app.inner.event_loop
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
/// - `NSTDHeapPtr data` - Custom user data to pass to each app event.
///
/// # Safety
///
/// This function's caller must guarantee validity of the `app`'s event callbacks.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_app_run(app: NSTDApp, mut data: NSTDHeapPtr) -> ! {
    let AppData {
        event_loop,
        mut gil,
    } = *app.inner;
    // Run the winit event loop.
    event_loop.run(move |event, handle, control_flow| {
        // Instantiate a new instance of `NSTDAppData`.
        let app_data = &mut NSTDAppData::new(handle, control_flow, &mut data, &mut gil);
        // Dispatch events.
        match event {
            // The event loop was just started.
            Event::NewEvents(StartCause::Init) => {
                if let Some(start) = app.events.start {
                    start(app_data);
                }
            }
            // All other events have been processed.
            Event::MainEventsCleared => {
                // Dispatch gamepad events.
                while let Some(event) = app_data.next_gamepad_event() {
                    match event.event {
                        // A gamepad was connected to the system.
                        GamepadEvent::Connected => {
                            if let Some(gamepad_connected) = app.events.gamepad_connected {
                                gamepad_connected(app_data, &event.id);
                            }
                        }
                        // A gamepad was disconnected from the system.
                        GamepadEvent::Disconnected => {
                            if let Some(gamepad_disconnected) = app.events.gamepad_disconnected {
                                gamepad_disconnected(app_data, &event.id);
                            }
                        }
                        // A gamepad button was pressed.
                        GamepadEvent::ButtonPressed(button, code) => {
                            if let Some(gamepad_button_pressed) = app.events.gamepad_button_pressed
                            {
                                let button = NSTDGamepadButton::from_winit(button);
                                let code = code.into_u32();
                                gamepad_button_pressed(app_data, &event.id, button, code);
                            }
                        }
                        // A gamepad button was released.
                        GamepadEvent::ButtonReleased(button, code) => {
                            if let Some(gamepad_button_released) =
                                app.events.gamepad_button_released
                            {
                                let button = NSTDGamepadButton::from_winit(button);
                                let code = code.into_u32();
                                gamepad_button_released(app_data, &event.id, button, code);
                            }
                        }
                        // A gamepad button's value changed.
                        GamepadEvent::ButtonChanged(button, value, code) => {
                            if let Some(gamepad_input) = app.events.gamepad_input {
                                let button = NSTDGamepadButton::from_winit(button);
                                let code = code.into_u32();
                                gamepad_input(app_data, &event.id, button, code, value);
                            }
                        }
                        // A gamepad axis value has changed.
                        GamepadEvent::AxisChanged(axis, value, code) => {
                            if let Some(gamepad_axis_input) = app.events.gamepad_axis_input {
                                let axis = NSTDGamepadAxis::from_winit(axis);
                                let code = code.into_u32();
                                gamepad_axis_input(app_data, &event.id, axis, code, value);
                            }
                        }
                        _ => (),
                    }
                }
                // Dispatch update event.
                if let Some(update) = app.events.update {
                    update(app_data);
                }
            }
            // A device event was received.
            Event::DeviceEvent { device_id, event } => match event {
                // A device was connected to the system.
                DeviceEvent::Added => {
                    if let Some(device_added) = app.events.device_added {
                        device_added(app_data, &device_id);
                    }
                }
                // A device was disconnected from the system.
                DeviceEvent::Removed => {
                    if let Some(device_removed) = app.events.device_removed {
                        device_removed(app_data, &device_id);
                    }
                }
                // A mouse device was moved.
                DeviceEvent::MouseMotion { delta } => {
                    if let Some(mouse_moved) = app.events.mouse_moved {
                        mouse_moved(app_data, &device_id, delta.0, -delta.1);
                    }
                }
                // A scroll wheel was scrolled.
                DeviceEvent::MouseWheel { delta } => {
                    if let Some(mouse_scrolled) = app.events.mouse_scrolled {
                        let (x, y, delta_t) = NSTDScrollDelta::from_winit(delta);
                        mouse_scrolled(app_data, &device_id, x, y, delta_t);
                    }
                }
                // There was motion on some analog axis.
                DeviceEvent::Motion { axis, value } => {
                    if let Some(axis_motion) = app.events.axis_motion {
                        axis_motion(app_data, &device_id, &axis, value);
                    }
                }
                // A button's state was changed.
                DeviceEvent::Button { button, state } => {
                    if let Some(button_input) = app.events.button_input {
                        let is_down = state == ElementState::Pressed;
                        button_input(app_data, &device_id, &button, is_down);
                    }
                }
                // There was some keyboard input.
                DeviceEvent::Key(input) => {
                    if let Some(key_input) = app.events.key_input {
                        let key = NSTDKey::from_winit(input.virtual_keycode);
                        let is_down = input.state == ElementState::Pressed;
                        key_input(app_data, &device_id, key, input.scancode, is_down);
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
                            app_data,
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
                        window_resized(app_data, &window_id, size.width, size.height);
                    }
                }
                // A window was moved.
                WindowEvent::Moved(pos) => {
                    if let Some(window_moved) = app.events.window_moved {
                        window_moved(app_data, &window_id, pos.x, pos.y);
                    }
                }
                // A window's focus has changed.
                WindowEvent::Focused(is_focused) => {
                    if let Some(window_focus_changed) = app.events.window_focus_changed {
                        window_focus_changed(app_data, &window_id, is_focused);
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
                        let is_down = state == ElementState::Pressed;
                        window_mouse_input(app_data, &window_id, &device_id, &input, is_down);
                    }
                }
                // A window received key input.
                WindowEvent::KeyboardInput {
                    device_id, input, ..
                } => {
                    if let Some(window_key_input) = app.events.window_key_input {
                        let key = NSTDKey::from_winit(input.virtual_keycode);
                        let is_down = input.state == ElementState::Pressed;
                        let scancode = input.scancode;
                        window_key_input(app_data, &window_id, &device_id, key, scancode, is_down);
                    }
                }
                // A window received character input.
                WindowEvent::ReceivedCharacter(chr) => {
                    if let Some(window_received_char) = app.events.window_received_char {
                        window_received_char(app_data, &window_id, chr.into());
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
                        window_scrolled(app_data, &window_id, &device_id, x, y, delta_t, touch);
                    }
                }
                // The cursor was moved over a window.
                WindowEvent::CursorMoved {
                    device_id,
                    position: pos,
                    ..
                } => {
                    if let Some(window_cursor_moved) = app.events.window_cursor_moved {
                        window_cursor_moved(app_data, &window_id, &device_id, pos.x, pos.y);
                    }
                }
                // The cursor entered a window.
                WindowEvent::CursorEntered { device_id } => {
                    if let Some(window_cursor_entered) = app.events.window_cursor_entered {
                        window_cursor_entered(app_data, &window_id, &device_id);
                    }
                }
                // The cursor left a window.
                WindowEvent::CursorLeft { device_id } => {
                    if let Some(window_cursor_left) = app.events.window_cursor_left {
                        window_cursor_left(app_data, &window_id, &device_id);
                    }
                }
                // A file was dropped into a window.
                WindowEvent::DroppedFile(path) => {
                    if let Some(window_file_received) = app.events.window_file_received {
                        let path = path.to_string_lossy();
                        let path = NSTDStr::from_str(&path);
                        window_file_received(app_data, &window_id, &path);
                    }
                }
                // A file was hovered over a window.
                WindowEvent::HoveredFile(path) => {
                    if let Some(window_file_hovered) = app.events.window_file_hovered {
                        let path = path.to_string_lossy();
                        let path = NSTDStr::from_str(&path);
                        window_file_hovered(app_data, &window_id, &path);
                    }
                }
                // A file was dragged away from a window.
                WindowEvent::HoveredFileCancelled => {
                    if let Some(window_file_canceled) = app.events.window_file_canceled {
                        window_file_canceled(app_data, &window_id);
                    }
                }
                // A window requests closing.
                WindowEvent::CloseRequested => {
                    if let Some(window_close_requested) = app.events.window_close_requested {
                        window_close_requested(app_data, &window_id);
                    }
                }
                // A window was permanently closed.
                WindowEvent::Destroyed => {
                    if let Some(window_closed) = app.events.window_closed {
                        window_closed(app_data, &window_id);
                    }
                }
                _ => (),
            },
            // The event loop is being exited.
            Event::LoopDestroyed => {
                if let Some(exit) = app.events.exit {
                    exit(app_data);
                }
            }
            _ => (),
        }
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

/// Invokes a callback function for each display detected by an `nstd` app.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to the `nstd` application.
///
/// - `void (*callback)(NSTDDisplayHandle)` - The callback function.
///
/// # Safety
///
/// The user of this function must guarantee that `callback` is a valid C function pointer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_app_displays(
    app: NSTDAppHandle,
    callback: Option<unsafe extern "C" fn(NSTDDisplayHandle)>,
) {
    if let Some(callback) = callback {
        for handle in app.available_monitors() {
            callback(&handle);
        }
    }
}

/// Returns a handle to the primary display.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to the `nstd` application.
///
/// # Returns
///
/// `NSTDDisplay display` - A handle to the primary display, null on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_primary_display(app: NSTDAppHandle) -> Option<NSTDDisplay> {
    app.primary_monitor().map(Box::new)
}

/// Sets the `nstd` application's device filtering mode.
///
/// # Parameters:
///
/// - `NSTDAppHandle app` - A handle to the `nstd` application.
///
/// - `NSTDDeviceEventFilter filter` - The device event filtering mode to use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_set_device_event_filter(
    app: NSTDAppHandle,
    filter: NSTDDeviceEventFilter,
) {
    app.set_device_event_filter(filter.into());
}

/// Signals an `NSTDApp`'s event loop to exit.
///
/// # Parameters:
///
/// - `const NSTDAppData *app` - The application data received from an event.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_exit(app: &NSTDAppData) {
    app.control_flow().set(ControlFlow::Exit);
}

/// Signals an `NSTDApp`'s event loop to exit with a specific error code.
///
/// # Parameters:
///
/// - `const NSTDAppData *app` - The application data received from an event.
///
/// - `NSTDErrorCode errc` - The error code to exit the application event loop with.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_exit_with_code(app: &NSTDAppData, errc: NSTDErrorCode) {
    app.control_flow().set(ControlFlow::ExitWithCode(errc));
}
