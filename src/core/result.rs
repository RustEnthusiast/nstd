//! Defines a "result" type with success and error variants.

/// Defines a "result" type with success and error variants.
#[repr(C)]
pub enum NSTDResult<T, E> {
    /// The success variant.
    Ok(T),
    /// The error variant.
    Err(E),
}
