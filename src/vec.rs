//! A dynamically sized contiguous sequence of values.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_deallocate, nstd_alloc_reallocate},
    core::{
        def::{NSTDAnyConst, NSTDErrorCode, NSTDUSize},
        mem::nstd_core_mem_copy,
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
impl NSTDVec {
    /// Returns the number of active bytes in the vector.
    #[inline]
    pub(crate) fn byte_len(&self) -> usize {
        self.len * self.buffer.ptr.size
    }
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

/// Pushes a value onto a vector by copying bytes to the end of the vector's buffer. The number of
/// bytes to push is determined by `vec.buffer.ptr.size`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAnyConst value` - A pointer to the value to push onto the vector.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// This operation is unsafe because undefined behaviour can occur if the size of the value being
/// pushed onto the vector is not equal to `vec.buffer.ptr.size`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_push(vec: &mut NSTDVec, value: NSTDAnyConst) -> NSTDErrorCode {
    let mut errc = 0;
    // Checking if the vector has reached it's capacity.
    if vec.len == vec.buffer.len {
        let new_cap = (vec.buffer.len.max(1) as f32 * 1.5).ceil() as usize;
        errc = nstd_vec_reserve(vec, new_cap);
    }
    // Copying bytes to the end of the vector.
    if errc == 0 {
        let vec_end = vec.buffer.ptr.raw.add(vec.byte_len());
        nstd_core_mem_copy(vec_end.cast(), value.cast(), vec.buffer.ptr.size);
        vec.len += 1;
    }
    errc
}

/// Removes the last value of a vector and returns a pointer to it.
///
/// # Note
///
/// It is highly advised to copy the return value onto the stack because the pointer can easily
/// become invalid if the vector is mutated.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// - `NSTDAnyConst value` - A pointer to the value that was popped off the stack, or null if the
/// vector is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_pop(vec: &mut NSTDVec) -> NSTDAnyConst {
    if vec.len > 0 {
        vec.len -= 1;
        return unsafe { vec.buffer.ptr.raw.add(vec.byte_len()) };
    }
    NSTD_CORE_NULL
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

/// 48878
#[test]
fn test() {
    use std::ptr::addr_of_mut;
    crate::test::run_test(|| unsafe {
        let mut vec = nstd_vec_new(4);
        for mut i in 0..100 {
            nstd_vec_push(&mut vec, addr_of_mut!(i).cast());
        }
        for _ in 0..100 {
            nstd_vec_pop(&mut vec);
        }
        nstd_vec_free(&mut vec);
    });
}
