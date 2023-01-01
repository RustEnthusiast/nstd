#ifndef NSTD_APP_DATA_H
#define NSTD_APP_DATA_H
#include "../heap_ptr.h"
#include "../nstd.h"

/// A handle to the application event loop.
typedef NSTDAny NSTDAppHandle;

/// Application data passed to each event.
typedef struct {
    /// A handle to the `nstd` app.
    NSTDAppHandle handle;
    /// Custom user data.
    NSTDOptionalHeapPtr *data;
    /// The gamepad input manager.
    NSTDAnyMut gil;
    /// The application's control flow.
    NSTDAnyMut control_flow;
} NSTDAppData;

#endif
