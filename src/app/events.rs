//! Contains callback based events through function pointers.
use crate::{
    app::data::NSTDAppData,
    core::{str::NSTDStr, unichar::NSTDUnichar},
    NSTDBool, NSTDFloat32, NSTDFloat64, NSTDInt32, NSTDUInt16, NSTDUInt32,
};
use gilrs::{Axis, Button, GamepadId};
use winit::{
    event::{DeviceId, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode},
    event_loop::DeviceEventFilter,
    window::WindowId,
};

/// A window's unique identifier.
pub type NSTDWindowID = Box<WindowId>;

/// A device's unique identifier.
pub type NSTDDeviceID = Box<DeviceId>;

/// A gamepad's unique identifier.
pub type NSTDGamepadID = Box<GamepadId>;

/// Identifier for an analog axis on a device.
pub type NSTDAnalogAxisID = NSTDUInt32;

/// A button's unique identifier.
pub type NSTDButtonID = NSTDUInt32;

/// An enumeration of device event filtering modes.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDDeviceEventFilter {
    /// Always dispatch device events.
    NSTD_DEVICE_EVENT_FILTER_NONE,
    /// Only dispatch device events when an application window is focused.
    NSTD_DEVICE_EVENT_FILTER_UNFOCUSED,
    /// Never dispatch device events.
    NSTD_DEVICE_EVENT_FILTER_ALL,
}
impl From<NSTDDeviceEventFilter> for DeviceEventFilter {
    /// Converts an [NSTDDeviceEventFilter] into a [DeviceEventFilter].
    fn from(value: NSTDDeviceEventFilter) -> Self {
        match value {
            NSTDDeviceEventFilter::NSTD_DEVICE_EVENT_FILTER_NONE => Self::Never,
            NSTDDeviceEventFilter::NSTD_DEVICE_EVENT_FILTER_UNFOCUSED => Self::Unfocused,
            NSTDDeviceEventFilter::NSTD_DEVICE_EVENT_FILTER_ALL => Self::Always,
        }
    }
}

/// Describes a mouse wheel's scroll delta.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDScrollDelta {
    /// The scroll was measured in lines.
    NSTD_SCROLL_DELTA_LINE,
    /// The scroll was measured in pixels.
    NSTD_SCROLL_DELTA_PIXEL,
}
impl NSTDScrollDelta {
    /// Converts a [MouseScrollDelta] into an [NSTDScrollDelta], returning the X & Y delta as the
    /// first two tuple elements.
    #[inline]
    pub(crate) fn from_winit(delta: MouseScrollDelta) -> (f64, f64, Self) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64, Self::NSTD_SCROLL_DELTA_LINE),
            MouseScrollDelta::PixelDelta(scroll) => {
                (scroll.x, scroll.y, Self::NSTD_SCROLL_DELTA_PIXEL)
            }
        }
    }
}

/// Describes a touch-screen's state.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDTouchState {
    /// The touch event has just started.
    NSTD_TOUCH_STATE_STARTED,
    /// The touch position has been moved.
    NSTD_TOUCH_STATE_MOVED,
    /// The touch event has ended.
    NSTD_TOUCH_STATE_ENDED,
    /// The touch event has been cancelled.
    NSTD_TOUCH_STATE_CANCELLED,
}
impl NSTDTouchState {
    /// Converts a [TouchPhase] into an [NSTDTouchState].
    #[inline]
    pub(crate) fn from_winit(phase: TouchPhase) -> Self {
        match phase {
            TouchPhase::Started => Self::NSTD_TOUCH_STATE_STARTED,
            TouchPhase::Moved => Self::NSTD_TOUCH_STATE_MOVED,
            TouchPhase::Ended => Self::NSTD_TOUCH_STATE_ENDED,
            TouchPhase::Cancelled => Self::NSTD_TOUCH_STATE_CANCELLED,
        }
    }
}

/// Represents a mouse button.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDMouseButton {
    /// The left mouse button.
    NSTD_MOUSE_BUTTON_LEFT,
    /// The middle mouse button.
    NSTD_MOUSE_BUTTON_MIDDLE,
    /// The right mouse button.
    NSTD_MOUSE_BUTTON_RIGHT,
    /// An extra mouse button.
    NSTD_MOUSE_BUTTON_OTHER,
}

/// Represents some type of mouse button input.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NSTDMouseInput {
    /// The mouse button that received input.
    pub button: NSTDMouseButton,
    /// The ID of the mouse button that received input.
    pub id: NSTDUInt16,
}
impl NSTDMouseInput {
    /// Converts [winit] [MouseButton] into [NSTDMouseInput].
    pub(crate) fn from_winit(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => Self {
                button: NSTDMouseButton::NSTD_MOUSE_BUTTON_LEFT,
                id: 0,
            },
            MouseButton::Middle => Self {
                button: NSTDMouseButton::NSTD_MOUSE_BUTTON_MIDDLE,
                id: 1,
            },
            MouseButton::Right => Self {
                button: NSTDMouseButton::NSTD_MOUSE_BUTTON_RIGHT,
                id: 2,
            },
            MouseButton::Other(id) => Self {
                button: NSTDMouseButton::NSTD_MOUSE_BUTTON_OTHER,
                id,
            },
        }
    }
}

/// Represents a key on a keyboard.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDKey {
    /// An unknown keyboard key.
    NSTD_KEY_UNKNOWN,
    /// The `esc` key.
    NSTD_KEY_ESCAPE,
    /// The `F1` key.
    NSTD_KEY_F1,
    /// The `F2` key.
    NSTD_KEY_F2,
    /// The `F3` key.
    NSTD_KEY_F3,
    /// The `F4` key.
    NSTD_KEY_F4,
    /// The `F5` key.
    NSTD_KEY_F5,
    /// The `F6` key.
    NSTD_KEY_F6,
    /// The `F7` key.
    NSTD_KEY_F7,
    /// The `F8` key.
    NSTD_KEY_F8,
    /// The `F9` key.
    NSTD_KEY_F9,
    /// The `F10` key.
    NSTD_KEY_F10,
    /// The `F11` key.
    NSTD_KEY_F11,
    /// The `F12` key.
    NSTD_KEY_F12,
    /// The `1` key.
    NSTD_KEY_1,
    /// The `2` key.
    NSTD_KEY_2,
    /// The `3` key.
    NSTD_KEY_3,
    /// The `4` key.
    NSTD_KEY_4,
    /// The `5` key.
    NSTD_KEY_5,
    /// The `6` key.
    NSTD_KEY_6,
    /// The `7` key.
    NSTD_KEY_7,
    /// The `8` key.
    NSTD_KEY_8,
    /// The `9` key.
    NSTD_KEY_9,
    /// The `0` key.
    NSTD_KEY_0,
    /// The `A` key.
    NSTD_KEY_A,
    /// The `B` key.
    NSTD_KEY_B,
    /// The `C` key.
    NSTD_KEY_C,
    /// The `D` key.
    NSTD_KEY_D,
    /// The `E` key.
    NSTD_KEY_E,
    /// The `F` key.
    NSTD_KEY_F,
    /// The `G` key.
    NSTD_KEY_G,
    /// The `H` key.
    NSTD_KEY_H,
    /// The `I` key.
    NSTD_KEY_I,
    /// The `J` key.
    NSTD_KEY_J,
    /// The `K` key.
    NSTD_KEY_K,
    /// The `L` key.
    NSTD_KEY_L,
    /// The `M` key.
    NSTD_KEY_M,
    /// The `N` key.
    NSTD_KEY_N,
    /// The `O` key.
    NSTD_KEY_O,
    /// The `P` key.
    NSTD_KEY_P,
    /// The `Q` key.
    NSTD_KEY_Q,
    /// The `R` key.
    NSTD_KEY_R,
    /// The `S` key.
    NSTD_KEY_S,
    /// The `T` key.
    NSTD_KEY_T,
    /// The `U` key.
    NSTD_KEY_U,
    /// The `V` key.
    NSTD_KEY_V,
    /// The `W` key.
    NSTD_KEY_W,
    /// The `X` key.
    NSTD_KEY_X,
    /// The `Y` key.
    NSTD_KEY_Y,
    /// The `Z` key.
    NSTD_KEY_Z,
    /// The ` key.
    NSTD_KEY_GRAVE,
    /// The `-` key.
    NSTD_KEY_MINUS,
    /// The `=` key.
    NSTD_KEY_EQUALS,
    /// The backspace key.
    NSTD_KEY_BACKSPACE,
    /// The tab key.
    NSTD_KEY_TAB,
    /// The `[` key.
    NSTD_KEY_OPEN_BRACKET,
    /// The `]` key.
    NSTD_KEY_CLOSE_BRACKET,
    /// The `\` key.
    NSTD_KEY_BACK_SLASH,
    /// The capital lock key.
    NSTD_KEY_CAPS_LOCK,
    /// The `;` key.
    NSTD_KEY_SEMICOLON,
    /// The `'` key.
    NSTD_KEY_APOSTROPHE,
    /// The enter key.
    NSTD_KEY_ENTER,
    /// The `,` key.
    NSTD_KEY_COMMA,
    /// The `.` key.
    NSTD_KEY_PERIOD,
    /// The `/` key.
    NSTD_KEY_FORWARD_SLASH,
    /// The space key.
    NSTD_KEY_SPACE,
    /// The left shift key.
    NSTD_KEY_LEFT_SHIFT,
    /// The left control key.
    NSTD_KEY_LEFT_CTRL,
    /// The left alt key.
    NSTD_KEY_LEFT_ALT,
    /// The right shift key.
    NSTD_KEY_RIGHT_SHIFT,
    /// The right control key.
    NSTD_KEY_RIGHT_CTRL,
    /// The right alt key.
    NSTD_KEY_RIGHT_ALT,
}
impl NSTDKey {
    /// Converts a [VirtualKeyCode] into an [NSTDKey].
    pub(crate) fn from_winit(key: Option<VirtualKeyCode>) -> Self {
        if let Some(key) = key {
            return match key {
                VirtualKeyCode::Escape => NSTDKey::NSTD_KEY_ESCAPE,
                VirtualKeyCode::F1 => NSTDKey::NSTD_KEY_F1,
                VirtualKeyCode::F2 => NSTDKey::NSTD_KEY_F2,
                VirtualKeyCode::F3 => NSTDKey::NSTD_KEY_F3,
                VirtualKeyCode::F4 => NSTDKey::NSTD_KEY_F4,
                VirtualKeyCode::F5 => NSTDKey::NSTD_KEY_F5,
                VirtualKeyCode::F6 => NSTDKey::NSTD_KEY_F6,
                VirtualKeyCode::F7 => NSTDKey::NSTD_KEY_F7,
                VirtualKeyCode::F8 => NSTDKey::NSTD_KEY_F8,
                VirtualKeyCode::F9 => NSTDKey::NSTD_KEY_F9,
                VirtualKeyCode::F10 => NSTDKey::NSTD_KEY_F10,
                VirtualKeyCode::F11 => NSTDKey::NSTD_KEY_F11,
                VirtualKeyCode::F12 => NSTDKey::NSTD_KEY_F12,
                VirtualKeyCode::Key1 => NSTDKey::NSTD_KEY_1,
                VirtualKeyCode::Key2 => NSTDKey::NSTD_KEY_2,
                VirtualKeyCode::Key3 => NSTDKey::NSTD_KEY_3,
                VirtualKeyCode::Key4 => NSTDKey::NSTD_KEY_4,
                VirtualKeyCode::Key5 => NSTDKey::NSTD_KEY_5,
                VirtualKeyCode::Key6 => NSTDKey::NSTD_KEY_6,
                VirtualKeyCode::Key7 => NSTDKey::NSTD_KEY_7,
                VirtualKeyCode::Key8 => NSTDKey::NSTD_KEY_8,
                VirtualKeyCode::Key9 => NSTDKey::NSTD_KEY_9,
                VirtualKeyCode::Key0 => NSTDKey::NSTD_KEY_0,
                VirtualKeyCode::A => NSTDKey::NSTD_KEY_A,
                VirtualKeyCode::B => NSTDKey::NSTD_KEY_B,
                VirtualKeyCode::C => NSTDKey::NSTD_KEY_C,
                VirtualKeyCode::D => NSTDKey::NSTD_KEY_D,
                VirtualKeyCode::E => NSTDKey::NSTD_KEY_E,
                VirtualKeyCode::F => NSTDKey::NSTD_KEY_F,
                VirtualKeyCode::G => NSTDKey::NSTD_KEY_G,
                VirtualKeyCode::H => NSTDKey::NSTD_KEY_H,
                VirtualKeyCode::I => NSTDKey::NSTD_KEY_I,
                VirtualKeyCode::J => NSTDKey::NSTD_KEY_J,
                VirtualKeyCode::K => NSTDKey::NSTD_KEY_K,
                VirtualKeyCode::L => NSTDKey::NSTD_KEY_L,
                VirtualKeyCode::M => NSTDKey::NSTD_KEY_M,
                VirtualKeyCode::N => NSTDKey::NSTD_KEY_N,
                VirtualKeyCode::O => NSTDKey::NSTD_KEY_O,
                VirtualKeyCode::P => NSTDKey::NSTD_KEY_P,
                VirtualKeyCode::Q => NSTDKey::NSTD_KEY_Q,
                VirtualKeyCode::R => NSTDKey::NSTD_KEY_R,
                VirtualKeyCode::S => NSTDKey::NSTD_KEY_S,
                VirtualKeyCode::T => NSTDKey::NSTD_KEY_T,
                VirtualKeyCode::U => NSTDKey::NSTD_KEY_U,
                VirtualKeyCode::V => NSTDKey::NSTD_KEY_V,
                VirtualKeyCode::W => NSTDKey::NSTD_KEY_W,
                VirtualKeyCode::X => NSTDKey::NSTD_KEY_X,
                VirtualKeyCode::Y => NSTDKey::NSTD_KEY_Y,
                VirtualKeyCode::Z => NSTDKey::NSTD_KEY_Z,
                VirtualKeyCode::Grave => NSTDKey::NSTD_KEY_GRAVE,
                VirtualKeyCode::Minus => NSTDKey::NSTD_KEY_MINUS,
                VirtualKeyCode::Equals => NSTDKey::NSTD_KEY_EQUALS,
                VirtualKeyCode::Back => NSTDKey::NSTD_KEY_BACKSPACE,
                VirtualKeyCode::Tab => NSTDKey::NSTD_KEY_TAB,
                VirtualKeyCode::LBracket => NSTDKey::NSTD_KEY_OPEN_BRACKET,
                VirtualKeyCode::RBracket => NSTDKey::NSTD_KEY_CLOSE_BRACKET,
                VirtualKeyCode::Backslash => NSTDKey::NSTD_KEY_BACK_SLASH,
                VirtualKeyCode::Capital => NSTDKey::NSTD_KEY_CAPS_LOCK,
                VirtualKeyCode::Semicolon => NSTDKey::NSTD_KEY_SEMICOLON,
                VirtualKeyCode::Apostrophe => NSTDKey::NSTD_KEY_APOSTROPHE,
                VirtualKeyCode::Return => NSTDKey::NSTD_KEY_ENTER,
                VirtualKeyCode::Comma => NSTDKey::NSTD_KEY_COMMA,
                VirtualKeyCode::Period => NSTDKey::NSTD_KEY_PERIOD,
                VirtualKeyCode::Slash => NSTDKey::NSTD_KEY_FORWARD_SLASH,
                VirtualKeyCode::Space => NSTDKey::NSTD_KEY_SPACE,
                VirtualKeyCode::LShift => NSTDKey::NSTD_KEY_LEFT_SHIFT,
                VirtualKeyCode::LControl => NSTDKey::NSTD_KEY_LEFT_CTRL,
                VirtualKeyCode::LAlt => NSTDKey::NSTD_KEY_LEFT_ALT,
                VirtualKeyCode::RShift => NSTDKey::NSTD_KEY_RIGHT_SHIFT,
                VirtualKeyCode::RControl => NSTDKey::NSTD_KEY_RIGHT_CTRL,
                VirtualKeyCode::RAlt => NSTDKey::NSTD_KEY_RIGHT_ALT,
                _ => NSTDKey::NSTD_KEY_UNKNOWN,
            };
        }
        NSTDKey::NSTD_KEY_UNKNOWN
    }
}

/// Represents a gamepad button.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDGamepadButton {
    /// The upper action pad button.
    ///
    /// Corresponds to the `Y` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_NORTH,
    /// The lower action pad button.
    ///
    /// Corresponds to the `A` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_SOUTH,
    /// The right action pad button.
    ///
    /// Corresponds to the `B` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_EAST,
    /// The left action pad button.
    ///
    /// Corresponds to the `X` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_WEST,
    /// The right bumper.
    ///
    /// Corresponds to `RB` on Xbox controllers & `R1` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_RIGHT_BUMPER,
    /// The left bumper.
    ///
    /// Corresponds to `LB` on Xbox controllers & `L1` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_LEFT_BUMPER,
    /// The right trigger.
    ///
    /// Corresponds to `RT` on Xbox controllers & `R2` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_RIGHT_TRIGGER,
    /// The left trigger.
    ///
    /// Corresponds to `LT` on Xbox controllers & `L2` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_LEFT_TRIGGER,
    /// The start/pause button.
    NSTD_GAMEPAD_BUTTON_START,
    /// The select/back button.
    NSTD_GAMEPAD_BUTTON_SELECT,
    /// The right thumb stick.
    NSTD_GAMEPAD_BUTTON_RIGHT_THUMB,
    /// The left thumb stick.
    NSTD_GAMEPAD_BUTTON_LEFT_THUMB,
    /// The upper direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_UP,
    /// The lower direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_DOWN,
    /// The right direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_RIGHT,
    /// The left direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_LEFT,
    /// An unrecognized button.
    NSTD_GAMEPAD_BUTTON_UNKNOWN,
}
impl NSTDGamepadButton {
    /// Converts a [gilrs] [Button] into an [NSTDGamepadButton].
    pub(crate) fn from_winit(button: Button) -> Self {
        match button {
            Button::North => Self::NSTD_GAMEPAD_BUTTON_NORTH,
            Button::South => Self::NSTD_GAMEPAD_BUTTON_SOUTH,
            Button::East => Self::NSTD_GAMEPAD_BUTTON_EAST,
            Button::West => Self::NSTD_GAMEPAD_BUTTON_WEST,
            Button::RightTrigger => Self::NSTD_GAMEPAD_BUTTON_RIGHT_BUMPER,
            Button::LeftTrigger => Self::NSTD_GAMEPAD_BUTTON_LEFT_BUMPER,
            Button::RightTrigger2 => Self::NSTD_GAMEPAD_BUTTON_RIGHT_TRIGGER,
            Button::LeftTrigger2 => Self::NSTD_GAMEPAD_BUTTON_LEFT_TRIGGER,
            Button::Start => Self::NSTD_GAMEPAD_BUTTON_START,
            Button::Select => Self::NSTD_GAMEPAD_BUTTON_SELECT,
            Button::RightThumb => Self::NSTD_GAMEPAD_BUTTON_RIGHT_THUMB,
            Button::LeftThumb => Self::NSTD_GAMEPAD_BUTTON_LEFT_THUMB,
            Button::DPadUp => Self::NSTD_GAMEPAD_BUTTON_DPAD_UP,
            Button::DPadDown => Self::NSTD_GAMEPAD_BUTTON_DPAD_DOWN,
            Button::DPadRight => Self::NSTD_GAMEPAD_BUTTON_DPAD_RIGHT,
            Button::DPadLeft => Self::NSTD_GAMEPAD_BUTTON_DPAD_LEFT,
            _ => Self::NSTD_GAMEPAD_BUTTON_UNKNOWN,
        }
    }
}

/// Represents a gamepad axis.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDGamepadAxis {
    /// The left stick x-axis.
    NSTD_GAMEPAD_AXIS_LEFT_X,
    /// The left stick y-axis.
    NSTD_GAMEPAD_AXIS_LEFT_Y,
    /// The left stick z-axis.
    NSTD_GAMEPAD_AXIS_LEFT_Z,
    /// The right stick x-axis.
    NSTD_GAMEPAD_AXIS_RIGHT_X,
    /// The right stick y-axis.
    NSTD_GAMEPAD_AXIS_RIGHT_Y,
    /// The right stick z-axis.
    NSTD_GAMEPAD_AXIS_RIGHT_Z,
    /// Left or right on the direction pad.
    NSTD_GAMEPAD_AXIS_DPAD_X,
    /// Up or down on the direction pad.
    NSTD_GAMEPAD_AXIS_DPAD_Y,
    /// An unknown axis.
    NSTD_GAMEPAD_AXIS_UNKNOWN,
}
impl NSTDGamepadAxis {
    /// Converts a [gilrs] [Axis] into an [NSTDGamepadAxis].
    pub(crate) fn from_winit(axis: Axis) -> Self {
        match axis {
            Axis::LeftStickX => Self::NSTD_GAMEPAD_AXIS_LEFT_X,
            Axis::LeftStickY => Self::NSTD_GAMEPAD_AXIS_LEFT_Y,
            Axis::LeftZ => Self::NSTD_GAMEPAD_AXIS_LEFT_Z,
            Axis::RightStickX => Self::NSTD_GAMEPAD_AXIS_RIGHT_X,
            Axis::RightStickY => Self::NSTD_GAMEPAD_AXIS_RIGHT_Y,
            Axis::RightZ => Self::NSTD_GAMEPAD_AXIS_RIGHT_Z,
            Axis::DPadX => Self::NSTD_GAMEPAD_AXIS_DPAD_X,
            Axis::DPadY => Self::NSTD_GAMEPAD_AXIS_DPAD_Y,
            _ => Self::NSTD_GAMEPAD_AXIS_UNKNOWN,
        }
    }
}

/// Contains callback based events through function pointers.
#[repr(C)]
#[derive(Default)]
pub struct NSTDAppEvents {
    /// Called once before starting the application event loop.
    pub start: Option<unsafe extern "C" fn(&mut NSTDAppData)>,
    /// Called when all other events have been processed.
    pub update: Option<unsafe extern "C" fn(&mut NSTDAppData)>,
    /// Called when a new device is connected to the system.
    pub device_added: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDDeviceID)>,
    /// Called when a device was disconnected from the system.
    pub device_removed: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDDeviceID)>,
    /// Called when a mouse device is moved.
    pub mouse_moved:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDDeviceID, NSTDFloat64, NSTDFloat64)>,
    /// Called when a scroll wheel is scrolled.
    pub mouse_scrolled: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDDeviceID,
            NSTDFloat64,
            NSTDFloat64,
            NSTDScrollDelta,
        ),
    >,
    /// Called when there is some motion on an analog axis device, such as a touchpad.
    ///
    /// # Note
    ///
    /// Some touchpads can return a negative y value.
    pub axis_motion:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDDeviceID, NSTDAnalogAxisID, NSTDFloat64)>,
    /// Called when a button, such as a mouse button's state changes.
    pub button_input:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDDeviceID, NSTDButtonID, NSTDBool)>,
    /// Called when a keyboard key is pressed or unpressed.
    pub key_input:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDDeviceID, NSTDKey, NSTDUInt32, NSTDBool)>,
    /// Called when a window's scale factor changes.
    pub window_dpi_changed: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDWindowID,
            NSTDFloat64,
            &mut NSTDUInt32,
            &mut NSTDUInt32,
        ),
    >,
    /// Called when a window is resized.
    pub window_resized:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, NSTDUInt32, NSTDUInt32)>,
    /// Called when a window is moved.
    pub window_moved:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, NSTDInt32, NSTDInt32)>,
    /// Focus for a window changed.
    pub window_focus_changed:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, NSTDBool)>,
    /// Mouse input was received.
    pub window_mouse_input: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDWindowID,
            NSTDDeviceID,
            &NSTDMouseInput,
            NSTDBool,
        ),
    >,
    /// Called when a window receives key input.
    pub window_key_input: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDWindowID,
            NSTDDeviceID,
            NSTDKey,
            NSTDUInt32,
            NSTDBool,
        ),
    >,
    /// Called when a window receives a character.
    pub window_received_char:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, NSTDUnichar)>,
    /// Called when a scroll device is scrolled over a window.
    pub window_scrolled: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDWindowID,
            NSTDDeviceID,
            NSTDFloat64,
            NSTDFloat64,
            NSTDScrollDelta,
            NSTDTouchState,
        ),
    >,
    /// Called when the cursor is moved over a window.
    pub window_cursor_moved: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDWindowID,
            NSTDDeviceID,
            NSTDFloat64,
            NSTDFloat64,
        ),
    >,
    /// The cursor entered a window.
    pub window_cursor_entered:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, NSTDDeviceID)>,
    /// The cursor left a window.
    pub window_cursor_left:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, NSTDDeviceID)>,
    /// A file was dropped into a window.
    pub window_file_received:
        Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, &NSTDStr)>,
    /// A file was hovered over a window.
    pub window_file_hovered: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID, &NSTDStr)>,
    /// A file was dragged away from a window.
    pub window_file_canceled: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID)>,
    /// A window requests closing.
    pub window_close_requested: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID)>,
    /// Called when a window is closed.
    pub window_closed: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDWindowID)>,
    /// A gamepad was connected to the system.
    pub gamepad_connected: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDGamepadID)>,
    /// A gamepad was disconnected to the system.
    pub gamepad_disconnected: Option<unsafe extern "C" fn(&mut NSTDAppData, NSTDGamepadID)>,
    /// A gamepad button was pressed.
    pub gamepad_button_pressed: Option<
        unsafe extern "C" fn(&mut NSTDAppData, NSTDGamepadID, NSTDGamepadButton, NSTDUInt32),
    >,
    /// A gamepad button was released.
    pub gamepad_button_released: Option<
        unsafe extern "C" fn(&mut NSTDAppData, NSTDGamepadID, NSTDGamepadButton, NSTDUInt32),
    >,
    /// A gamepad button's value changed.
    pub gamepad_input: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDGamepadID,
            NSTDGamepadButton,
            NSTDUInt32,
            NSTDFloat32,
        ),
    >,
    /// A gamepad axis value has changed.
    pub gamepad_axis_input: Option<
        unsafe extern "C" fn(
            &mut NSTDAppData,
            NSTDGamepadID,
            NSTDGamepadAxis,
            NSTDUInt32,
            NSTDFloat32,
        ),
    >,
    /// Called once before exiting the application event loop.
    pub exit: Option<unsafe extern "C" fn(&mut NSTDAppData)>,
}
