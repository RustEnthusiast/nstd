//! A reference counting smart pointer.
use crate::heap_ptr::NSTDHeapPtr;

/// A reference counting smart pointer.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NSTDSharedPtr {
    /// A heap pointer to private data about the shared object.
    pub ptr: NSTDHeapPtr,
}
