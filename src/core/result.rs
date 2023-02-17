//! Defines a "result" type with success and error variants.
use nstdapi::nstdapi;

/// Describes an `NSTDResult` variant.
#[nstdapi]
#[allow(non_camel_case_types)]
pub enum NSTDResultStatus {
    /// An error variant.
    NSTD_RESULT_STATUS_ERR,
    /// A successful variant.
    NSTD_RESULT_STATUS_OK,
}

/// Defines a "result" type with success and error variants.
#[nstdapi]
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
            _ => panic!("called `NSTDResult::unwrap()` on an `Err` value"),
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
            _ => panic!("{msg}"),
        }
    }
}
