#ifndef NSTD_APP_DATA_H
#define NSTD_APP_DATA_H
#include "../nstd.h"
#include "handle.h"
NSTDCPPSTART

/// Represents the control flow of an `nstd` application.
typedef enum {
    /// Exit the application.
    NSTD_APP_CONTROL_FLOW_EXIT,
    /// Poll for more events.
    NSTD_APP_CONTROL_FLOW_POLL
} NSTDAppControlFlow;

/// Application data passed to each event.
typedef struct {
    /// A handle to the `nstd` app.
    NSTDAppHandle handle;
    /// The application's control flow.
    NSTDAppControlFlow control_flow;
    /// Custom user data.
    NSTDAnyMut data;
} NSTDAppData;

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
NSTDAPI void nstd_app_data_set_control_flow(const NSTDAppData *app_data,
NSTDAppControlFlow control_flow);

NSTDCPPEND
#endif
