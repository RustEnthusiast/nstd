#ifndef NSTD_OS_UNIX_TIME_H
#define NSTD_OS_UNIX_TIME_H
#include "../../core/optional.h"
#include "../../core/time.h"
#include "../../nstd.h"

/// A structure representing system time since January 1st 1970.
typedef struct {
    /// The time span since January 1st 1970.
    NSTDDuration seconds;
} NSTDUnixTime;

/// Represents an optional value of type `NSTDUnixTime`.
NSTDOptional(NSTDUnixTime) NSTDUnixOptionalTime;

/// Returns the current system time as an `NSTDUnixTime` object.
///
/// # Returns
///
/// `NSTDUnixOptionalTime time` - The current time on success, or an uninitialized "none" value on
/// failure.
NSTDAPI NSTDUnixOptionalTime nstd_os_unix_time_now();

/// Returns the number of seconds stored in an `NSTDUnixTime` object as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDUnixTime *time` - The time object.
///
/// # Returns
///
/// `NSTDFloat64 seconds` - The number of seconds in a time object represented as an
/// `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_os_unix_time_get(const NSTDUnixTime *time);

/// Returns the number of seconds in an `NSTDUnixTime` object.
///
/// # Parameters:
///
/// - `const NSTDUnixTime *time` - The time object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `time`.
NSTDAPI NSTDInt64 nstd_os_unix_time_seconds(const NSTDUnixTime *time);

/// Returns the number of nanoseconds in an `NSTDUnixTime` object.
///
/// # Parameters:
///
/// - `const NSTDUnixTime *time` - The time object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `time`.
NSTDAPI NSTDUInt32 nstd_os_unix_time_nanoseconds(const NSTDUnixTime *time);

/// Computes the addition of an `NSTDUnixTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `const NSTDUnixTime *time` - The time object
///
/// - `const NSTDDuration *duration` - The duration to add.
///
/// # Returns
///
/// `NSTDUnixTime time` - The result of the addition.
NSTDAPI NSTDUnixTime nstd_os_unix_time_add(const NSTDUnixTime *time, const NSTDDuration *duration);

/// Computes the subtraction between an `NSTDUnixTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `const NSTDUnixTime *time` - The time object
///
/// - `const NSTDDuration *duration` - The duration to subtract.
///
/// # Returns
///
/// `NSTDUnixTime time` - The result of the subtraction.
NSTDAPI NSTDUnixTime nstd_os_unix_time_sub(const NSTDUnixTime *time, const NSTDDuration *duration);

#endif
