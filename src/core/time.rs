//! Low level time utilities.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDFloat64, NSTDInt64, NSTDUInt32,
};
use core::time::Duration;
use nstdapi::nstdapi;

/// Represents a span of time.
#[nstdapi]
#[derive(Clone, Copy, PartialEq)]
pub struct NSTDDuration {
    /// The duration in seconds.
    seconds: NSTDFloat64,
}
impl NSTDDuration {
    /// Converts an [NSTDDuration] into a [Duration].
    ///
    /// # Panics
    ///
    /// Panics if `duration` is negative, overflows Rust's `Duration` structure, or is non-finite.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn into_duration(self) -> Duration {
        Duration::from_secs_f64(self.seconds)
    }
}
gen_optional!(NSTDOptionalDuration, NSTDDuration);

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
pub const fn nstd_core_time_duration_new(seconds: NSTDFloat64) -> NSTDDuration {
    NSTDDuration { seconds }
}

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
#[inline]
#[nstdapi]
pub const fn nstd_core_time_duration_get(duration: NSTDDuration) -> NSTDFloat64 {
    duration.seconds
}

/// Returns the number of seconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `NSTDDuration duration` - The duration object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `duration`.
#[inline]
#[nstdapi]
pub const fn nstd_core_time_duration_seconds(duration: NSTDDuration) -> NSTDInt64 {
    duration.seconds as _
}

/// Returns the number of nanoseconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `NSTDDuration duration` - The duration object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `duration`.
#[nstdapi]
pub fn nstd_core_time_duration_nanoseconds(duration: NSTDDuration) -> NSTDUInt32 {
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
/// - `NSTDDuration lhs` - The left-hand side operand.
///
/// - `NSTDDuration rhs` - The right-hand side operand.
///
/// # Returns
///
/// `NSTDDuration duration` - The result of the time span addition.
#[inline]
#[nstdapi]
pub fn nstd_core_time_duration_add(lhs: NSTDDuration, rhs: NSTDDuration) -> NSTDDuration {
    nstd_core_time_duration_new(lhs.seconds + rhs.seconds)
}

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
#[inline]
#[nstdapi]
pub fn nstd_core_time_duration_sub(lhs: NSTDDuration, rhs: NSTDDuration) -> NSTDDuration {
    nstd_core_time_duration_new(lhs.seconds - rhs.seconds)
}
