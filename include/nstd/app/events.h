#ifndef NSTD_APP_EVENTS_H
#define NSTD_APP_EVENTS_H
#include "../nstd.h"
#include "data.h"

/// A window's unique identifier.
typedef NSTDAnyConst NSTDWindowID;

/// A device's unique identifier.
typedef NSTDAnyConst NSTDDeviceID;

/// Contains callback based events through function pointers.
typedef struct {
    /// Called once before starting the application event loop.
    void (*start)(const NSTDAppData *);
    /// Called when all other events have been processed.
    void (*update)(const NSTDAppData *);
    /// Called when a new device is connected to the system.
    void (*device_added)(const NSTDAppData *, NSTDDeviceID);
    /// Called when a device was disconnected from the system.
    void (*device_removed)(const NSTDAppData *, NSTDDeviceID);
    /// A window requests closing.
    void (*window_close_requested)(const NSTDAppData *, NSTDWindowID);
    /// Called once before exiting the application event loop.
    void (*exit)(const NSTDAppData *);
} NSTDAppEvents;

#endif
