//! Defines a "result" type with success and error variants.
use crate::NSTDUInt8;
use nstdapi::nstdapi;

/// Describes an erroneous `NSTDResult` value.
pub const NSTD_RESULT_ERR: NSTDUInt8 = 0;
/// Describes a successful `NSTDResult` value.
pub const NSTD_RESULT_OK: NSTDUInt8 = 1;

/// Defines a "result" type with success and error variants.
#[nstdapi]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDResult<T, E> {
    /// The error variant.
    Err(E),
    /// The success variant.
    Ok(T),
}
impl<T, E> NSTDResult<T, E> {
    /// Attempts to return the contained `Ok` value in an `NSTDResult`.
    ///
    /// This operation is only useful for testing code, it's use in production should be
    /// discouraged.
    ///
    /// # Panics
    ///
    /// Panics if `self` is an `Err` value.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(_) => panic!("called `NSTDResult::unwrap()` on an `Err` value"),
        }
    }

    /// Attempts to return the contained `Ok` value in an `NSTDResult`.
    ///
    /// # Panics
    ///
    /// Panics with `msg` if `self` is an `Err` value.
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(_) => panic!("{msg}"),
        }
    }
}
