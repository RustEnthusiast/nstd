//! Application data passed to each event.
use crate::{app::handle::NSTDAppHandle, NSTDAnyMut};
use std::cell::Cell;
use winit::event_loop::ControlFlow;

/// Application data passed to each event.
#[repr(C)]
pub struct NSTDAppData<'a> {
    /// A handle to the `nstd` app.
    pub handle: NSTDAppHandle<'a>,
    /// Custom user data.
    pub data: NSTDAnyMut,
    /// The application's control flow.
    control_flow: &'a Cell<ControlFlow>,
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

    /// Returns a reference to the control flow cell.
    #[inline]
    pub(crate) fn control_flow(&self) -> &Cell<ControlFlow> {
        self.control_flow
    }
}
