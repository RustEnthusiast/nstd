//! Unix time utilities.
use crate::{
    core::{
        optional::{gen_optional, NSTDOptional},
        time::{
            nstd_core_time_duration_get, nstd_core_time_duration_nanoseconds,
            nstd_core_time_duration_new, nstd_core_time_duration_seconds, NSTDDuration,
        },
    },
    NSTDFloat64, NSTDInt64, NSTDUInt32,
};
use core::mem::MaybeUninit;
use libc::{clock_gettime, timespec, CLOCK_REALTIME};
use nstdapi::nstdapi;

/// A structure representing system time since January 1st 1970.
#[nstdapi]
#[derive(Clone, Copy, PartialEq)]
pub struct NSTDUnixTime {
    /// The time span since January 1st 1970.
    duration: NSTDDuration,
}
impl From<timespec> for NSTDUnixTime {
    /// Converts a [timespec] into an [NSTDUnixTime] object.
    fn from(value: timespec) -> Self {
        const NANOS_IN_SEC: NSTDFloat64 = 1_000_000_000.0;
        let mut seconds = value.tv_sec as _;
        seconds += value.tv_nsec as NSTDFloat64 / NANOS_IN_SEC;
        Self {
            duration: nstd_core_time_duration_new(seconds),
        }
    }
}
gen_optional!(NSTDUnixOptionalTime, NSTDUnixTime);

/// Returns the current system time as an `NSTDUnixTime` object.
///
/// # Returns
///
/// `NSTDUnixOptionalTime time` - The current time on success, or an uninitialized "none" value on
/// failure.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_time_now() -> NSTDUnixOptionalTime {
    let mut timespec = MaybeUninit::uninit();
    // SAFETY: `clock_gettime` is safe.
    if unsafe { clock_gettime(CLOCK_REALTIME, timespec.as_mut_ptr()) } == 0 {
        // SAFETY: `timespec` is initialized.
        return NSTDOptional::Some(NSTDUnixTime::from(unsafe { timespec.assume_init() }));
    }
    NSTDOptional::None
}

/// Returns the number of seconds stored in an `NSTDUnixTime` object as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `NSTDUnixTime time` - The time object.
///
/// # Returns
///
/// `NSTDFloat64 seconds` - The number of seconds in a time object represented as an
/// `NSTDFloat64`.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_time_get(time: NSTDUnixTime) -> NSTDFloat64 {
    nstd_core_time_duration_get(time.duration)
}

/// Returns the number of seconds in an `NSTDUnixTime` object.
///
/// # Parameters:
///
/// - `NSTDUnixTime time` - The time object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_time_seconds(time: NSTDUnixTime) -> NSTDInt64 {
    nstd_core_time_duration_seconds(time.duration)
}

/// Returns the number of nanoseconds in an `NSTDUnixTime` object.
///
/// # Parameters:
///
/// - `NSTDUnixTime time` - The time object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_time_nanoseconds(time: NSTDUnixTime) -> NSTDUInt32 {
    nstd_core_time_duration_nanoseconds(time.duration)
}

/// Computes the addition of an `NSTDUnixTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `NSTDUnixTime time` - The time object
///
/// - `NSTDDuration duration` - The duration to add.
///
/// # Returns
///
/// `NSTDUnixTime time` - The result of the addition.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_time_add(time: NSTDUnixTime, duration: NSTDDuration) -> NSTDUnixTime {
    let secs = nstd_core_time_duration_get(time.duration) + nstd_core_time_duration_get(duration);
    NSTDUnixTime {
        duration: nstd_core_time_duration_new(secs),
    }
}

/// Computes the subtraction between an `NSTDUnixTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `NSTDUnixTime time` - The time object
///
/// - `NSTDDuration duration` - The duration to subtract.
///
/// # Returns
///
/// `NSTDUnixTime time` - The result of the subtraction.
#[inline]
#[nstdapi]
pub fn nstd_os_unix_time_sub(time: NSTDUnixTime, duration: NSTDDuration) -> NSTDUnixTime {
    let secs = nstd_core_time_duration_get(time.duration) - nstd_core_time_duration_get(duration);
    NSTDUnixTime {
        duration: nstd_core_time_duration_new(secs),
    }
}
