//! Application data passed to each event.
use crate::{app::handle::NSTDAppHandle, NSTDAnyMut};
use std::cell::Cell;
use winit::event_loop::ControlFlow;

/// Application data passed to each event.
#[repr(C)]
pub struct NSTDAppData<'a> {
    /// A handle to the `nstd` app.
    pub handle: NSTDAppHandle<'a>,
    /// The application's control flow.
    pub control_flow: &'a Cell<ControlFlow>,
    /// Custom user data.
    pub data: NSTDAnyMut,
}
impl<'a> NSTDAppData<'a> {
    /// Creates a new instance of [NSTDAppData].
    #[inline]
    pub(crate) fn new(
        handle: NSTDAppHandle<'a>,
        control_flow: &'a mut ControlFlow,
        data: NSTDAnyMut,
    ) -> Self {
        Self {
            handle,
            control_flow: Cell::from_mut(control_flow),
            data,
        }
    }
}
