//! A pointer type for single value heap allocation.
use crate::core::ptr::NSTDPtr;

/// A pointer type for single value heap allocation.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NSTDHeapPtr {
    /// A pointer to the value on the heap.
    ptr: NSTDPtr,
}
