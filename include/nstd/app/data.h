#ifndef NSTD_APP_DATA_H
#define NSTD_APP_DATA_H
#include "../nstd.h"
#include "handle.h"

/// Application data passed to each event.
typedef struct {
    /// A handle to the `nstd` app.
    NSTDAppHandle handle;
    /// Custom user data.
    NSTDAnyMut data;
} NSTDAppData;

#endif
