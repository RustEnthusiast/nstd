#ifndef NSTD_APP_EVENTS_H
#define NSTD_APP_EVENTS_H
#include "../nstd.h"
#include "data.h"

/// A window's unique identifier.
typedef NSTDAnyConst NSTDWindowID;

/// A device's unique identifier.
typedef NSTDAnyConst NSTDDeviceID;

/// Identifier for an analog axis on a device.
typedef NSTDAnyConst NSTDAnalogAxisID;

/// A button's unique identifier.
typedef NSTDAnyConst NSTDButtonID;

/// Describes a mouse wheel's scroll delta.
typedef enum {
    /// The scroll was measured in lines.
    NSTD_SCROLL_DELTA_LINE,
    /// The scroll was measured in pixels.
    NSTD_SCROLL_DELTA_PIXEL
} NSTDScrollDelta;

/// Describes the state of a button.
typedef enum {
    /// The button is up.
    NSTD_BUTTON_STATE_RELEASED,
    /// The button is pressed down.
    NSTD_BUTTON_STATE_PRESSED,
} NSTDButtonState;

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
    /// Called when a mouse device is moved.
    void (*mouse_moved)(const NSTDAppData *, NSTDDeviceID, NSTDFloat64, NSTDFloat64);
    /// Called when a scroll wheel is scrolled.
    void (*mouse_scrolled)(const NSTDAppData *, NSTDDeviceID, NSTDFloat64, NSTDFloat64,
    NSTDScrollDelta);
    /// Called when there is some motion on an analog axis device, such as a touchpad.
    ///
    /// # Note
    ///
    /// Some touchpads can return a negative y value.
    void (*axis_motion)(const NSTDAppData *, NSTDDeviceID, NSTDAnalogAxisID, NSTDFloat64);
    /// Called when a button, such as a mouse button's state changes.
    void (*button_changed)(const NSTDAppData *, NSTDDeviceID, NSTDButtonID, NSTDButtonState);
    /// A window requests closing.
    void (*window_close_requested)(const NSTDAppData *, NSTDWindowID);
    /// Called once before exiting the application event loop.
    void (*exit)(const NSTDAppData *);
} NSTDAppEvents;

#endif
