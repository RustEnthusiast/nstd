#ifndef NSTD_APP_DATA_H
#define NSTD_APP_DATA_H
#include "../nstd.h"

/// A handle to the application event loop.
typedef NSTDAny NSTDAppHandle;

/// Application data passed to each event.
typedef struct {
    /// A handle to the `nstd` app.
    NSTDAppHandle handle;
    /// Custom user data.
    NSTDAnyMut data;
    /// The application's control flow.
    NSTDAny control_flow;
} NSTDAppData;

#endif
