//! Time utilities.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDFloat64, NSTDInt64, NSTDUInt32,
};
use nstdapi::nstdapi;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a span of time.
#[nstdapi]
#[derive(Clone, Copy, PartialEq)]
pub struct NSTDDuration {
    /// The duration in seconds.
    seconds: NSTDFloat64,
}
gen_optional!(NSTDOptionalDuration, NSTDDuration);

/// A structure representing system time since January 1st 1970.
#[nstdapi]
#[derive(Clone, Copy, PartialEq)]
pub struct NSTDTime {
    /// The time span since January 1st 1970.
    duration: NSTDDuration,
}
impl From<SystemTime> for NSTDTime {
    /// Converts a [SystemTime] into an [NSTDTime] object.
    fn from(value: SystemTime) -> Self {
        match value.duration_since(UNIX_EPOCH) {
            Ok(dur) => NSTDTime {
                duration: nstd_time_duration_new(dur.as_secs_f64()),
            },
            Err(dur) => NSTDTime {
                duration: nstd_time_duration_new(-dur.duration().as_secs_f64()),
            },
        }
    }
}
gen_optional!(NSTDOptionalTime, NSTDTime);

/// Returns the current system time as an `NSTDTime` object.
///
/// # Returns
///
/// `NSTDTime time` - The current time.
#[inline]
#[nstdapi]
pub fn nstd_time_now() -> NSTDTime {
    NSTDTime::from(SystemTime::now())
}

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
#[inline]
#[nstdapi]
pub fn nstd_time_get(time: &NSTDTime) -> NSTDFloat64 {
    time.duration.seconds
}

/// Returns the number of seconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_time_seconds(time: &NSTDTime) -> NSTDInt64 {
    nstd_time_duration_seconds(&time.duration)
}

/// Returns the number of nanoseconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_time_nanoseconds(time: &NSTDTime) -> NSTDUInt32 {
    nstd_time_duration_nanoseconds(&time.duration)
}

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
#[inline]
#[nstdapi]
pub fn nstd_time_add(time: &NSTDTime, duration: &NSTDDuration) -> NSTDTime {
    NSTDTime {
        duration: nstd_time_duration_new(time.duration.seconds + duration.seconds),
    }
}

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
#[inline]
#[nstdapi]
pub fn nstd_time_sub(time: &NSTDTime, duration: &NSTDDuration) -> NSTDTime {
    NSTDTime {
        duration: nstd_time_duration_new(time.duration.seconds - duration.seconds),
    }
}

/// Creates a new `NSTDDuration` object from an `NSTDFloat64` representing the duration in seconds.
///
/// # Parameters:
///
/// - `NSTDFloat64 seconds` - The time span in seconds.
///
/// # Returns
///
/// `NSTDDuration duration` - The time span represented as an `NSTDDuration` object.
#[inline]
#[nstdapi]
pub const fn nstd_time_duration_new(seconds: NSTDFloat64) -> NSTDDuration {
    NSTDDuration { seconds }
}

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
#[inline]
#[nstdapi]
pub const fn nstd_time_duration_get(duration: &NSTDDuration) -> NSTDFloat64 {
    duration.seconds
}

/// Returns the number of seconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `duration`.
#[inline]
#[nstdapi]
pub const fn nstd_time_duration_seconds(duration: &NSTDDuration) -> NSTDInt64 {
    duration.seconds as _
}

/// Returns the number of nanoseconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `duration`.
#[nstdapi]
pub fn nstd_time_duration_nanoseconds(duration: &NSTDDuration) -> NSTDUInt32 {
    const NANOS_IN_SEC: NSTDFloat64 = 1_000_000_000.0;
    let nanos = duration.seconds - duration.seconds as NSTDInt64 as NSTDFloat64;
    match nanos >= 0.0 {
        true => (nanos * NANOS_IN_SEC) as _,
        false => (nanos * -NANOS_IN_SEC) as _,
    }
}

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
#[inline]
#[nstdapi]
pub fn nstd_time_duration_add(lhs: &NSTDDuration, rhs: &NSTDDuration) -> NSTDDuration {
    nstd_time_duration_new(lhs.seconds + rhs.seconds)
}

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
#[inline]
#[nstdapi]
pub fn nstd_time_duration_sub(lhs: &NSTDDuration, rhs: &NSTDDuration) -> NSTDDuration {
    nstd_time_duration_new(lhs.seconds - rhs.seconds)
}
