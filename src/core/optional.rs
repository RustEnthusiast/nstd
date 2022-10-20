//! Represents an optional (possibly uninitialized) value.

/// Describes an `NSTDOptional` variant.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDOptionalStatus {
    /// "Some" initialized value.
    NSTD_OPTIONAL_STATUS_SOME,
    /// No value.
    NSTD_OPTIONAL_STATUS_NONE,
}

/// Represents an optional (possibly uninitialized) value.
#[repr(C)]
pub enum NSTDOptional<T> {
    /// The initialized variant.
    Some(T),
    /// The uninitialized variant.
    None,
}
