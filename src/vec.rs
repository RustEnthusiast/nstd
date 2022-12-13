//! A dynamically sized contiguous sequence of values.
use crate::{
    alloc::{
        nstd_alloc_allocate, nstd_alloc_deallocate, nstd_alloc_reallocate,
        NSTDAllocError::{self, NSTD_ALLOC_ERROR_NONE},
    },
    core::{
        def::{NSTDByte, NSTDErrorCode},
        mem::{nstd_core_mem_copy, nstd_core_mem_copy_overlapping},
        ptr::raw::{nstd_core_ptr_raw_dangling, nstd_core_ptr_raw_dangling_mut},
        slice::{
            nstd_core_slice_as_ptr, nstd_core_slice_len, nstd_core_slice_mut_new,
            nstd_core_slice_new, nstd_core_slice_stride, NSTDSlice, NSTDSliceMut,
        },
    },
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_NULL,
};
use core::ptr::{addr_of, NonNull};

/// A dynamically sized contiguous sequence of values.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDVec {
    /// A raw pointer to the vector's memory buffer.
    ptr: NSTDAnyMut,
    /// The number of bytes each value in the vector takes up.
    stride: NSTDUInt,
    /// The number of active elements in the vector.
    len: NSTDUInt,
    /// The number of values allocated in the memory buffer.
    cap: NSTDUInt,
}
impl NSTDVec {
    /// Creates a new [NSTDVec] from a Rust slice.
    ///
    /// # Panics
    ///
    /// This operation will panic if either `size_of::<T>()` is 0 or allocating fails.
    #[allow(dead_code)]
    pub(crate) fn from_slice<T: Copy>(slice: &[T]) -> Self {
        let stride = core::mem::size_of::<T>();
        let len = slice.len();
        if len > 0 {
            // Allocate the new vector.
            let mut vec = nstd_vec_new_with_cap(stride, len);
            assert!(!vec.ptr.is_null());
            // SAFETY: `vec`'s memory buffer has just been allocated and validated.
            unsafe { nstd_core_mem_copy(vec.ptr.cast(), slice.as_ptr().cast(), len * stride) };
            vec.len = len;
            vec
        } else {
            nstd_vec_new(stride)
        }
    }

    /// Returns the number of active bytes in the vector.
    #[inline]
    fn byte_len(&self) -> usize {
        self.len * self.stride
    }

    /// Returns the number of bytes in the vector's memory buffer.
    #[inline]
    fn buffer_byte_len(&self) -> usize {
        self.cap * self.stride
    }

    /// Creates a Rust byte slice containing all the *active* elements from this `NSTDVec`.
    ///
    /// # Panics
    ///
    /// This operation will panic in the following situations:
    ///
    /// - `size_of::<T>()` does not match the vector's stride.
    ///
    /// - The total length of the vector's buffer exceeds `isize::MAX` bytes.
    ///
    /// # Safety
    ///
    /// - The vector's data must remain valid while the returned slice is in use.
    ///
    /// - The vector's data must be properly aligned.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(self.stride == core::mem::size_of::<T>() && self.byte_len() <= isize::MAX as usize);
        match self.ptr.is_null() {
            false => core::slice::from_raw_parts(self.ptr.cast(), self.len),
            _ => core::slice::from_raw_parts(NonNull::dangling().as_ptr(), 0),
        }
    }

    /// Returns a pointer to one byte past the end of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the total length of the vector's buffer exceeds `isize::MAX` bytes.
    ///
    /// # Safety
    ///
    /// The vector must have already allocated memory.
    #[inline]
    unsafe fn end(&self) -> NSTDAnyMut {
        let len = self.byte_len();
        assert!(len <= isize::MAX as usize);
        self.ptr.add(len)
    }

    /// Attempts to reserve some memory for the vector if needed.
    #[inline]
    fn try_reserve(&mut self) -> NSTDAllocError {
        if self.len == self.cap {
            let additional = 1 + self.cap / 2;
            return nstd_vec_reserve(self, additional);
        }
        NSTD_ALLOC_ERROR_NONE
    }
}
impl Drop for NSTDVec {
    /// [NSTDVec]'s destructor.
    ///
    /// # Panics
    ///
    /// Panics if deallocating fails.
    #[inline]
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let buffer_len = self.buffer_byte_len();
            // SAFETY: The vector has allocated.
            unsafe {
                assert!(nstd_alloc_deallocate(&mut self.ptr, buffer_len) == NSTD_ALLOC_ERROR_NONE);
            }
        }
    }
}
impl<A> FromIterator<A> for NSTDVec {
    /// Creates a new [NSTDVec] from an iterator.
    ///
    /// # Note
    ///
    /// Each value will need to be dropped manually, as [NSTDVec] does not automatically drop it's
    /// contents.
    ///
    /// # Panics
    ///
    /// This operation will panic if `A`'s size in bytes is 0 or the iterator's length in bytes
    /// exceeds `NSTDInt`'s max value.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut s = nstd_vec_new(core::mem::size_of::<A>());
        for v in iter {
            // SAFETY: `v` is stored on the stack.
            unsafe { nstd_vec_push(&mut s, addr_of!(v).cast()) };
            // Be sure to forget `v` so it doesn't get dropped.
            core::mem::forget(v);
        }
        s
    }
}

/// Creates a new vector without allocating any resources.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size in bytes of each value in the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if `element_size` is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::vec::nstd_vec_new;
///
/// const SIZE: usize = core::mem::size_of::<u32>();
///
/// let vec = nstd_vec_new(SIZE);
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_new(element_size: NSTDUInt) -> NSTDVec {
    assert!(element_size != 0);
    NSTDVec {
        ptr: NSTD_NULL,
        stride: element_size,
        cap: 0,
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
/// - `NSTDUInt element_size` - The size in bytes of each value in the vector.
///
/// - `NSTDUInt cap` - The initial capacity for the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if either `element_size` or `cap` are zero.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_extend, nstd_vec_get, nstd_vec_len, nstd_vec_new_with_cap},
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
/// };
///
/// const SIZE: usize = core::mem::size_of::<i16>();
///
/// let numbers = [642i16, 324i16, 190i16];
/// let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3);
/// let mut vec = nstd_vec_new_with_cap(SIZE, 3);
/// unsafe {
///     assert!(nstd_vec_extend(&mut vec, &numbers) == NSTD_ALLOC_ERROR_NONE);
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<i16>();
///         let vv = nstd_vec_get(&vec, i).cast::<i16>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(*sv == *vv);
///     }
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_new_with_cap(element_size: NSTDUInt, mut cap: NSTDUInt) -> NSTDVec {
    // Ensure that neither `element_size` or `cap` are zero.
    assert!(element_size != 0 && cap != 0);
    // Attempt to allocate the memory buffer.
    // SAFETY: Both `element_size` & `cap` are above 0.
    let mem = unsafe { nstd_alloc_allocate(cap * element_size) };
    if mem.is_null() {
        cap = 0;
    }
    // Construct the vector.
    NSTDVec {
        ptr: mem,
        stride: element_size,
        cap,
        len: 0,
    }
}

/// Creates a new vector from a slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to copy data from.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector with a copy of `slice`'s contents.
///
/// # Panics
///
/// This operation will panic if the slice's stride is 0 or allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure that `slice`'s data is valid for reads.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_len},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// let numbers = [59237u128, 13953u128, 50285u128];
/// let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3);
/// unsafe {
///     let mut vec = nstd_vec_from_slice(&numbers);
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<u128>();
///         let vv = nstd_vec_get(&vec, i).cast::<u128>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(*sv == *vv);
///     }
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_from_slice(slice: &NSTDSlice) -> NSTDVec {
    let stride = nstd_core_slice_stride(slice);
    let len = nstd_core_slice_len(slice);
    if len > 0 {
        // Allocate the new vector.
        let mut vec = nstd_vec_new_with_cap(stride, len);
        assert!(!vec.ptr.is_null());
        let bytes = len * stride;
        nstd_core_mem_copy(vec.ptr.cast(), nstd_core_slice_as_ptr(slice).cast(), bytes);
        vec.len = len;
        vec
    } else {
        nstd_vec_new(stride)
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
    if vec.len > 0 {
        let mut cloned = nstd_vec_new_with_cap(vec.stride, vec.len);
        assert!(!cloned.ptr.is_null());
        // SAFETY: Both vectors are non-null.
        unsafe { nstd_core_mem_copy(cloned.ptr.cast(), vec.ptr.cast(), vec.byte_len()) };
        cloned.len = vec.len;
        cloned
    } else {
        nstd_vec_new(vec.stride)
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
/// `NSTDUInt len` - The length of the vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_len(vec: &NSTDVec) -> NSTDUInt {
    vec.len
}

/// Returns a vector's capacity.
///
/// This is the max number of values the vector can contain without reallocating.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUInt cap` - The vector's capacity.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_cap(vec: &NSTDVec) -> NSTDUInt {
    vec.cap
}

/// Returns the amount of bytes each value in a vector occupies.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUInt stride` - The size of each value in the vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_stride(vec: &NSTDVec) -> NSTDUInt {
    vec.stride
}

/// Returns an immutable slice containing all of a vector's active elements.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDSlice slice` - An *immutable* view into the vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_as_slice(vec: &NSTDVec) -> NSTDSlice {
    match vec.ptr.is_null() {
        false => nstd_core_slice_new(vec.ptr, vec.stride, vec.len),
        _ => nstd_core_slice_new(nstd_core_ptr_raw_dangling(), vec.stride, 0),
    }
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
    match vec.ptr.is_null() {
        false => nstd_core_slice_mut_new(vec.ptr, vec.stride, vec.len),
        _ => nstd_core_slice_mut_new(nstd_core_ptr_raw_dangling_mut(), vec.stride, 0),
    }
}

/// Returns a pointer to a vector's raw data.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the vector's raw data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_as_ptr(vec: &NSTDVec) -> NSTDAny {
    vec.ptr
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
    vec.ptr
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
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the vector's boundaries.
///
/// # Panics
///
/// Panics if the vec's current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_len},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i64>();
///
/// let numbers = [-639i64, 429i64, -440i64];
/// let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3);
/// unsafe {
///     let mut vec = nstd_vec_from_slice(&numbers);
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<i64>();
///         let vv = nstd_vec_get(&vec, i).cast::<i64>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(*sv == *vv);
///     }
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_get(vec: &NSTDVec, mut pos: NSTDUInt) -> NSTDAny {
    if pos < vec.len {
        pos *= vec.stride;
        assert!(pos <= isize::MAX as usize);
        // SAFETY: `pos` is a valid index.
        return unsafe { vec.ptr.add(pos) };
    }
    NSTD_NULL
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
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the vector's boundaries.
///
/// # Panics
///
/// Panics if the vec's current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_get_mut, nstd_vec_len},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i64>();
///
/// let numbers = [639i64, -429i64, 440i64];
/// let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3);
/// unsafe {
///     let mut vec = nstd_vec_from_slice(&numbers);
///     for i in 0..nstd_vec_len(&vec) {
///         let vv = nstd_vec_get_mut(&mut vec, i).cast::<i64>();
///         assert!(!vv.is_null());
///         *vv = -*vv;
///     }
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<i64>();
///         let vv = nstd_vec_get(&vec, i).cast::<i64>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(-*sv == *vv);
///     }
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_get_mut(vec: &mut NSTDVec, pos: NSTDUInt) -> NSTDAnyMut {
    nstd_vec_get(vec, pos) as NSTDAnyMut
}

/// Pushes a value onto a vector by copying bytes to the end of the vector's buffer. The number of
/// bytes to push is determined by `vec`'s stride.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAny value` - A pointer to the value to push onto the vector.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// Panics if `vec`'s current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation is unsafe because undefined behavior can occur if the size of the value being
/// pushed onto the vector is not equal to `vec`'s stride.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::vec::{nstd_vec_new, nstd_vec_push};
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// let mut vec = nstd_vec_new(SIZE);
/// let values: [f64; 3] = [6.0, 3.1, 9.4];
/// for value in values {
///     unsafe { nstd_vec_push(&mut vec, addr_of!(value).cast()) };
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_push(vec: &mut NSTDVec, value: NSTDAny) -> NSTDAllocError {
    // Attempt to reserve space for the push.
    let errc = vec.try_reserve();
    // On success: copy bytes to the end of the vector.
    if errc == NSTD_ALLOC_ERROR_NONE {
        nstd_core_mem_copy(vec.end().cast(), value.cast(), vec.stride);
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
/// - `NSTDAny value` - A pointer to the value that was popped off the stack, or null if the
/// vector is empty.
///
/// # Panics
///
/// Panics if `vec`'s new length (in bytes) exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::{
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_extend, nstd_vec_new, nstd_vec_pop},
/// };
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// let mut vec = nstd_vec_new(SIZE);
/// let values: [f64; 3] = [9.4, 3.1, 6.0];
/// let values_slice = nstd_core_slice_new(values.as_ptr().cast(), SIZE, 3);
/// unsafe {
///     nstd_vec_extend(&mut vec, &values_slice);
///     for value in values.iter().rev() {
///         assert!(*value == *nstd_vec_pop(&mut vec).cast::<f64>());
///     }
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_pop(vec: &mut NSTDVec) -> NSTDAny {
    if vec.len > 0 {
        vec.len -= 1;
        // SAFETY: The vector is non-null.
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
/// - `NSTDAny value` - A pointer to the value to insert into the vector.
///
/// - `NSTDUInt index` - The index at which to insert the value.
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
/// # Panics
///
/// This function will panic if `index` multiplied by `vec`'s stride exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation is unsafe because undefined behavior can occur if the size of the value being
/// inserted into the vector is not equal to `vec`'s stride.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::{
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_insert},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u32>();
///
/// let slice: [u32; 4] = [1, 2, 3, 5];
/// let slice = nstd_core_slice_new(slice.as_ptr().cast(), SIZE, 4);
/// unsafe {
///     let mut vec = nstd_vec_from_slice(&slice);
///     let four = 4u32;
///     assert!(nstd_vec_insert(&mut vec, addr_of!(four).cast(), 3) == 0);
///     for i in 1..=5 {
///         let v = nstd_vec_get(&vec, i - 1);
///         assert!(!v.is_null());
///         assert!(*v.cast::<u32>() == i as u32);
///     }
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_insert(
    vec: &mut NSTDVec,
    value: NSTDAny,
    mut index: NSTDUInt,
) -> NSTDErrorCode {
    // Make sure `index` is valid.
    if index > vec.len {
        1
    }
    // Attempt to reserve space for the insert.
    else if vec.try_reserve() != NSTD_ALLOC_ERROR_NONE {
        2
    }
    // Insert the value.
    else {
        // Move elements at/after `index` over by one element.
        let stride = vec.stride;
        let bytes_to_copy = (vec.len - index) * stride;
        index *= stride;
        assert!(index <= isize::MAX as usize);
        let idxptr = vec.ptr.add(index).cast::<NSTDByte>();
        let dest = idxptr.add(stride);
        nstd_core_mem_copy_overlapping(dest, idxptr, bytes_to_copy);
        // Write `value` over the old value at `index`.
        nstd_core_mem_copy(idxptr, value.cast(), stride);
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
/// - `NSTDUInt index` - The index of the element to remove.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if `index` is invalid.
///
/// # Panics
///
/// This operation will panic if `index` multiplied by `vec`'s stride exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_remove},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u32>();
///
/// let slice: [u32; 5] = [1, 2, 3, 4, 5];
/// let slice = nstd_core_slice_new(slice.as_ptr().cast(), SIZE, 5);
/// unsafe {
///     let mut vec = nstd_vec_from_slice(&slice);
///     assert!(nstd_vec_remove(&mut vec, 0) == 0);
///     assert!(nstd_vec_remove(&mut vec, 3) == 0);
///     for i in 0..3 {
///         let v = nstd_vec_get(&vec, i);
///         assert!(!v.is_null());
///         assert!(*v.cast::<u32>() == (i + 2) as u32);
///     }
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_remove(vec: &mut NSTDVec, mut index: NSTDUInt) -> NSTDErrorCode {
    // Make sure `index` is valid. This also ensures that `vec.len` is at least 1.
    if index < vec.len {
        // Move bytes after `index` to the left by one element.
        let stride = vec.stride;
        let bytes_to_copy = (vec.len - index - 1) * stride;
        index *= stride;
        assert!(index <= isize::MAX as usize);
        // SAFETY: The vector's data is valid for the shift.
        unsafe {
            let idxptr = vec.ptr.add(index).cast::<NSTDByte>();
            let src = idxptr.add(stride);
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
/// - `const NSTDSlice *values` - A slice of values to push onto the vector.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `vec` and `values` strides do not match.
///
/// - The current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation can cause undefined behavior if `values`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_extend, nstd_vec_get, nstd_vec_new},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// let values: [i128; 5] = [1, 2, 3, 4, 5];
/// let slice = nstd_core_slice_new(values.as_ptr().cast(), SIZE, 5);
/// unsafe {
///     let mut vec = nstd_vec_new(SIZE);
///     assert!(nstd_vec_extend(&mut vec, &slice) == NSTD_ALLOC_ERROR_NONE);
///     for i in 0..5 {
///         let v = nstd_vec_get(&vec, i);
///         assert!(!v.is_null());
///         assert!(*v.cast::<i128>() == values[i]);
///     }
/// }
/// ```
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_vec_extend(vec: &mut NSTDVec, values: &NSTDSlice) -> NSTDAllocError {
    // Ensure value sizes are the same for both the vector and the slice.
    assert!(vec.stride == nstd_core_slice_stride(values));
    let len = nstd_core_slice_len(values);
    // Making sure there's enough space for the extension.
    let mut errc = NSTD_ALLOC_ERROR_NONE;
    let reserved = vec.cap - vec.len;
    if reserved < len {
        let additional = len - reserved;
        errc = nstd_vec_reserve(vec, additional);
    }
    // On success copy bytes to the end of the vector.
    if errc == NSTD_ALLOC_ERROR_NONE {
        let ptr = nstd_core_slice_as_ptr(values).cast();
        nstd_core_mem_copy(vec.end().cast(), ptr, values.byte_len());
        vec.len += len;
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
/// - `NSTDUInt len` - The number of elements to keep.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_truncate(vec: &mut NSTDVec, len: NSTDUInt) {
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
/// - `NSTDUInt size` - The number of additional elements to allocate for.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_reserve(vec: &mut NSTDVec, size: NSTDUInt) -> NSTDAllocError {
    // Check `size`.
    if size == 0 {
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    // Calculate the number of bytes to allocate.
    let bytes_to_alloc = size * vec.stride;
    // Checking if the vector is null and needs to make it's first allocation.
    if vec.ptr.is_null() {
        // SAFETY: `bytes_to_alloc` is above 0.
        let mem = unsafe { nstd_alloc_allocate(bytes_to_alloc) };
        if !mem.is_null() {
            vec.ptr = mem;
            vec.cap = size;
            return NSTD_ALLOC_ERROR_NONE;
        }
        NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY
    }
    // Otherwise increase the vector's capacity.
    else {
        // This can't be 0 because the vector is non-null.
        // After an nstd vector has allocated it will always have at least one value allocated.
        // An example of this behavior can be seen in `nstd_vec_shrink`.
        let current_byte_len = vec.buffer_byte_len();
        let new_byte_len = current_byte_len + bytes_to_alloc;
        // SAFETY: The vector is non-null & the lengths are above 0.
        let errc = unsafe { nstd_alloc_reallocate(&mut vec.ptr, current_byte_len, new_byte_len) };
        // On success increase the buffer length.
        if errc == NSTD_ALLOC_ERROR_NONE {
            vec.cap += size;
        }
        errc
    }
}

/// Decreases a vector's capacity to match it's length.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_shrink(vec: &mut NSTDVec) -> NSTDAllocError {
    // Make sure the vector is non-null and it's capacity is greater than it's length.
    if !vec.ptr.is_null() && vec.len < vec.cap {
        let current_len = vec.buffer_byte_len();
        // Make sure to allocate at least one element to avoid undefined behavior.
        let new_len = vec.byte_len().max(vec.stride);
        // SAFETY: The vector is non-null & the lengths are above 0.
        let errc = unsafe { nstd_alloc_reallocate(&mut vec.ptr, current_len, new_len) };
        if errc == NSTD_ALLOC_ERROR_NONE {
            // The buffer's new length is at least 1.
            vec.cap = vec.len.max(1);
        }
        return errc;
    }
    NSTD_ALLOC_ERROR_NONE
}

/// Sets a vector's length to zero.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to clear.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_vec_clear(vec: &mut NSTDVec) {
    vec.len = 0;
}

/// Frees an instance of `NSTDVec`.
///
/// # Parameters:
///
/// - `NSTDVec vec` - The vector to free.
///
/// # Panics
///
/// Panics if deallocating fails.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_vec_free(vec: NSTDVec) {}
