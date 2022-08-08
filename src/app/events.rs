//! Contains callback based events through function pointers.
use crate::app::data::NSTDAppData;

/// Contains callback based events through function pointers.
#[repr(C)]
pub struct NSTDAppEvents {
    /// Called once before starting the application event loop.
    pub start: Option<unsafe extern "C" fn(&NSTDAppData)>,
    /// Called when all other events have been processed.
    pub update: Option<unsafe extern "C" fn(&NSTDAppData)>,
    /// Called once before exiting the application event loop.
    pub exit: Option<unsafe extern "C" fn(&NSTDAppData)>,
}
impl NSTDAppEvents {
    /// Creates a new, all null, events structure.
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            start: None,
            update: None,
            exit: None,
        }
    }
}
