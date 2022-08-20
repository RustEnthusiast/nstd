#ifndef NSTD_APP_DATA_H
#define NSTD_APP_DATA_H
#include "../nstd.h"
#include "handle.h"

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

#endif
