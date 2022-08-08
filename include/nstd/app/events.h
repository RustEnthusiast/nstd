#ifndef NSTD_APP_EVENTS_H
#define NSTD_APP_EVENTS_H
#include "../nstd.h"
#include "handle.h"

/// Contains callback based events through function pointers.
typedef struct {
    /// Called once before starting the application event loop.
    void (*start)(NSTDAppHandle);
    /// Called once before exiting the application event loop.
    void (*exit)(NSTDAppHandle);
} NSTDAppEvents;

#endif
