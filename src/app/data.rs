//! Application data passed to each event.
use crate::{app::handle::NSTDAppHandle, NSTDAnyMut};

/// Application data passed to each event.
#[repr(C)]
pub struct NSTDAppData<'a> {
    /// A handle to the `nstd` app.
    pub handle: NSTDAppHandle<'a>,
    /// Custom user data.
    pub data: NSTDAnyMut,
}
impl<'a> NSTDAppData<'a> {
    /// Creates a new instance of [NSTDAppData].
    #[inline]
    pub(crate) fn new(handle: NSTDAppHandle<'a>, data: NSTDAnyMut) -> Self {
        Self { handle, data }
    }
}
