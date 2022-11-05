//! Represents an optional (possibly uninitialized) value.

/// Describes an `NSTDOptional` variant.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDOptionalStatus {
    /// No value.
    NSTD_OPTIONAL_STATUS_NONE,
    /// "Some" initialized value.
    NSTD_OPTIONAL_STATUS_SOME,
}

/// Represents an optional (possibly uninitialized) value.
#[repr(C)]
pub enum NSTDOptional<T> {
    /// The uninitialized variant.
    None,
    /// The initialized variant.
    Some(T),
}
