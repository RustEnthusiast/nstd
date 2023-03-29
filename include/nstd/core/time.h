#ifndef NSTD_CORE_TIME_H
#define NSTD_CORE_TIME_H
#include "../nstd.h"
#include "optional.h"

/// Represents a span of time.
typedef struct {
    /// The duration in seconds.
    NSTDFloat64 seconds;
} NSTDDuration;

/// Represents an optional value of type `NSTDDuration`.
NSTDOptional(NSTDDuration) NSTDOptionalDuration;

/// Creates a new `NSTDDuration` object from an `NSTDFloat64` representing the duration in seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 seconds` - The time span in seconds.
///
/// # Returns
///
/// `NSTDDuration duration` - The time span represented as an `NSTDDuration` object.
NSTDAPI NSTDDuration nstd_core_time_duration_new(NSTDFloat64 seconds);

/// Returns the number of seconds stored in an `NSTDDuration` as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `NSTDDuration duration` - The duration object.
///
/// # Returns
///
/// `NSTDFloat64 seconds` - The number of seconds in a duration object represented as an
/// `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_time_duration_get(NSTDDuration duration);

/// Returns the number of seconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `NSTDDuration duration` - The duration object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `duration`.
NSTDAPI NSTDInt64 nstd_core_time_duration_seconds(NSTDDuration duration);

/// Returns the number of nanoseconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `NSTDDuration duration` - The duration object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `duration`.
NSTDAPI NSTDUInt32 nstd_core_time_duration_nanoseconds(NSTDDuration duration);

/// Computes the addition of two time spans.
///
/// # Parameters:
///
/// - `NSTDDuration lhs` - The left-hand side operand.
///
/// - `NSTDDuration rhs` - The right-hand side operand.
///
/// # Returns
///
/// `NSTDDuration duration` - The result of the time span addition.
NSTDAPI NSTDDuration nstd_core_time_duration_add(NSTDDuration lhs, NSTDDuration rhs);

/// Computes the subtraction between two time spans.
///
/// # Parameters:
///
/// - `NSTDDuration lhs` - The left-hand side operand.
///
/// - `NSTDDuration rhs` - The right-hand side operand.
///
/// # Returns
///
/// `NSTDDuration duration` - The result of the time span subtraction.
NSTDAPI NSTDDuration nstd_core_time_duration_sub(NSTDDuration lhs, NSTDDuration rhs);

#endif
