//! Contains callback based events through function pointers.
use crate::app::data::NSTDAppData;
use winit::window::WindowId;

/// A window's unique identifier.
pub type NSTDWindowID<'a> = &'a WindowId;

/// Contains callback based events through function pointers.
#[repr(C)]
#[derive(Default)]
pub struct NSTDAppEvents {
    /// Called once before starting the application event loop.
    pub start: Option<unsafe extern "C" fn(&NSTDAppData)>,
    /// Called when all other events have been processed.
    pub update: Option<unsafe extern "C" fn(&NSTDAppData)>,
    /// A window requests closing.
    pub window_close_requested: Option<unsafe extern "C" fn(&NSTDAppData, NSTDWindowID)>,
    /// Called once before exiting the application event loop.
    pub exit: Option<unsafe extern "C" fn(&NSTDAppData)>,
}
