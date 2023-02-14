#ifndef NSTD_TIME_H
#define NSTD_TIME_H
#include "core/optional.h"
#include "nstd.h"

/// A structure representing system time since January 1st 1970.
typedef struct {
    /// The number of seconds since January 1st 1970.
    NSTDInt64 secs;
    /// The remaining nanoseconds.
    NSTDUInt32 nanos;
} NSTDTime;

/// Represents an optional value of type `NSTDTime`.
NSTDOptional(NSTDTime) NSTDOptionalTime;

/// Represents a span of time.
typedef struct {
    /// The duration in seconds.
    NSTDInt64 secs;
    /// The nanoseconds.
    NSTDUInt32 nanos;
} NSTDDuration;

/// Represents an optional value of type `NSTDDuration`.
NSTDOptional(NSTDDuration) NSTDOptionalDuration;

/// Returns the current system time as an `NSTDTime` object.
///
/// # Returns
///
/// `NSTDTime time` - The current time.
NSTDAPI NSTDTime nstd_time_now();

#endif
