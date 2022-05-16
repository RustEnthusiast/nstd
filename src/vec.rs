//! A dynamically sized contiguous sequence of values.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_deallocate, nstd_alloc_reallocate},
    core::{
        def::{NSTDAny, NSTDAnyConst, NSTDByte, NSTDErrorCode, NSTDUSize},
        mem::{nstd_core_mem_copy, nstd_core_mem_copy_overlapping},
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

    /// Returns a pointer to one element past the end of the vector.
    ///
    /// # Safety
    ///
    /// This method does ***NOT*** check to make sure the vector is non-null.
    #[inline]
    pub(crate) unsafe fn end(&self) -> NSTDAny {
        self.buffer.ptr.raw.add(self.byte_len())
    }

    /// Attempts to reserve some memory for the vector if needed.
    #[inline]
    pub(crate) fn try_reserve(&mut self) -> NSTDErrorCode {
        if self.len == self.buffer.len {
            let new_cap = (self.buffer.len.max(1) as f32 * 1.5).ceil() as usize;
            return nstd_vec_reserve(self, new_cap);
        }
        0
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

/// Returns a pointer to the element at index `pos` in `vec`.
///
/// # Note
///
/// It is highly advised to copy the return value onto the stack because the pointer can easily
/// become invalid if the vector is mutated.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_CORE_NULL` if `pos` is out of
/// the vector's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_get(vec: &mut NSTDVec, pos: NSTDUSize) -> NSTDAny {
    nstd_vec_get_const(vec, pos) as NSTDAny
}

/// Returns an immutable pointer to the element at index `pos` in `vec`.
///
/// # Note
///
/// It is highly advised to copy the return value onto the stack because the pointer can easily
/// become invalid if the vector is mutated.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_CORE_NULL` if `pos` is out
/// of the vector's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_get_const(vec: &NSTDVec, pos: NSTDUSize) -> NSTDAnyConst {
    match pos < vec.len {
        true => unsafe { vec.buffer.ptr.raw.add(pos * vec.buffer.ptr.size) },
        false => NSTD_CORE_NULL,
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_push(vec: &mut NSTDVec, value: NSTDAnyConst) -> NSTDErrorCode {
    // Attempt to reserve space for the push.
    let errc = vec.try_reserve();
    // On success: copy bytes to the end of the vector.
    if errc == 0 {
        nstd_core_mem_copy(vec.end().cast(), value.cast(), vec.buffer.ptr.size);
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
        return unsafe { vec.end() };
    }
    NSTD_CORE_NULL
}

/// Attempts to insert a value into a vector at `index`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAnyConst value` - A pointer to the value to insert into the vector.
///
/// - `NSTDUSize index` - The index at which to insert the value.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - `index` is greater than the vector's length.
///
/// - `2` - Reserving space for the vector failed.
///
/// # Safety
///
/// This operation is unsafe because undefined behaviour can occur if the size of the value being
/// inserted into the vector is not equal to `vec.buffer.ptr.size`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_insert(
    vec: &mut NSTDVec,
    value: NSTDAnyConst,
    index: NSTDUSize,
) -> NSTDErrorCode {
    // Make sure `index` is valid.
    if index > vec.len {
        1
    }
    // Attempt to reserve space for the insert.
    else if vec.try_reserve() != 0 {
        2
    }
    // Insert the value.
    else {
        // Move elements at/after `index` over by one element.
        let bytes_to_copy = (vec.len - index) * vec.buffer.ptr.size;
        let idxpos = index * vec.buffer.ptr.size;
        let idxptr = vec.buffer.ptr.raw.add(idxpos).cast::<NSTDByte>();
        let dest = idxptr.add(vec.buffer.ptr.size);
        nstd_core_mem_copy_overlapping(dest, idxptr, bytes_to_copy);
        // Write `value` over the old value at `index`.
        nstd_core_mem_copy(idxptr, value.cast(), vec.buffer.ptr.size);
        vec.len += 1;
        0
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

/// Decreases a vector's capacity to match it's length.
///
/// # Note
///
/// This will return an error code of `0` if the vector is "null".
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_shrink(vec: &mut NSTDVec) -> NSTDErrorCode {
    let mut errc = 0;
    // Make sure the vector is non-null and it's capacity is greater than it's length.
    if !vec.buffer.ptr.raw.is_null() && vec.len < vec.buffer.len {
        let current_len = vec.buffer.byte_len();
        // Make sure to allocate at least one element to avoid undefined behavior.
        let new_len = vec.byte_len().max(vec.buffer.ptr.size);
        errc = unsafe { nstd_alloc_reallocate(&mut vec.buffer.ptr.raw, current_len, new_len) };
        if errc == 0 {
            // The buffer's new length is at least 1.
            vec.buffer.len = vec.len.max(1);
        }
    }
    errc
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
