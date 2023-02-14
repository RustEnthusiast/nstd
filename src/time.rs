//! Time utilities.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDInt64, NSTDUInt32,
};
use nstdapi::nstdapi;
use std::time::{SystemTime, UNIX_EPOCH};

/// A structure representing system time since January 1st 1970.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDTime {
    /// The number of seconds since January 1st 1970.
    pub secs: NSTDInt64,
    /// The remaining nanoseconds.
    pub nanos: NSTDUInt32,
}
impl From<SystemTime> for NSTDTime {
    /// Converts a [SystemTime] into an [NSTDTime] object.
    fn from(value: SystemTime) -> Self {
        match value.duration_since(UNIX_EPOCH) {
            Ok(dur) => NSTDTime {
                secs: dur.as_secs() as _,
                nanos: dur.subsec_nanos(),
            },
            Err(dur) => {
                let dur = dur.duration();
                NSTDTime {
                    secs: -(dur.as_secs() as NSTDInt64),
                    nanos: dur.subsec_nanos(),
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
    pub secs: NSTDInt64,
    /// The nanoseconds.
    pub nanos: NSTDUInt32,
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
