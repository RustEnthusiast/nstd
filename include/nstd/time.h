#ifndef NSTD_TIME_H
#define NSTD_TIME_H
#include "core/optional.h"
#include "core/time.h"
#include "nstd.h"

/// A structure representing system time since January 1st 1970.
typedef struct {
    /// The time span since January 1st 1970.
    NSTDDuration duration;
} NSTDTime;

/// Represents an optional value of type `NSTDTime`.
NSTDOptional(NSTDTime) NSTDOptionalTime;

/// Returns the current system time as an `NSTDTime` object.
///
/// # Returns
///
/// `NSTDTime time` - The current time.
NSTDAPI NSTDTime nstd_time_now();

/// Returns the number of seconds stored in an `NSTDTime` object as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object.
///
/// # Returns
///
/// `NSTDFloat64 seconds` - The number of seconds in a time object represented as an
/// `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_time_get(const NSTDTime *time);

/// Returns the number of seconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `time`.
NSTDAPI NSTDInt64 nstd_time_seconds(const NSTDTime *time);

/// Returns the number of nanoseconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `time`.
NSTDAPI NSTDUInt32 nstd_time_nanoseconds(const NSTDTime *time);

/// Computes the addition of an `NSTDTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object
///
/// - `const NSTDDuration *duration` - The duration to add.
///
/// # Returns
///
/// `NSTDTime time` - The result of the addition.
NSTDAPI NSTDTime nstd_time_add(const NSTDTime *time, const NSTDDuration *duration);

/// Computes the subtraction between an `NSTDTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object
///
/// - `const NSTDDuration *duration` - The duration to subtract.
///
/// # Returns
///
/// `NSTDTime time` - The result of the subtraction.
NSTDAPI NSTDTime nstd_time_sub(const NSTDTime *time, const NSTDDuration *duration);

#endif
