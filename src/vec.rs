//! A dynamically sized contiguous sequence of values.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_deallocate, nstd_alloc_reallocate},
    core::{
        def::{NSTDErrorCode, NSTDUSize},
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

/// Reserves some space on the heap for at least `size` more elements to be pushed onto a vector
/// without making more allocations.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to reserve space for.
///
/// - `NSTDUSize size` - The number of additional elements to allocate for.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Panics
///
/// This operation will panic if `size` is zero.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_reserve(vec: &mut NSTDVec, size: NSTDUSize) -> NSTDErrorCode {
    assert!(size != 0);
    // Calculate the number of bytes to allocate.
    let bytes_to_alloc = size * vec.buffer.ptr.size;
    // Checking if the vector is null and needs to make it's first allocation.
    if vec.buffer.ptr.raw.is_null() {
        let mem = unsafe { nstd_alloc_allocate(bytes_to_alloc) };
        if !mem.is_null() {
            vec.buffer.ptr.raw = mem;
            vec.buffer.len = size;
            return 0;
        }
        1
    }
    // Otherwise increase the vector's capacity.
    else {
        let current_byte_len = vec.buffer.byte_len();
        let new_byte_len = current_byte_len + bytes_to_alloc;
        let errc = unsafe {
            nstd_alloc_reallocate(&mut vec.buffer.ptr.raw, current_byte_len, new_byte_len)
        };
        // On success increase the buffer length.
        if errc == 0 {
            vec.buffer.len += size;
        }
        errc
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
