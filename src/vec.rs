//! A dynamically sized contiguous sequence of values.
use crate::{
    alloc::nstd_alloc_deallocate,
    core::{
        def::NSTDUSize,
        slice::{nstd_core_slice_new, NSTDSlice},
        NSTD_CORE_NULL,
    },
};

/// A dynamically sized contiguous sequence of values.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDVec {
    /// The underlying memory buffer.
    pub buffer: NSTDSlice,
    /// The number of active elements in the vector.
    pub len: NSTDUSize,
}

/// Creates a new vector without allocating any resources.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size in bytes of each value in the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if `element_size` is zero.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_new(element_size: NSTDUSize) -> NSTDVec {
    assert!(element_size != 0);
    NSTDVec {
        buffer: nstd_core_slice_new(NSTD_CORE_NULL, element_size, 0),
        len: 0,
    }
}

/// Frees an instance of `NSTDVec`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_free(vec: &mut NSTDVec) {
    if !vec.buffer.ptr.raw.is_null() {
        let buffer_len = vec.buffer.byte_len();
        unsafe { nstd_alloc_deallocate(&mut vec.buffer.ptr.raw, buffer_len) };
        vec.buffer.len = 0;
        vec.len = 0;
    }
}
