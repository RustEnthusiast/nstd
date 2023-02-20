#ifndef NSTD_TIME_H
#define NSTD_TIME_H
#include "core/optional.h"
#include "nstd.h"

/// Represents a span of time.
typedef struct {
    /// The duration in seconds.
    NSTDFloat64 seconds;
} NSTDDuration;

/// Represents an optional value of type `NSTDDuration`.
NSTDOptional(NSTDDuration) NSTDOptionalDuration;

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

/// Creates a new `NSTDDuration` object from an `NSTDFloat64` representing the duration in seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 seconds` - The time span in seconds.
///
/// # Returns
///
/// `NSTDDuration duration` - The time span represented as an `NSTDDuration` object.
NSTDAPI NSTDDuration nstd_time_duration_new(NSTDFloat64 seconds);

/// Returns the number of seconds stored in an `NSTDDuration` as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDFloat64 seconds` - The number of seconds in a duration object represented as an
/// `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_time_duration_get(const NSTDDuration *duration);

/// Returns the number of seconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `duration`.
NSTDAPI NSTDInt64 nstd_time_duration_seconds(const NSTDDuration *duration);

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

/// Computes the addition of two time spans.
///
/// # Parameters:
///
/// - `const NSTDDuration *lhs` - The left-hand side operand.
///
/// - `const NSTDDuration *rhs` - The right-hand side operand.
///
/// # Returns
///
/// `NSTDDuration duration` - The result of the time span addition.
NSTDAPI NSTDDuration nstd_time_duration_add(const NSTDDuration *lhs, const NSTDDuration *rhs);

/// Computes the subtraction between two time spans.
///
/// # Parameters:
///
/// - `const NSTDDuration *lhs` - The left-hand side operand.
///
/// - `const NSTDDuration *rhs` - The right-hand side operand.
///
/// # Returns
///
/// `NSTDDuration duration` - The result of the time span subtraction.
NSTDAPI NSTDDuration nstd_time_duration_sub(const NSTDDuration *lhs, const NSTDDuration *rhs);

#endif
