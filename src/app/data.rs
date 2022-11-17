//! Application data passed to each event.
use crate::heap_ptr::NSTDHeapPtr;
use gilrs::{Event as GamepadEvent, Gilrs};
use std::cell::Cell;
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

/// A handle to the application event loop.
pub type NSTDAppHandle<'a> = &'a EventLoopWindowTarget<()>;

/// Application data passed to each event.
#[repr(C)]
pub struct NSTDAppData<'a, 'b> {
    /// A handle to the `nstd` app.
    pub handle: NSTDAppHandle<'a>,
    /// Custom user data.
    pub data: &'a mut NSTDHeapPtr,
    /// The gamepad input manager.
    gil: &'a mut Gilrs,
    /// The application's control flow.
    control_flow: &'b Cell<ControlFlow>,
}
impl<'a, 'b> NSTDAppData<'a, 'b> {
    /// Creates a new instance of [NSTDAppData].
    #[inline]
    pub(crate) fn new(
        handle: NSTDAppHandle<'a>,
        control_flow: &'b mut ControlFlow,
        data: &'a mut NSTDHeapPtr,
        gil: &'a mut Gilrs,
    ) -> Self {
        Self {
            handle,
            control_flow: Cell::from_mut(control_flow),
            data,
            gil,
        }
    }

    /// Returns a reference to the control flow cell.
    #[inline]
    pub(crate) fn control_flow(&self) -> &Cell<ControlFlow> {
        self.control_flow
    }

    /// Returns the next gamepad event if there is one.
    #[inline]
    pub(crate) fn next_gamepad_event(&mut self) -> Option<GamepadEvent> {
        self.gil.next_event()
    }
}
