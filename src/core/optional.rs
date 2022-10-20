//! Represents an optional (possibly uninitialized) value.

/// Represents an optional (possibly uninitialized) value.
#[repr(C)]
pub enum NSTDOptional<T> {
    /// The initialized variant.
    Some(T),
    /// The uninitialized variant.
    None,
}
