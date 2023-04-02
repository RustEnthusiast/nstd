//! Time utilities.
use crate::{
    core::time::{nstd_core_time_duration_new, NSTDDuration},
    NSTDFloat64, NSTDInt64, NSTDUInt32,
};
use cfg_if::cfg_if;
use nstdapi::nstdapi;
use std::time::{SystemTime, UNIX_EPOCH};

cfg_if! {
    if #[cfg(unix)] {
        use crate::os::unix::time::{
            nstd_os_unix_time_add, nstd_os_unix_time_get, nstd_os_unix_time_nanoseconds,
            nstd_os_unix_time_now, nstd_os_unix_time_seconds, nstd_os_unix_time_sub,
            NSTDUnixOptionalTime, NSTDUnixTime,
        };

        /// A structure representing system time since January 1st 1970.
        pub type NSTDTime = NSTDUnixTime;
        impl From<SystemTime> for NSTDTime {
            /// Converts a [SystemTime] into an [NSTDTime] object.
            fn from(value: SystemTime) -> Self {
                match value.duration_since(UNIX_EPOCH) {
                    Ok(dur) => NSTDTime::from_duration(
                        nstd_core_time_duration_new(dur.as_secs_f64()),
                    ),
                    Err(dur) => NSTDTime::from_duration(
                        nstd_core_time_duration_new(-dur.duration().as_secs_f64()),
                    ),
                }
            }
        }

        /// Represents an optional value of type `NSTDTime`.
        pub type NSTDOptionalTime = NSTDUnixOptionalTime;
    } else {
        use crate::core::{
            optional::{gen_optional, NSTDOptional},
            time::{
                nstd_core_time_duration_get, nstd_core_time_duration_nanoseconds,
                nstd_core_time_duration_seconds,
            },
        };

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
                        duration: nstd_core_time_duration_new(dur.as_secs_f64()),
                    },
                    Err(dur) => NSTDTime {
                        duration: nstd_core_time_duration_new(-dur.duration().as_secs_f64()),
                    },
                }
            }
        }
        gen_optional!(NSTDOptionalTime, NSTDTime);
    }
}

/// Returns the current system time as an `NSTDTime` object.
///
/// # Returns
///
/// `NSTDOptionalTime time` - The current time on success, or an uninitialized "none" variant on
/// failure.
#[inline]
#[nstdapi]
pub fn nstd_time_now() -> NSTDOptionalTime {
    #[cfg(unix)]
    return nstd_os_unix_time_now();
    #[cfg(not(unix))]
    return NSTDOptional::Some(NSTDTime::from(SystemTime::now()));
}

/// Returns the number of seconds stored in an `NSTDTime` object as an `NSTDFloat64`.
///
/// # Parameters:
///
/// - `NSTDTime time` - The time object.
///
/// # Returns
///
/// `NSTDFloat64 seconds` - The number of seconds in a time object represented as an
/// `NSTDFloat64`.
#[inline]
#[nstdapi]
pub fn nstd_time_get(time: NSTDTime) -> NSTDFloat64 {
    #[cfg(unix)]
    return nstd_os_unix_time_get(time);
    #[cfg(not(unix))]
    return nstd_core_time_duration_get(time.duration);
}

/// Returns the number of seconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `NSTDTime time` - The time object.
///
/// # Returns
///
/// `NSTDInt64 seconds` - The number of seconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_time_seconds(time: NSTDTime) -> NSTDInt64 {
    #[cfg(unix)]
    return nstd_os_unix_time_seconds(time);
    #[cfg(not(unix))]
    return nstd_core_time_duration_seconds(time.duration);
}

/// Returns the number of nanoseconds in an `NSTDTime` object.
///
/// # Parameters:
///
/// - `NSTDTime time` - The time object.
///
/// # Returns
///
/// `NSTDUInt32 nanoseconds` - The number of nanoseconds held in `time`.
#[inline]
#[nstdapi]
pub fn nstd_time_nanoseconds(time: NSTDTime) -> NSTDUInt32 {
    #[cfg(unix)]
    return nstd_os_unix_time_nanoseconds(time);
    #[cfg(not(unix))]
    return nstd_core_time_duration_nanoseconds(time.duration);
}

/// Computes the addition of an `NSTDTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `NSTDTime time` - The time object
///
/// - `NSTDDuration duration` - The duration to add.
///
/// # Returns
///
/// `NSTDTime time` - The result of the addition.
#[inline]
#[nstdapi]
pub fn nstd_time_add(time: NSTDTime, duration: NSTDDuration) -> NSTDTime {
    #[cfg(unix)]
    return nstd_os_unix_time_add(time, duration);
    #[cfg(not(unix))]
    {
        let s = nstd_core_time_duration_get(time.duration) + nstd_core_time_duration_get(duration);
        NSTDTime {
            duration: nstd_core_time_duration_new(s),
        }
    }
}

/// Computes the subtraction between an `NSTDTime` object and an `NSTDDuration`.
///
/// # Parameters:
///
/// - `NSTDTime time` - The time object
///
/// - `NSTDDuration duration` - The duration to subtract.
///
/// # Returns
///
/// `NSTDTime time` - The result of the subtraction.
#[inline]
#[nstdapi]
pub fn nstd_time_sub(time: NSTDTime, duration: NSTDDuration) -> NSTDTime {
    #[cfg(unix)]
    return nstd_os_unix_time_sub(time, duration);
    #[cfg(not(unix))]
    {
        let s = nstd_core_time_duration_get(time.duration) - nstd_core_time_duration_get(duration);
        NSTDTime {
            duration: nstd_core_time_duration_new(s),
        }
    }
}
