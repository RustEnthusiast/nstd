//! Defines a "result" type with success and error variants.
use nstdapi::nstdapi;

/// Describes an `NSTDResult` variant.
#[nstdapi]
#[allow(non_camel_case_types)]
pub enum NSTDResultStatus {
    /// A successful variant.
    NSTD_RESULT_STATUS_OK,
    /// An error variant.
    NSTD_RESULT_STATUS_ERR,
}

/// Defines a "result" type with success and error variants.
#[nstdapi]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDResult<T, E> {
    /// The success variant.
    Ok(T),
    /// The error variant.
    Err(E),
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
}
