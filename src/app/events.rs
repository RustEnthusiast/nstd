//! Contains callback based events through function pointers.

/// Contains callback based events through function pointers.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDAppEvents {
    /// Called once before starting the application event loop.
    pub start: Option<unsafe extern "C" fn()>,
}
impl NSTDAppEvents {
    /// Creates a new, all null, events structure.
    #[inline]
    pub(crate) fn new() -> Self {
        Self { start: None }
    }
}
