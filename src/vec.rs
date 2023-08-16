//! A dynamically sized contiguous sequence of values.
extern crate alloc;
use crate::{
    alloc::{
        NSTDAllocError::{self, NSTD_ALLOC_ERROR_NONE},
        NSTDAllocator, GLOBAL_ALLOCATOR, NSTD_ALLOCATOR,
    },
    core::{
        def::{NSTDByte, NSTDErrorCode},
        mem::{nstd_core_mem_copy, nstd_core_mem_copy_overlapping},
        optional::NSTDOptional,
        ptr::raw::nstd_core_ptr_raw_dangling_mut,
        slice::{
            nstd_core_slice_as_ptr, nstd_core_slice_len, nstd_core_slice_mut_new_unchecked,
            nstd_core_slice_new_unchecked, nstd_core_slice_stride, NSTDSlice, NSTDSliceMut,
        },
    },
    NSTDAny, NSTDAnyMut, NSTDBool, NSTDUInt, NSTD_NULL,
};
use alloc::vec::Vec;
use core::ptr::addr_of;
use nstdapi::nstdapi;

/// A dynamically sized contiguous sequence of values.
#[nstdapi]
pub struct NSTDVec<'a> {
    /// The memory allocator.
    allocator: &'a NSTDAllocator,
    /// A raw pointer to the vector's memory buffer.
    ptr: NSTDAnyMut,
    /// The number of bytes each value in the vector takes up.
    stride: NSTDUInt,
    /// The number of active elements in the vector.
    len: NSTDUInt,
    /// The number of values allocated in the memory buffer.
    cap: NSTDUInt,
}
impl<'a> NSTDVec<'a> {
    /// Creates a new [`NSTDVec`] from a Rust [Vec].
    #[allow(dead_code)]
    pub(crate) fn from_vec<T>(vec: Vec<T>) -> NSTDVec<'a> {
        let cap = vec.capacity();
        let data = vec.leak();
        NSTDVec {
            allocator: &GLOBAL_ALLOCATOR,
            ptr: data.as_mut_ptr().cast(),
            stride: core::mem::size_of::<T>(),
            len: data.len(),
            cap,
        }
    }

    /// Checks if the vector's capacity is greater than 0.
    #[inline]
    const fn has_allocated(&self) -> NSTDBool {
        self.cap > 0
    }

    /// Returns the number of active bytes in the vector.
    #[inline]
    const fn byte_len(&self) -> usize {
        self.len * self.stride
    }

    /// Returns the number of bytes in the vector's memory buffer.
    #[inline]
    const fn buffer_byte_len(&self) -> usize {
        self.cap * self.stride
    }

    /// Creates a Rust slice containing all the *active* elements from this `NSTDVec`.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` does not match the vector's stride.
    ///
    /// # Safety
    ///
    /// - The vector's data must remain valid while the returned slice is in use.
    ///
    /// - The vector's data must be properly aligned.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(self.stride == core::mem::size_of::<T>());
        core::slice::from_raw_parts(self.ptr as _, self.len)
    }

    /// Returns a pointer to one byte past the end of the vector.
    #[inline]
    fn end(&mut self) -> NSTDAnyMut {
        // SAFETY: `self.ptr` is never null.
        unsafe { self.ptr.add(self.byte_len()) }
    }

    /// Attempts to reserve some memory for the vector if needed.
    #[inline]
    fn try_reserve(&mut self) -> NSTDAllocError {
        if self.len == self.cap {
            let additional = 1 + self.cap / 2;
            #[allow(unused_unsafe)]
            // SAFETY: This operation is safe.
            return unsafe { nstd_vec_reserve(self, additional) };
        }
        NSTD_ALLOC_ERROR_NONE
    }
}
impl Drop for NSTDVec<'_> {
    /// [`NSTDVec`]'s destructor.
    #[inline]
    fn drop(&mut self) {
        let buffer_len = self.buffer_byte_len();
        if buffer_len > 0 {
            // SAFETY: The vector has allocated.
            unsafe { (self.allocator.deallocate)(self.allocator.state, &mut self.ptr, buffer_len) };
        }
    }
}
impl<A> FromIterator<A> for NSTDVec<'_> {
    /// Creates a new [`NSTDVec`] from an iterator.
    ///
    /// # Note
    ///
    /// Each value will need to be dropped manually, as [`NSTDVec`] does not automatically drop it's
    /// contents.
    ///
    /// # Panics
    ///
    /// This operation will panic if allocating fails.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        #[allow(unused_unsafe)]
        // SAFETY: This operation is safe.
        let mut s = unsafe { nstd_vec_new(&NSTD_ALLOCATOR, core::mem::size_of::<A>()) };
        let mut errc;
        for v in iter {
            // SAFETY: `v` is stored on the stack.
            errc = unsafe { nstd_vec_push(&mut s, addr_of!(v).cast()) };
            assert!(errc == NSTD_ALLOC_ERROR_NONE);
            // Be sure to forget `v` so it doesn't get dropped.
            core::mem::forget(v);
        }
        s
    }
}
/// # Safety
///
/// The data that the vector holds must be able to be safely sent between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Send for NSTDVec<'_> {}
/// # Safety
///
/// The data that the vector holds must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDVec<'_> {}

/// Represents an optional value of type `NSTDVec`.
pub type NSTDOptionalVec<'a> = NSTDOptional<NSTDVec<'a>>;

/// Creates a new vector without allocating any resources.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt stride` - The size in bytes of each value in the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Example
///
/// ```
/// use nstd_sys::{alloc::NSTD_ALLOCATOR, vec::nstd_vec_new};
///
/// const SIZE: usize = core::mem::size_of::<u32>();
///
/// let vec = unsafe { nstd_vec_new(&NSTD_ALLOCATOR, SIZE) };
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_vec_new(allocator: &NSTDAllocator, stride: NSTDUInt) -> NSTDVec<'_> {
    NSTDVec {
        allocator,
        ptr: nstd_core_ptr_raw_dangling_mut(),
        stride,
        cap: 0,
        len: 0,
    }
}

/// Creates a new vector initialized with the given capacity.
///
/// If allocation fails, a vector with a capacity of 0 will be returned.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt stride` - The size in bytes of each value in the vector.
///
/// - `NSTDUInt cap` - The initial capacity for the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{NSTDAllocError::NSTD_ALLOC_ERROR_NONE, NSTD_ALLOCATOR},
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_extend, nstd_vec_get, nstd_vec_len, nstd_vec_new_with_cap},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i16>();
///
/// unsafe {
///     let numbers = [642i16, 324i16, 190i16];
///     let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3).unwrap();
///     let mut vec = nstd_vec_new_with_cap(&NSTD_ALLOCATOR, SIZE, 3);
///     assert!(nstd_vec_extend(&mut vec, &numbers) == NSTD_ALLOC_ERROR_NONE);
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<i16>();
///         let vv = nstd_vec_get(&vec, i).cast::<i16>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(*sv == *vv);
///     }
/// }
/// ```
#[nstdapi]
pub fn nstd_vec_new_with_cap(
    allocator: &NSTDAllocator,
    stride: NSTDUInt,
    mut cap: NSTDUInt,
) -> NSTDVec<'_> {
    // Check if either `stride` or `cap` are zero.
    if stride == 0 || cap == 0 {
        return NSTDVec {
            allocator,
            ptr: nstd_core_ptr_raw_dangling_mut(),
            stride,
            cap,
            len: 0,
        };
    }
    // Attempt to allocate the memory buffer.
    // SAFETY: Both `stride` & `cap` are above 0.
    let mut ptr = unsafe { (allocator.allocate)(allocator.state, cap * stride) };
    if ptr.is_null() {
        ptr = nstd_core_ptr_raw_dangling_mut();
        cap = 0;
    }
    // Construct the vector.
    NSTDVec {
        allocator,
        ptr,
        stride,
        cap,
        len: 0,
    }
}

/// Creates a new vector from a slice.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `const NSTDSlice *slice` - The slice to copy data from.
///
/// # Returns
///
/// `NSTDOptionalVec vec` - The new vector with a copy of `slice`'s contents on success, or an
/// uninitialized "none" variant if allocating fails.
///
/// # Safety
///
/// The caller of this function must ensure that `slice`'s data is valid for reads.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_len},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// unsafe {
///     let numbers = [59237u128, 13953u128, 50285u128];
///     let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3).unwrap();
///     let mut vec = nstd_vec_from_slice(&NSTD_ALLOCATOR, &numbers).unwrap();
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<u128>();
///         let vv = nstd_vec_get(&vec, i).cast::<u128>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(*sv == *vv);
///     }
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_vec_from_slice<'a>(
    allocator: &'a NSTDAllocator,
    slice: &NSTDSlice,
) -> NSTDOptionalVec<'a> {
    let stride = nstd_core_slice_stride(slice);
    let len = nstd_core_slice_len(slice);
    if len > 0 {
        // Allocate the new vector.
        let mut vec = nstd_vec_new_with_cap(allocator, stride, len);
        if !vec.has_allocated() {
            return NSTDOptional::None;
        }
        let bytes = len * stride;
        nstd_core_mem_copy(vec.ptr.cast(), nstd_core_slice_as_ptr(slice).cast(), bytes);
        vec.len = len;
        NSTDOptional::Some(vec)
    } else {
        NSTDOptional::Some(nstd_vec_new(allocator, stride))
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
/// `NSTDOptionalVec cloned` - The new deep copy of `vec` on success, or an uninitialized "none"
/// variant if allocating fails.
#[nstdapi]
pub fn nstd_vec_clone<'a>(vec: &NSTDVec<'a>) -> NSTDOptionalVec<'a> {
    if vec.len > 0 {
        let mut cloned = nstd_vec_new_with_cap(vec.allocator, vec.stride, vec.len);
        if !cloned.has_allocated() {
            return NSTDOptional::None;
        }
        // SAFETY: Both vectors are non-null.
        unsafe { nstd_core_mem_copy(cloned.ptr.cast(), vec.ptr.cast(), vec.byte_len()) };
        cloned.len = vec.len;
        NSTDOptional::Some(cloned)
    } else {
        NSTDOptional::Some(nstd_vec_new(vec.allocator, vec.stride))
    }
}

/// Returns an immutable reference to a vector's allocator.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `const NSTDAllocator *allocator` - The vector's allocator.
#[inline]
#[nstdapi]
pub const fn nstd_vec_allocator<'a>(vec: &NSTDVec<'a>) -> &'a NSTDAllocator {
    vec.allocator
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
#[nstdapi]
pub const fn nstd_vec_len(vec: &NSTDVec<'_>) -> NSTDUInt {
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
#[nstdapi]
pub const fn nstd_vec_cap(vec: &NSTDVec<'_>) -> NSTDUInt {
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
#[nstdapi]
pub const fn nstd_vec_stride(vec: &NSTDVec<'_>) -> NSTDUInt {
    vec.stride
}

/// Returns the number of reserved elements within a vector's inactive buffer.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUInt reserved` - The number of uninitialized elements within `vec`'s inactive buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     vec::{nstd_vec_new_with_cap, nstd_vec_reserved},
/// };
///
/// unsafe {
///     let vec = nstd_vec_new_with_cap(&NSTD_ALLOCATOR, 2, 16);
///     assert!(nstd_vec_reserved(&vec) == 16);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_vec_reserved(vec: &NSTDVec<'_>) -> NSTDUInt {
    vec.cap - vec.len
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
#[nstdapi]
pub const fn nstd_vec_as_slice(vec: &NSTDVec<'_>) -> NSTDSlice {
    // SAFETY: `vec.ptr` is checked, vector lengths are never greater than `NSTDInt`'s max value.
    unsafe { nstd_core_slice_new_unchecked(vec.ptr, vec.stride, vec.len) }
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
#[nstdapi]
pub fn nstd_vec_as_slice_mut(vec: &mut NSTDVec<'_>) -> NSTDSliceMut {
    // SAFETY: `vec.ptr` is checked, vector lengths are never greater than `NSTDInt`'s max value.
    unsafe { nstd_core_slice_mut_new_unchecked(vec.ptr, vec.stride, vec.len) }
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
#[nstdapi]
pub const fn nstd_vec_as_ptr(vec: &NSTDVec<'_>) -> NSTDAny {
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
#[nstdapi]
pub fn nstd_vec_as_ptr_mut(vec: &mut NSTDVec<'_>) -> NSTDAnyMut {
    vec.ptr
}

/// Returns a pointer to the end of a vector.
///
/// Note that this does not return a pointer to the last element or the last byte in the vector, but
/// a pointer to *one byte past* the end of the vector's active buffer.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAny end` - A pointer to the end of the vector.
#[inline]
#[nstdapi]
pub const fn nstd_vec_end(vec: &NSTDVec<'_>) -> NSTDAny {
    // SAFETY: `len` is within the bounds of the vector and does not overflow `isize`.
    unsafe { vec.ptr.add(vec.byte_len()) }
}

/// Returns a mutable pointer to the end of a vector.
///
/// Note that this does not return a pointer to the last element or the last byte in the vector, but
/// a pointer to *one byte past* the end of the vector's active buffer.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAnyMut end` - A mutable pointer to the end of the vector.
#[inline]
#[nstdapi]
pub fn nstd_vec_end_mut(vec: &mut NSTDVec<'_>) -> NSTDAnyMut {
    // SAFETY: `len` is within the bounds of the vector and does not overflow `isize`.
    unsafe { vec.ptr.add(vec.byte_len()) }
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
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_len},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i64>();
///
/// unsafe {
///     let numbers = [-639i64, 429i64, -440i64];
///     let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3).unwrap();
///     let mut vec = nstd_vec_from_slice(&NSTD_ALLOCATOR, &numbers).unwrap();
///     for i in 0..nstd_vec_len(&vec) {
///         let sv = nstd_core_slice_get(&numbers, i).cast::<i64>();
///         let vv = nstd_vec_get(&vec, i).cast::<i64>();
///         assert!(!sv.is_null() && !vv.is_null());
///         assert!(*sv == *vv);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_vec_get(vec: &NSTDVec<'_>, mut pos: NSTDUInt) -> NSTDAny {
    if pos < vec.len {
        pos *= vec.stride;
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
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::slice::{nstd_core_slice_get, nstd_core_slice_new},
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_get_mut, nstd_vec_len},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i64>();
///
/// unsafe {
///     let numbers = [639i64, -429i64, 440i64];
///     let numbers = nstd_core_slice_new(numbers.as_ptr().cast(), SIZE, 3).unwrap();
///     let mut vec = nstd_vec_from_slice(&NSTD_ALLOCATOR, &numbers).unwrap();
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
#[nstdapi]
pub fn nstd_vec_get_mut(vec: &mut NSTDVec<'_>, pos: NSTDUInt) -> NSTDAnyMut {
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
/// # Safety
///
/// This operation is unsafe because undefined behavior can occur if the size of the value being
/// pushed onto the vector is not equal to `vec`'s stride.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     vec::{nstd_vec_new, nstd_vec_push},
/// };
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// unsafe {
///     let mut vec = nstd_vec_new(&NSTD_ALLOCATOR, SIZE);
///     let values: [f64; 3] = [6.0, 3.1, 9.4];
///     for value in values {
///         nstd_vec_push(&mut vec, addr_of!(value).cast());
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_vec_push(vec: &mut NSTDVec<'_>, value: NSTDAny) -> NSTDAllocError {
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
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_extend, nstd_vec_new, nstd_vec_pop},
/// };
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// unsafe {
///     let mut vec = nstd_vec_new(&NSTD_ALLOCATOR, SIZE);
///     let values: [f64; 3] = [9.4, 3.1, 6.0];
///     let values_slice = nstd_core_slice_new(values.as_ptr().cast(), SIZE, 3).unwrap();
///     nstd_vec_extend(&mut vec, &values_slice);
///     for value in values.iter().rev() {
///         assert!(*value == *nstd_vec_pop(&mut vec).cast::<f64>());
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_vec_pop(vec: &mut NSTDVec<'_>) -> NSTDAny {
    if vec.len > 0 {
        vec.len -= 1;
        return vec.end();
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
/// # Errors
///
/// - `1` - `index` is greater than the vector's length.
///
/// - `2` - Reserving space for the vector failed.
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
///     alloc::NSTD_ALLOCATOR,
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_insert},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u32>();
///
/// unsafe {
///     let slice: [u32; 4] = [1, 2, 3, 5];
///     let slice = nstd_core_slice_new(slice.as_ptr().cast(), SIZE, 4).unwrap();
///     let mut vec = nstd_vec_from_slice(&NSTD_ALLOCATOR, &slice).unwrap();
///     let four = 4u32;
///     assert!(nstd_vec_insert(&mut vec, addr_of!(four).cast(), 3) == 0);
///     for i in 1..=5 {
///         let v = nstd_vec_get(&vec, i - 1);
///         assert!(!v.is_null());
///         assert!(*v.cast::<u32>() == i as u32);
///     }
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_vec_insert(
    vec: &mut NSTDVec<'_>,
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
        if vec.stride > 0 {
            let stride = vec.stride;
            let bytes_to_copy = (vec.len - index) * stride;
            index *= stride;
            let idxptr = vec.ptr.add(index).cast::<NSTDByte>();
            let dest = idxptr.add(stride);
            nstd_core_mem_copy_overlapping(dest, idxptr, bytes_to_copy);
            // Write `value` over the old value at `index`.
            nstd_core_mem_copy(idxptr, value.cast(), stride);
        }
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
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_from_slice, nstd_vec_get, nstd_vec_remove},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u32>();
///
/// unsafe {
///     let slice: [u32; 5] = [1, 2, 3, 4, 5];
///     let slice = nstd_core_slice_new(slice.as_ptr().cast(), SIZE, 5).unwrap();
///     let mut vec = nstd_vec_from_slice(&NSTD_ALLOCATOR, &slice).unwrap();
///     assert!(nstd_vec_remove(&mut vec, 0) == 0);
///     assert!(nstd_vec_remove(&mut vec, 3) == 0);
///     for i in 0..3 {
///         let v = nstd_vec_get(&vec, i);
///         assert!(!v.is_null());
///         assert!(*v.cast::<u32>() == (i + 2) as u32);
///     }
/// }
/// ```
#[nstdapi]
pub fn nstd_vec_remove(vec: &mut NSTDVec<'_>, mut index: NSTDUInt) -> NSTDErrorCode {
    // Make sure `index` is valid. This also ensures that `vec.len` is at least 1.
    if index < vec.len {
        // Move bytes after `index` to the left by one element.
        if vec.stride > 0 {
            let stride = vec.stride;
            let bytes_to_copy = (vec.len - index - 1) * stride;
            index *= stride;
            // SAFETY: The vector's data is valid for the shift.
            unsafe {
                let idxptr = vec.ptr.add(index).cast::<NSTDByte>();
                let src = idxptr.add(stride);
                nstd_core_mem_copy_overlapping(idxptr, src, bytes_to_copy);
            }
        }
        // Decrement the vector's length AFTER shifting the bytes.
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
/// This operation will panic if `vec` and `values` strides do not match.
///
/// # Safety
///
/// This operation can cause undefined behavior if `values`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{NSTDAllocError::NSTD_ALLOC_ERROR_NONE, NSTD_ALLOCATOR},
///     core::slice::nstd_core_slice_new,
///     vec::{nstd_vec_extend, nstd_vec_get, nstd_vec_new},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let values: [i128; 5] = [1, 2, 3, 4, 5];
///     let slice = nstd_core_slice_new(values.as_ptr().cast(), SIZE, 5).unwrap();
///     let mut vec = nstd_vec_new(&NSTD_ALLOCATOR, SIZE);
///     assert!(nstd_vec_extend(&mut vec, &slice) == NSTD_ALLOC_ERROR_NONE);
///     for i in 0..5 {
///         let v = nstd_vec_get(&vec, i);
///         assert!(!v.is_null());
///         assert!(*v.cast::<i128>() == values[i]);
///     }
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_vec_extend(vec: &mut NSTDVec<'_>, values: &NSTDSlice) -> NSTDAllocError {
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
#[nstdapi]
pub fn nstd_vec_truncate(vec: &mut NSTDVec<'_>, len: NSTDUInt) {
    if vec.len > len {
        vec.len = len;
    }
}

/// Sets a vectors length.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDUInt len` - The new length for the vector.
///
/// # Safety
///
/// - If `len` is greater than the vector's current length, care must be taken to ensure that the
/// new elements are properly initialized.
///
/// - `len`'s value must not be greater than the vector's capacity.
#[inline]
#[nstdapi]
pub unsafe fn nstd_vec_set_len(vec: &mut NSTDVec<'_>, len: NSTDUInt) {
    vec.len = len;
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
#[nstdapi]
pub fn nstd_vec_reserve(vec: &mut NSTDVec<'_>, size: NSTDUInt) -> NSTDAllocError {
    // Calculate the number of bytes to allocate.
    let bytes_to_alloc = size * vec.stride;
    if bytes_to_alloc == 0 {
        vec.cap += size;
        return NSTD_ALLOC_ERROR_NONE;
    }
    // Check if the vector has allocated.
    if vec.has_allocated() {
        // This can't be 0 because the vector is non-null.
        // After an nstd vector has allocated it will always have at least one value allocated.
        // An example of this behavior can be seen in `nstd_vec_shrink`.
        let byte_len = vec.buffer_byte_len();
        let new_byte_len = byte_len + bytes_to_alloc;
        // SAFETY: The vector is non-null & the lengths are above 0.
        let errc = unsafe {
            (vec.allocator.reallocate)(vec.allocator.state, &mut vec.ptr, byte_len, new_byte_len)
        };
        // On success increase the buffer length.
        if errc == NSTD_ALLOC_ERROR_NONE {
            vec.cap += size;
        }
        errc
    } else {
        // SAFETY: `bytes_to_alloc` is above 0.
        let mem = unsafe { (vec.allocator.allocate)(vec.allocator.state, bytes_to_alloc) };
        if !mem.is_null() {
            vec.ptr = mem;
            vec.cap = size;
            return NSTD_ALLOC_ERROR_NONE;
        }
        NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY
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
#[nstdapi]
pub fn nstd_vec_shrink(vec: &mut NSTDVec<'_>) -> NSTDAllocError {
    // Make sure the vector's capacity is greater than it's length.
    if vec.cap > vec.len {
        // Make sure the vector's stride is greater than 0 before reallocating.
        if vec.stride > 0 {
            let current_len = vec.buffer_byte_len();
            // Make sure to allocate at least one element to avoid undefined behavior.
            let new_len = vec.byte_len().max(vec.stride);
            // SAFETY: The vector is non-null & the lengths are above 0.
            let errc = unsafe {
                (vec.allocator.reallocate)(vec.allocator.state, &mut vec.ptr, current_len, new_len)
            };
            if errc == NSTD_ALLOC_ERROR_NONE {
                // The buffer's new length is at least 1.
                vec.cap = vec.len.max(1);
            }
            return errc;
        }
        vec.cap = vec.len.max(1);
    }
    NSTD_ALLOC_ERROR_NONE
}

/// Sets a vector's length to zero.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to clear.
#[inline]
#[nstdapi]
pub fn nstd_vec_clear(vec: &mut NSTDVec<'_>) {
    vec.len = 0;
}

/// Frees an instance of `NSTDVec`.
///
/// # Parameters:
///
/// - `NSTDVec vec` - The vector to free.
#[inline]
#[nstdapi]
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
pub fn nstd_vec_free(vec: NSTDVec<'_>) {}

/// Frees an instance of `NSTDVec` after invoking `callback` with each of the vector's elements.
///
/// # Parameters:
///
/// - `NSTDVec vec` - The vector to free.
///
/// - `void (*callback)(NSTDAnyMut)` - The vector data's destructor.
///
/// # Safety
///
/// This operation makes a direct call on a C function pointer (`callback`).
#[nstdapi]
pub unsafe fn nstd_vec_drop(mut vec: NSTDVec<'_>, callback: unsafe extern "C" fn(NSTDAnyMut)) {
    let mut ptr = nstd_vec_as_ptr_mut(&mut vec);
    let end = nstd_vec_end(&vec);
    while ptr < end as _ {
        callback(ptr);
        ptr = ptr.add(vec.stride);
    }
}
