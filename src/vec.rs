//! A dynamically sized contiguous sequence of values.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_deallocate, nstd_alloc_reallocate},
    core::{
        def::{NSTDByte, NSTDErrorCode},
        mem::{nstd_core_mem_copy, nstd_core_mem_copy_overlapping},
        slice::{nstd_core_slice_const_new, nstd_core_slice_mut_new, NSTDSliceConst, NSTDSliceMut},
    },
    NSTDAnyConst, NSTDAnyMut, NSTDUSize, NSTD_NULL,
};

/// A dynamically sized contiguous sequence of values.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDVec {
    /// The underlying memory buffer.
    buffer: NSTDSliceMut,
    /// The number of active elements in the vector.
    len: NSTDUSize,
}
impl NSTDVec {
    /// Returns the number of active bytes in the vector.
    #[inline]
    pub(crate) fn byte_len(&self) -> usize {
        self.len * self.buffer.ptr.size
    }

    /// Creates a Rust byte slice containing all the *active* elements from this `NSTDVec`.
    #[inline]
    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer.ptr.raw.cast(), self.byte_len()) }
    }

    /// Returns a pointer to one element past the end of the vector.
    ///
    /// # Safety
    ///
    /// This method does ***NOT*** check to make sure the vector is non-null.
    #[inline]
    pub(crate) unsafe fn end(&self) -> NSTDAnyMut {
        self.buffer.ptr.raw.add(self.byte_len())
    }

    /// Attempts to reserve some memory for the vector if needed.
    #[inline]
    pub(crate) fn try_reserve(&mut self) -> NSTDErrorCode {
        if self.len == self.buffer.len {
            let additional = 1 + self.buffer.len / 2;
            return nstd_vec_reserve(self, additional);
        }
        0
    }
}
impl<T> From<&[T]> for NSTDVec {
    /// Creates a new vector from a Rust slice.
    ///
    /// # Note
    ///
    /// It is possible for this function to return a null vector on error.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` is 0.
    fn from(slice: &[T]) -> Self {
        // Check the size of `T`.
        let element_size = core::mem::size_of::<T>();
        assert!(element_size != 0);
        // Check the length of the slice.
        let len = slice.len();
        if len > 0 {
            // Create the new vector.
            let mut vec = nstd_vec_new_with_cap(element_size, len);
            if !vec.buffer.ptr.raw.is_null() {
                // On success, copy the bytes from the slice to the vector's memory buffer and
                // set it's length.
                unsafe {
                    nstd_core_mem_copy(
                        vec.buffer.ptr.raw.cast(),
                        slice.as_ptr().cast(),
                        len * element_size,
                    );
                }
                vec.len = len;
            }
            vec
        } else {
            nstd_vec_new(element_size)
        }
    }
}
impl Drop for NSTDVec {
    /// [NSTDVec]'s destructor.
    #[inline]
    fn drop(&mut self) {
        if !self.buffer.ptr.raw.is_null() {
            let buffer_len = self.buffer.byte_len();
            unsafe { nstd_alloc_deallocate(&mut self.buffer.ptr.raw, buffer_len) };
        }
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
        buffer: nstd_core_slice_mut_new(NSTD_NULL, element_size, 0),
        len: 0,
    }
}

/// Creates a new vector initialized with the given capacity.
///
/// # Note
///
/// This will return a "null vector" (a vector that has not allocated yet) on error.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size in bytes of each value in the vector.
///
/// - `NSTDUSize cap` - The initial capacity for the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if either `element_size` or `cap` are zero.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_new_with_cap(element_size: NSTDUSize, mut cap: NSTDUSize) -> NSTDVec {
    // Ensure that neither `element_size` or `cap` are zero.
    assert!(element_size != 0);
    assert!(cap != 0);
    // Attempt to allocate the memory buffer.
    let mem = unsafe { nstd_alloc_allocate(cap * element_size) };
    if mem.is_null() {
        cap = 0;
    }
    // Construct the vector.
    NSTDVec {
        buffer: nstd_core_slice_mut_new(mem, element_size, cap),
        len: 0,
    }
}

/// Creates a new deep copy of `vec`.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector to create a new deep copy of.
///
/// # Returns
///
/// `NSTDVec cloned` - The new deep copy of `vec`.
///
/// # Panics
///
/// This operation will panic if allocating for the new vector fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_clone(vec: &NSTDVec) -> NSTDVec {
    if !vec.buffer.ptr.raw.is_null() {
        let mut cloned = nstd_vec_new_with_cap(vec.buffer.ptr.size, vec.buffer.len);
        assert!(!cloned.buffer.ptr.raw.is_null());
        unsafe {
            nstd_core_mem_copy(
                cloned.buffer.ptr.raw.cast(),
                vec.buffer.ptr.raw.cast(),
                vec.byte_len(),
            );
        }
        cloned.len = vec.len;
        cloned
    } else {
        nstd_vec_new(vec.buffer.ptr.size)
    }
}

/// Returns the length of a vector.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_len(vec: &NSTDVec) -> NSTDUSize {
    vec.len
}

/// Returns an immutable slice containing all of a vector's active elements.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDSliceConst slice` - An *immutable* view into the vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_as_slice(vec: &NSTDVec) -> NSTDSliceConst {
    nstd_core_slice_const_new(vec.buffer.ptr.raw, vec.buffer.ptr.size, vec.len)
}

/// Returns a slice containing all of a vector's active elements.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDSliceMut slice` - A *mutable* view into the vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_as_slice_mut(vec: &mut NSTDVec) -> NSTDSliceMut {
    nstd_core_slice_mut_new(vec.buffer.ptr.raw, vec.buffer.ptr.size, vec.len)
}

/// Returns a pointer to a vector's raw data.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAnyConst ptr` - A pointer to the vector's raw data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_as_ptr(vec: &NSTDVec) -> NSTDAnyConst {
    vec.buffer.ptr.raw
}

/// Returns a pointer to a vector's raw data.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the vector's raw data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_as_mut_ptr(vec: &mut NSTDVec) -> NSTDAnyMut {
    vec.buffer.ptr.raw
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
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the vector's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_get(vec: &NSTDVec, pos: NSTDUSize) -> NSTDAnyConst {
    match pos < vec.len {
        true => unsafe { vec.buffer.ptr.raw.add(pos * vec.buffer.ptr.size) },
        false => NSTD_NULL,
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
/// `NSTDAnyMut element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the vector's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_get_mut(vec: &mut NSTDVec, pos: NSTDUSize) -> NSTDAnyMut {
    nstd_vec_get(vec, pos) as NSTDAnyMut
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
    NSTD_NULL
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

/// Removes the element at `index` in a vector.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDUSize index` - The index of the element to remove.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if `index` is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_remove(vec: &mut NSTDVec, index: NSTDUSize) -> NSTDErrorCode {
    // Make sure `index` is valid. This also ensures that `vec.len` is at least 1.
    if index < vec.len {
        // Move bytes after `index` to the left by one element.
        let bytes_to_copy = (vec.len - index - 1) * vec.buffer.ptr.size;
        let idxpos = index * vec.buffer.ptr.size;
        unsafe {
            let idxptr = vec.buffer.ptr.raw.add(idxpos).cast::<NSTDByte>();
            let src = idxptr.add(vec.buffer.ptr.size);
            nstd_core_mem_copy_overlapping(idxptr, src, bytes_to_copy);
        }
        // Decrement the vector's length AFTER shifting the bytes.
        // This is done here because another thread may attempt to shrink the vector. This would
        // cause undefined behavior if the vectors length is decremented before shifting the bytes.
        vec.len -= 1;
        0
    } else {
        1
    }
}

/// Pushes a series of values onto a vector.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to extend.
///
/// - `const NSTDSliceConst *values` - A slice of values to push onto the vector.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if reserving memory for the extension fails.
///
/// # Panics
///
/// This operation will panic if the element sizes for `vec` and `values` do not match.
///
/// # Safety
///
/// This operation can cause undefined behavior if `values`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_extend(
    vec: &mut NSTDVec,
    values: &NSTDSliceConst,
) -> NSTDErrorCode {
    // Ensure value sizes are the same for both the vector and the slice.
    assert!(vec.buffer.ptr.size == values.ptr.size);
    // Making sure there's enough space for the extension.
    let mut errc = 0;
    let reserved = vec.buffer.len - vec.len;
    if reserved < values.len {
        let additional = values.len - reserved;
        errc = nstd_vec_reserve(vec, additional);
    }
    // On success copy bytes to the end of the vector.
    if errc == 0 {
        nstd_core_mem_copy(vec.end().cast(), values.ptr.raw.cast(), values.byte_len());
        vec.len += values.len;
    }
    errc
}

/// Shortens a vector, keeping the first `len` elements.
///
/// # Note
///
/// This function does nothing if `vec.len` is less than or equal to `len`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to truncate.
///
/// - `NSTDUSize len` - The number of elements to keep.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_truncate(vec: &mut NSTDVec, len: NSTDUSize) {
    if vec.len > len {
        vec.len = len;
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
/// - `NSTDVec vec` - The vector to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_vec_free(vec: NSTDVec) {}
