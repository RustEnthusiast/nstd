#ifndef NSTD_APP_EVENTS_H
#define NSTD_APP_EVENTS_H
#include "../core/optional.h"
#include "../core/str.h"
#include "../core/unichar.h"
#include "../heap_ptr.h"
#include "../nstd.h"

/// A window's unique identifier.
typedef NSTDAnyMut NSTDWindowID;

/// A device's unique identifier.
typedef NSTDAnyMut NSTDDeviceID;

/// A gamepad's unique identifier.
typedef NSTDAnyMut NSTDGamepadID;

/// Identifier for an analog axis on a device.
typedef NSTDUInt32 NSTDAnalogAxisID;

/// A button's unique identifier.
typedef NSTDUInt32 NSTDButtonID;

/// An enumeration of device event filtering modes.
typedef enum {
    /// Never dispatch device events.
    NSTD_DEVICE_EVENT_FILTER_NONE,
    /// Only dispatch device events when an application window is focused.
    NSTD_DEVICE_EVENT_FILTER_UNFOCUSED,
    /// Always dispatch device events.
    NSTD_DEVICE_EVENT_FILTER_ALL
} NSTDDeviceEventFilter;

/// Describes a mouse wheel's scroll delta.
typedef enum {
    /// The scroll was measured in lines.
    NSTD_SCROLL_DELTA_LINE,
    /// The scroll was measured in pixels.
    NSTD_SCROLL_DELTA_PIXEL
} NSTDScrollDelta;

/// Describes a touch-screen's state.
typedef enum {
    /// The touch event has just started.
    NSTD_TOUCH_STATE_STARTED,
    /// The touch position has been moved.
    NSTD_TOUCH_STATE_MOVED,
    /// The touch event has ended.
    NSTD_TOUCH_STATE_ENDED,
    /// The touch event has been cancelled.
    NSTD_TOUCH_STATE_CANCELLED
} NSTDTouchState;

/// Represents a mouse button.
typedef enum {
    /// The left mouse button.
    NSTD_MOUSE_BUTTON_LEFT,
    /// The middle mouse button.
    NSTD_MOUSE_BUTTON_MIDDLE,
    /// The right mouse button.
    NSTD_MOUSE_BUTTON_RIGHT,
    /// An extra mouse button.
    NSTD_MOUSE_BUTTON_OTHER
} NSTDMouseButton;

/// Represents some type of mouse button input.
typedef struct {
    /// The mouse button that received input.
    NSTDMouseButton button;
    /// The ID of the mouse button that received input.
    NSTDOptionalUInt16 id;
} NSTDMouseInput;

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

/// Represents a gamepad button.
typedef enum {
    /// The upper action pad button.
    ///
    /// Corresponds to the `Y` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_NORTH,
    /// The lower action pad button.
    ///
    /// Corresponds to the `A` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_SOUTH,
    /// The right action pad button.
    ///
    /// Corresponds to the `B` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_EAST,
    /// The left action pad button.
    ///
    /// Corresponds to the `X` button on Xbox controllers.
    NSTD_GAMEPAD_BUTTON_WEST,
    /// The right bumper.
    ///
    /// Corresponds to `RB` on Xbox controllers & `R1` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_RIGHT_BUMPER,
    /// The left bumper.
    ///
    /// Corresponds to `LB` on Xbox controllers & `L1` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_LEFT_BUMPER,
    /// The right trigger.
    ///
    /// Corresponds to `RT` on Xbox controllers & `R2` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_RIGHT_TRIGGER,
    /// The left trigger.
    ///
    /// Corresponds to `LT` on Xbox controllers & `L2` on Playstation controllers.
    NSTD_GAMEPAD_BUTTON_LEFT_TRIGGER,
    /// The start/pause button.
    NSTD_GAMEPAD_BUTTON_START,
    /// The select/back button.
    NSTD_GAMEPAD_BUTTON_SELECT,
    /// The right thumb stick.
    NSTD_GAMEPAD_BUTTON_RIGHT_THUMB,
    /// The left thumb stick.
    NSTD_GAMEPAD_BUTTON_LEFT_THUMB,
    /// The upper direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_UP,
    /// The lower direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_DOWN,
    /// The right direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_RIGHT,
    /// The left direction pad button.
    NSTD_GAMEPAD_BUTTON_DPAD_LEFT,
    /// An unrecognized button.
    NSTD_GAMEPAD_BUTTON_UNKNOWN
} NSTDGamepadButton;

/// Represents a gamepad axis.
typedef enum {
    /// The left stick x-axis.
    NSTD_GAMEPAD_AXIS_LEFT_X,
    /// The left stick y-axis.
    NSTD_GAMEPAD_AXIS_LEFT_Y,
    /// The left stick z-axis.
    NSTD_GAMEPAD_AXIS_LEFT_Z,
    /// The right stick x-axis.
    NSTD_GAMEPAD_AXIS_RIGHT_X,
    /// The right stick y-axis.
    NSTD_GAMEPAD_AXIS_RIGHT_Y,
    /// The right stick z-axis.
    NSTD_GAMEPAD_AXIS_RIGHT_Z,
    /// Left or right on the direction pad.
    NSTD_GAMEPAD_AXIS_DPAD_X,
    /// Up or down on the direction pad.
    NSTD_GAMEPAD_AXIS_DPAD_Y,
    /// An unknown axis.
    NSTD_GAMEPAD_AXIS_UNKNOWN
} NSTDGamepadAxis;

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

/// Contains callback based events through function pointers.
typedef struct {
    /// Called once before starting the application event loop.
    void (*start)(NSTDAppData *);
    /// Called when all other events have been processed.
    void (*update)(NSTDAppData *);
    /// Called when a new device is connected to the system.
    void (*device_added)(NSTDAppData *, NSTDDeviceID);
    /// Called when a device was disconnected from the system.
    void (*device_removed)(NSTDAppData *, NSTDDeviceID);
    /// Called when a mouse device is moved.
    void (*mouse_moved)(NSTDAppData *, NSTDDeviceID, NSTDFloat64, NSTDFloat64);
    /// Called when a scroll wheel is scrolled.
    void (*mouse_scrolled)(NSTDAppData *, NSTDDeviceID, NSTDFloat64, NSTDFloat64, NSTDScrollDelta);
    /// Called when there is some motion on an analog axis device, such as a touchpad.
    ///
    /// # Note
    ///
    /// Some touchpads can return a negative y value.
    void (*axis_motion)(NSTDAppData *, NSTDDeviceID, NSTDAnalogAxisID, NSTDFloat64);
    /// Called when a button, such as a mouse button's state changes.
    void (*button_input)(NSTDAppData *, NSTDDeviceID, NSTDButtonID, NSTDBool);
    /// Called when a keyboard key is pressed or unpressed.
    void (*key_input)(NSTDAppData *, NSTDDeviceID, NSTDKey, NSTDUInt32, NSTDBool);
    /// Called when a window's scale factor changes.
    void (*window_dpi_changed)(NSTDAppData *, NSTDWindowID, NSTDFloat64, NSTDUInt32 *,
    NSTDUInt32 *);
    /// Called when a window is resized.
    void (*window_resized)(NSTDAppData *, NSTDWindowID, NSTDUInt32, NSTDUInt32);
    /// Called when a window is moved.
    void (*window_moved)(NSTDAppData *, NSTDWindowID, NSTDInt32, NSTDInt32);
    /// Focus for a window changed.
    void (*window_focus_changed)(NSTDAppData *, NSTDWindowID, NSTDBool);
    /// Mouse input was received.
    void (*window_mouse_input)(NSTDAppData *, NSTDWindowID, NSTDDeviceID, const NSTDMouseInput *,
    NSTDBool);
    /// Called when a window receives key input.
    void (*window_key_input)(NSTDAppData *, NSTDWindowID, NSTDDeviceID, NSTDKey, NSTDUInt32,
    NSTDBool);
    /// Called when a window receives a character.
    void (*window_received_char)(NSTDAppData *, NSTDWindowID, NSTDUnichar);
    /// Called when a scroll device is scrolled over a window.
    void (*window_scrolled)(NSTDAppData *, NSTDWindowID, NSTDDeviceID, NSTDFloat64, NSTDFloat64,
    NSTDScrollDelta, NSTDTouchState);
    /// Called when the cursor is moved over a window.
    void (*window_cursor_moved)(NSTDAppData *, NSTDWindowID, NSTDDeviceID, NSTDFloat64,
    NSTDFloat64);
    /// The cursor entered a window.
    void (*window_cursor_entered)(NSTDAppData *, NSTDWindowID, NSTDDeviceID);
    /// The cursor left a window.
    void (*window_cursor_left)(NSTDAppData *, NSTDWindowID, NSTDDeviceID);
    /// A file was dropped into a window.
    void (*window_file_received)(NSTDAppData *, NSTDWindowID, const NSTDStr *);
    /// A file was hovered over a window.
    void (*window_file_hovered)(NSTDAppData *, NSTDWindowID, const NSTDStr *);
    /// A file was dragged away from a window.
    void (*window_file_canceled)(NSTDAppData *, NSTDWindowID);
    /// A window requests closing.
    void (*window_close_requested)(NSTDAppData *, NSTDWindowID);
    /// Called when a window is closed.
    void (*window_closed)(NSTDAppData *, NSTDWindowID);
    /// A gamepad was connected to the system.
    void (*gamepad_connected)(NSTDAppData *, NSTDGamepadID);
    /// A gamepad was disconnected to the system.
    void (*gamepad_disconnected)(NSTDAppData *, NSTDGamepadID);
    /// A gamepad button was pressed.
    void (*gamepad_button_pressed)(NSTDAppData *, NSTDGamepadID, NSTDGamepadButton, NSTDUInt32);
    /// A gamepad button was released.
    void (*gamepad_button_released)(NSTDAppData *, NSTDGamepadID, NSTDGamepadButton, NSTDUInt32);
    /// A gamepad button's value changed.
    void (*gamepad_input)(NSTDAppData *, NSTDGamepadID, NSTDGamepadButton, NSTDUInt32, NSTDFloat32);
    /// A gamepad axis value has changed.
    void (*gamepad_axis_input)(NSTDAppData *, NSTDGamepadID, NSTDGamepadAxis, NSTDUInt32,
    NSTDFloat32);
    /// Called once before exiting the application event loop.
    void (*exit)(NSTDAppData *);
} NSTDAppEvents;

#endif
