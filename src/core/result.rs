//! Defines a "result" type with success and error variants.

/// Describes an `NSTDResult` variant.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDResultStatus {
    /// A successful variant.
    NSTD_RESULT_STATUS_OK,
    /// An error variant.
    NSTD_RESULT_STATUS_ERR,
}

/// Defines a "result" type with success and error variants.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDResult<T, E> {
    /// The success variant.
    Ok(T),
    /// The error variant.
    Err(E),
}
