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

/// Represents a key on a keyboard.
typedef enum {
    /// An unknown keyboard key.
    NSTD_KEY_UNKNOWN,
    /// The `esc` key.
    NSTD_KEY_ESCAPE,
    /// The `F1` key.
    NSTD_KEY_F1,
    /// The `F2` key.
    NSTD_KEY_F2,
    /// The `F3` key.
    NSTD_KEY_F3,
    /// The `F4` key.
    NSTD_KEY_F4,
    /// The `F5` key.
    NSTD_KEY_F5,
    /// The `F6` key.
    NSTD_KEY_F6,
    /// The `F7` key.
    NSTD_KEY_F7,
    /// The `F8` key.
    NSTD_KEY_F8,
    /// The `F9` key.
    NSTD_KEY_F9,
    /// The `F10` key.
    NSTD_KEY_F10,
    /// The `F11` key.
    NSTD_KEY_F11,
    /// The `F12` key.
    NSTD_KEY_F12,
    /// The `1` key.
    NSTD_KEY_1,
    /// The `2` key.
    NSTD_KEY_2,
    /// The `3` key.
    NSTD_KEY_3,
    /// The `4` key.
    NSTD_KEY_4,
    /// The `5` key.
    NSTD_KEY_5,
    /// The `6` key.
    NSTD_KEY_6,
    /// The `7` key.
    NSTD_KEY_7,
    /// The `8` key.
    NSTD_KEY_8,
    /// The `9` key.
    NSTD_KEY_9,
    /// The `0` key.
    NSTD_KEY_0,
    /// The `A` key.
    NSTD_KEY_A,
    /// The `B` key.
    NSTD_KEY_B,
    /// The `C` key.
    NSTD_KEY_C,
    /// The `D` key.
    NSTD_KEY_D,
    /// The `E` key.
    NSTD_KEY_E,
    /// The `F` key.
    NSTD_KEY_F,
    /// The `G` key.
    NSTD_KEY_G,
    /// The `H` key.
    NSTD_KEY_H,
    /// The `I` key.
    NSTD_KEY_I,
    /// The `J` key.
    NSTD_KEY_J,
    /// The `K` key.
    NSTD_KEY_K,
    /// The `L` key.
    NSTD_KEY_L,
    /// The `M` key.
    NSTD_KEY_M,
    /// The `N` key.
    NSTD_KEY_N,
    /// The `O` key.
    NSTD_KEY_O,
    /// The `P` key.
    NSTD_KEY_P,
    /// The `Q` key.
    NSTD_KEY_Q,
    /// The `R` key.
    NSTD_KEY_R,
    /// The `S` key.
    NSTD_KEY_S,
    /// The `T` key.
    NSTD_KEY_T,
    /// The `U` key.
    NSTD_KEY_U,
    /// The `V` key.
    NSTD_KEY_V,
    /// The `W` key.
    NSTD_KEY_W,
    /// The `X` key.
    NSTD_KEY_X,
    /// The `Y` key.
    NSTD_KEY_Y,
    /// The `Z` key.
    NSTD_KEY_Z,
    /// The ` key.
    NSTD_KEY_GRAVE,
    /// The `-` key.
    NSTD_KEY_MINUS,
    /// The `=` key.
    NSTD_KEY_EQUALS,
    /// The backspace key.
    NSTD_KEY_BACKSPACE,
    /// The tab key.
    NSTD_KEY_TAB,
    /// The `[` key.
    NSTD_KEY_OPEN_BRACKET,
    /// The `]` key.
    NSTD_KEY_CLOSE_BRACKET,
    /// The `\` key.
    NSTD_KEY_BACK_SLASH,
    /// The capital lock key.
    NSTD_KEY_CAPS_LOCK,
    /// The `;` key.
    NSTD_KEY_SEMICOLON,
    /// The `'` key.
    NSTD_KEY_APOSTROPHE,
    /// The enter key.
    NSTD_KEY_ENTER,
    /// The `,` key.
    NSTD_KEY_COMMA,
    /// The `.` key.
    NSTD_KEY_PERIOD,
    /// The `/` key.
    NSTD_KEY_FORWARD_SLASH,
    /// The space key.
    NSTD_KEY_SPACE,
    /// The left shift key.
    NSTD_KEY_LEFT_SHIFT,
    /// The left control key.
    NSTD_KEY_LEFT_CTRL,
    /// The left alt key.
    NSTD_KEY_LEFT_ALT,
    /// The right shift key.
    NSTD_KEY_RIGHT_SHIFT,
    /// The right control key.
    NSTD_KEY_RIGHT_CTRL,
    /// The right alt key.
    NSTD_KEY_RIGHT_ALT
} NSTDKey;

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
    void (*button_input)(const NSTDAppData *, NSTDDeviceID, NSTDButtonID, NSTDBool);
    /// Called when a keyboard key is pressed or unpressed.
    void (*key_input)(const NSTDAppData *, NSTDDeviceID, NSTDKey, NSTDUInt32, NSTDBool);
    /// Called when a window is resized.
    void (*window_resized)(const NSTDAppData *, NSTDWindowID, NSTDUInt32, NSTDUInt32);
    /// Called when a window is moved.
    void (*window_moved)(const NSTDAppData *, NSTDWindowID, NSTDInt32, NSTDInt32);
    /// Focus for a window changed.
    void (*window_focus_changed)(const NSTDAppData *, NSTDWindowID, NSTDBool);
    /// Called when the cursor is moved over a window.
    void (*window_cursor_moved)(const NSTDAppData *, NSTDWindowID, NSTDDeviceID, NSTDFloat64,
    NSTDFloat64);
    /// The cursor entered a window.
    void (*window_cursor_entered)(const NSTDAppData *, NSTDWindowID, NSTDDeviceID);
    /// The cursor left a window.
    void (*window_cursor_left)(const NSTDAppData *, NSTDWindowID, NSTDDeviceID);
    /// A window requests closing.
    void (*window_close_requested)(const NSTDAppData *, NSTDWindowID);
    /// Called when a window is closed.
    void (*window_closed)(const NSTDAppData *, NSTDWindowID);
    /// Called once before exiting the application event loop.
    void (*exit)(const NSTDAppData *);
} NSTDAppEvents;

#endif
