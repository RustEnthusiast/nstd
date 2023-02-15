//! Time utilities.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDInt64, NSTDUInt32, NSTDUInt64,
};
use nstdapi::nstdapi;
use std::time::{SystemTime, UNIX_EPOCH};

/// A structure representing system time since January 1st 1970.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDTime {
    /// The number of seconds since January 1st 1970.
    seconds: NSTDInt64,
    /// The remaining nanoseconds.
    nanoseconds: NSTDInt64,
}
impl From<SystemTime> for NSTDTime {
    /// Converts a [SystemTime] into an [NSTDTime] object.
    fn from(value: SystemTime) -> Self {
        match value.duration_since(UNIX_EPOCH) {
            Ok(dur) => NSTDTime {
                seconds: dur.as_secs() as _,
                nanoseconds: dur.subsec_nanos() as _,
            },
            Err(dur) => {
                let dur = dur.duration();
                let seconds = -(dur.as_secs() as NSTDInt64);
                NSTDTime {
                    seconds,
                    nanoseconds: match seconds {
                        0 => -(dur.subsec_nanos() as NSTDInt64),
                        _ => dur.subsec_nanos() as _,
                    },
                }
            }
        }
    }
}
gen_optional!(NSTDOptionalTime, NSTDTime);

/// Represents a span of time.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDDuration {
    /// The duration in seconds.
    seconds: NSTDUInt64,
    /// The nanoseconds.
    nanoseconds: NSTDUInt32,
}
gen_optional!(NSTDOptionalDuration, NSTDDuration);

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
    time.seconds
}

/// Returns the number of nanoseconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `const NSTDTime *time` - The time object.
///
/// # Returns
///
/// `NSTDInt64 nanoseconds` - The number of nanoseconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_time_nanoseconds(time: &NSTDTime) -> NSTDInt64 {
    time.nanoseconds
}

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
#[inline]
#[nstdapi]
pub const fn nstd_time_duration_new(
    mut seconds: NSTDUInt64,
    mut nanoseconds: NSTDUInt32,
) -> NSTDDuration {
    /// The number of nanoseconds in a whole second.
    const NANOS_IN_SEC: NSTDUInt32 = 1_000_000_000;
    let extra_secs = nanoseconds / NANOS_IN_SEC;
    nanoseconds -= NANOS_IN_SEC * extra_secs;
    seconds += extra_secs as NSTDUInt64;
    NSTDDuration {
        seconds,
        nanoseconds,
    }
}

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
#[inline]
#[nstdapi]
pub const unsafe fn nstd_time_duration_new_unchecked(
    seconds: NSTDUInt64,
    nanoseconds: NSTDUInt32,
) -> NSTDDuration {
    NSTDDuration {
        seconds,
        nanoseconds,
    }
}

/// Returns the number of seconds in an `NSTDDuration` object.
///
/// # Parameters:
///
/// - `const NSTDDuration *duration` - The duration object.
///
/// # Returns
///
/// `NSTDUInt64 seconds` - The number of seconds held in `duration`.
#[inline]
#[nstdapi]
pub const fn nstd_time_duration_seconds(duration: &NSTDDuration) -> NSTDUInt64 {
    duration.seconds
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
#[inline]
#[nstdapi]
pub const fn nstd_time_duration_nanoseconds(duration: &NSTDDuration) -> NSTDUInt32 {
    duration.nanoseconds
}
