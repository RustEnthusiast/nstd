//! Contains callback based events through function pointers.
use crate::{app::data::NSTDAppData, NSTDFloat64};
use winit::{
    event::{AxisId, ButtonId, DeviceId, ElementState},
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

/// Describes the state of a button.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDButtonState {
    /// The button is up.
    NSTD_BUTTON_STATE_RELEASED,
    /// The button is pressed down.
    NSTD_BUTTON_STATE_PRESSED,
}
impl From<ElementState> for NSTDButtonState {
    /// Creates an [NSTDButtonState] from a [winit] [ElementState].
    #[inline]
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Released => Self::NSTD_BUTTON_STATE_RELEASED,
            ElementState::Pressed => Self::NSTD_BUTTON_STATE_PRESSED,
        }
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
    pub button_changed:
        Option<unsafe extern "C" fn(&NSTDAppData, NSTDDeviceID, NSTDButtonID, NSTDButtonState)>,
    /// A window requests closing.
    pub window_close_requested: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID)>,
    /// Called once before exiting the application event loop.
    pub exit: Option<unsafe extern "C" fn(&NSTDAppData)>,
}
