//! Contains callback based events through function pointers.
use crate::{app::data::NSTDAppData, NSTDBool, NSTDFloat64, NSTDInt32, NSTDUInt32};
use winit::{
    event::{AxisId, ButtonId, DeviceId, VirtualKeyCode},
    window::WindowId,
};

/// A window's unique identifier.
pub type NSTDWindowID<'a> = &'a WindowId;

/// A device's unique identifier.
pub type NSTDDeviceID<'a> = &'a DeviceId;

/// Identifier for an analog axis on a device.
pub type NSTDAnalogAxisID<'a> = &'a AxisId;

/// A button's unique identifier.
pub type NSTDButtonID<'a> = &'a ButtonId;

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

/// Represents a key on a keyboard.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
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
impl From<Option<VirtualKeyCode>> for NSTDKey {
    /// Converts a [VirtualKeyCode] into an [NSTDKey].
    fn from(key: Option<VirtualKeyCode>) -> Self {
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

/// Contains callback based events through function pointers.
#[repr(C)]
#[derive(Default)]
pub struct NSTDAppEvents {
    /// Called once before starting the application event loop.
    pub start: Option<unsafe extern "C" fn(&NSTDAppData)>,
    /// Called when all other events have been processed.
    pub update: Option<unsafe extern "C" fn(&NSTDAppData)>,
    /// Called when a new device is connected to the system.
    pub device_added: Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID)>,
    /// Called when a device was disconnected from the system.
    pub device_removed: Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID)>,
    /// Called when a mouse device is moved.
    pub mouse_moved:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID, NSTDFloat64, NSTDFloat64)>,
    /// Called when a scroll wheel is scrolled.
    pub mouse_scrolled: Option<
        unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID, NSTDFloat64, NSTDFloat64, NSTDScrollDelta),
    >,
    /// Called when there is some motion on an analog axis device, such as a touchpad.
    ///
    /// # Note
    ///
    /// Some touchpads can return a negative y value.
    pub axis_motion:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID, NSTDAnalogAxisID, NSTDFloat64)>,
    /// Called when a button, such as a mouse button's state changes.
    pub button_input:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID, NSTDButtonID, NSTDBool)>,
    /// Called when a keyboard key is pressed or unpressed.
    pub key_input:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID, NSTDKey, NSTDUInt32, NSTDBool)>,
    /// Called when a window is resized.
    pub window_resized:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID, NSTDUInt32, NSTDUInt32)>,
    /// Called when a window is moved.
    pub window_moved:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID, NSTDInt32, NSTDInt32)>,
    /// Focus for a window changed.
    pub window_focus_changed: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID, NSTDBool)>,
    /// Called when the cursor is moved over a window.
    pub window_cursor_moved: Option<
        unsafe extern "C" fn(&NSTDAppData, NSTDWindowID, NSTDDeviceID, NSTDFloat64, NSTDFloat64),
    >,
    /// The cursor entered a window.
    pub window_cursor_entered:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID, NSTDDeviceID)>,
    /// The cursor left a window.
    pub window_cursor_left: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID, NSTDDeviceID)>,
    /// A window requests closing.
    pub window_close_requested: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID)>,
    /// Called when a window is closed.
    pub window_closed: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID)>,
    /// Called once before exiting the application event loop.
    pub exit: Option<unsafe extern "C" fn(&NSTDAppData)>,
}
