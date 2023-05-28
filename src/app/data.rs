//! Application data passed to each event.
use crate::heap_ptr::NSTDOptionalHeapPtr;
use gilrs::{Event as GamepadEvent, Gilrs};
use nstdapi::nstdapi;
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

/// A handle to the application event loop.
pub type NSTDAppHandle<'a> = &'a EventLoopWindowTarget<()>;

/// Application data passed to each event.
#[nstdapi]
pub struct NSTDAppData<'a> {
    /// A handle to the `nstd` app.
    pub handle: NSTDAppHandle<'a>,
    /// Custom user data.
    pub data: &'a mut NSTDOptionalHeapPtr<'static>,
    /// The gamepad input manager.
    gil: &'a mut Gilrs,
    /// The application's control flow.
    control_flow: &'a mut ControlFlow,
}
impl<'a> NSTDAppData<'a> {
    /// Creates a new instance of [NSTDAppData].
    #[inline]
    pub(crate) fn new(
        handle: NSTDAppHandle<'a>,
        control_flow: &'a mut ControlFlow,
        data: &'a mut NSTDOptionalHeapPtr<'static>,
        gil: &'a mut Gilrs,
    ) -> Self {
        Self {
            handle,
            control_flow,
            data,
            gil,
        }
    }

    /// Returns a reference to the control flow cell.
    #[inline]
    pub(crate) fn control_flow(&mut self) -> &mut ControlFlow {
        self.control_flow
    }

    /// Returns the next gamepad event if there is one.
    #[inline]
    pub(crate) fn next_gamepad_event(&mut self) -> Option<GamepadEvent> {
        self.gil.next_event()
    }
}
