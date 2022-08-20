//! Application data passed to each event.
use crate::{app::handle::NSTDAppHandle, NSTDAnyMut};
use std::cell::Cell;
use winit::event_loop::ControlFlow;

/// Represents the control flow of an `nstd` application.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDAppControlFlow {
    /// Exit the application.
    NSTD_APP_CONTROL_FLOW_EXIT,
    /// Poll for more events.
    NSTD_APP_CONTROL_FLOW_POLL,
}
impl From<NSTDAppControlFlow> for ControlFlow {
    /// Creates a new instance of [ControlFlow] from an [NSTDAppControlFlow].
    #[inline]
    fn from(control_flow: NSTDAppControlFlow) -> ControlFlow {
        match control_flow {
            NSTDAppControlFlow::NSTD_APP_CONTROL_FLOW_EXIT => ControlFlow::Exit,
            NSTDAppControlFlow::NSTD_APP_CONTROL_FLOW_POLL => ControlFlow::Poll,
        }
    }
}

/// Application data passed to each event.
#[repr(C)]
pub struct NSTDAppData<'a> {
    /// A handle to the `nstd` app.
    pub handle: NSTDAppHandle<'a>,
    /// The application's control flow.
    pub control_flow: Cell<NSTDAppControlFlow>,
    /// Custom user data.
    pub data: NSTDAnyMut,
}
impl<'a> NSTDAppData<'a> {
    /// Creates a new instance of [NSTDAppData].
    #[inline]
    pub(crate) fn new(handle: NSTDAppHandle<'a>, data: NSTDAnyMut) -> Self {
        Self {
            handle,
            control_flow: Cell::new(NSTDAppControlFlow::NSTD_APP_CONTROL_FLOW_POLL),
            data,
        }
    }
}
