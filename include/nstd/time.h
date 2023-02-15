#ifndef NSTD_TIME_H
#define NSTD_TIME_H
#include "core/optional.h"
#include "nstd.h"

/// A structure representing system time since January 1st 1970.
typedef struct {
    /// The number of seconds since January 1st 1970.
    NSTDInt64 seconds;
    /// The remaining nanoseconds.
    NSTDInt64 nanoseconds;
} NSTDTime;

/// Represents an optional value of type `NSTDTime`.
NSTDOptional(NSTDTime) NSTDOptionalTime;

/// Represents a span of time.
typedef struct {
    /// The duration in seconds.
    NSTDUInt64 seconds;
    /// The nanoseconds.
    NSTDUInt32 nanoseconds;
} NSTDDuration;

/// Represents an optional value of type `NSTDDuration`.
NSTDOptional(NSTDDuration) NSTDOptionalDuration;

/// Returns the current system time as an `NSTDTime` object.
///
/// # Returns
///
/// `NSTDTime time` - The current time.
NSTDAPI NSTDTime nstd_time_now();

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
/// `NSTDInt64 nanoseconds` - The number of nanoseconds held in `time`.
NSTDAPI NSTDInt64 nstd_time_nanoseconds(const NSTDTime *time);

/// Creates a new `NSTDDuration` object from seconds and nanoseconds.
///
/// # Parameters:
///
/// - `NSTDUInt64 seconds` - The time span in seconds.
///
/// - `NSTDUInt32 nanoseconds` - The remaining nanoseconds.
///
/// # Returns
///
/// `NSTDDuration duration` - The time span represented as an `NSTDDuration` object.
NSTDAPI NSTDDuration nstd_time_duration_new(NSTDUInt64 seconds, NSTDUInt32 nanoseconds);

/// Creates a new `NSTDDuration` object from seconds and nanoseconds without checking if the number
/// of nanoseconds will overflow into the number of seconds.
///
/// # Parameters:
///
/// - `NSTDUInt64 seconds` - The time span in seconds.
///
/// - `NSTDUInt32 nanoseconds` - The remaining nanoseconds.
///
/// # Returns
///
/// `NSTDDuration duration` - The time span represented as an `NSTDDuration` object.
///
/// # Safety
///
/// This operation saves time by skipping the `nanoseconds` check. The user of this function must
/// ensure that the number of `nanoseconds` cannot overflow into `seconds`.
NSTDAPI NSTDDuration nstd_time_duration_new_unchecked(NSTDUInt64 seconds, NSTDUInt32 nanoseconds);

/// Returns the number of seconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDUInt64 seconds` - The number of seconds held in `duration`.
NSTDAPI NSTDUInt64 nstd_time_duration_seconds(const NSTDDuration *duration);

/// Returns the number of nanoseconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `duration`.
NSTDAPI NSTDUInt32 nstd_time_duration_nanoseconds(const NSTDDuration *duration);

#endif
