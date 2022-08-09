//! Contains callback based events through function pointers.
use crate::app::data::NSTDAppData;
use winit::{event::DeviceId, window::WindowId};

/// A window's unique identifier.
pub type NSTDWindowID<'a> = &'a WindowId;

/// A device's unique identifier.
pub type NSTDDeviceID<'a> = &'a DeviceId;

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
    /// A window requests closing.
    pub window_close_requested: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID)>,
    /// Called once before exiting the application event loop.
    pub exit: Option<unsafe extern "C" fn(&NSTDAppData)>,
}
