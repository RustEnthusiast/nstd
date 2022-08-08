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
impl Into<ControlFlow> for NSTDAppControlFlow {
    /// Creates a new instance of [ControlFlow] from an [NSTDAppControlFlow].
    #[inline]
    fn into(self) -> ControlFlow {
        match self {
            Self::NSTD_APP_CONTROL_FLOW_EXIT => ControlFlow::Exit,
            Self::NSTD_APP_CONTROL_FLOW_POLL => ControlFlow::Poll,
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

/// Sets an `nstd` application's control flow through it's `NSTDAppData`.
///
/// # Note
///
/// This will have no effect in the `start` event.
///
/// # Parameters:
///
/// - `const NSTDAppData *app_data` - The application data.
///
/// - `NSTDAppControlFlow control_flow` - The new application control flow.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_app_data_set_control_flow(
    app_data: &NSTDAppData,
    control_flow: NSTDAppControlFlow,
) {
    app_data.control_flow.set(control_flow);
}
