#ifndef NSTD_TIME_H
#define NSTD_TIME_H
#include "nstd.h"

/// A structure representing system time since January 1st 1970.
typedef struct {
    /// The number of seconds since January 1st 1970.
    NSTDInt64 secs;
    /// The remaining nanoseconds.
    NSTDUInt32 nanos;
} NSTDTime;

/// Returns the current system time as an `NSTDTime` object.
///
/// # Returns
///
/// `NSTDTime time` - The current time.
NSTDAPI NSTDTime nstd_time_now();

#endif
