#ifndef NSTD_APP_EVENTS_H
#define NSTD_APP_EVENTS_H
#include "../nstd.h"
#include "data.h"

/// Contains callback based events through function pointers.
typedef struct {
    /// Called once before starting the application event loop.
    void (*start)(const NSTDAppData *);
    /// Called when all other events have been processed.
    void (*update)(const NSTDAppData *);
    /// Called once before exiting the application event loop.
    void (*exit)(const NSTDAppData *);
} NSTDAppEvents;

#endif
