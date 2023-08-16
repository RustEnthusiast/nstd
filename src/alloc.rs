//! Low level memory allocation.
extern crate alloc;
#[cfg(windows)]
use crate::os::windows::alloc::{
    nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_allocate_zeroed,
    nstd_os_windows_alloc_deallocate, nstd_os_windows_alloc_reallocate,
    NSTDWindowsAllocError::{
        self, NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND, NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP,
        NSTD_WINDOWS_ALLOC_ERROR_INVALID_LAYOUT, NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND,
        NSTD_WINDOWS_ALLOC_ERROR_NONE, NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY,
    },
};
use crate::{
    core::{
        mem::nstd_core_mem_copy,
        ptr::raw::{nstd_core_ptr_raw_dangling_mut, MAX_ALIGN},
    },
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_NULL,
};
use alloc::alloc::Layout;
use cfg_if::cfg_if;
use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::addr_of,
};
use nstdapi::nstdapi;

/// An FFI safe [Box] variant for `nstd`.
#[repr(transparent)]
#[allow(dead_code)]
pub(crate) struct CBox<T>(NSTDAnyMut, PhantomData<T>);
#[allow(dead_code)]
impl<T> CBox<T> {
    /// Creates a new heap allocated [`CBox`] object.
    pub(crate) fn new(value: T) -> Option<Self> {
        let size = core::mem::size_of::<T>();
        match size {
            #[allow(unused_unsafe)]
            // SAFETY: This operation is safe.
            0 => unsafe { Some(Self(nstd_core_ptr_raw_dangling_mut(), PhantomData)) },
            // SAFETY: `size` is greater than 0.
            _ => match unsafe { nstd_alloc_allocate(size) } {
                NSTD_NULL => None,
                mem => {
                    // SAFETY: `mem` is a non-null pointer to `size` uninitialized bytes.
                    unsafe { nstd_core_mem_copy(mem.cast(), addr_of!(value).cast(), size) };
                    core::mem::forget(value);
                    Some(Self(mem, PhantomData))
                }
            },
        }
    }

    /// Moves a [`CBox`] value onto the stack.
    pub(crate) fn into_inner(mut self) -> T {
        // SAFETY: `self.0` points to a valid object of type `T`.
        let value = unsafe { (self.0 as *const T).read() };
        let size = core::mem::size_of::<T>();
        if size > 0 {
            // SAFETY:
            // - `self.0` points to a valid object of type `T`.
            // - `size` is greater than 0.
            unsafe { nstd_alloc_deallocate(&mut self.0, size) };
        }
        core::mem::forget(self);
        value
    }
}
impl<T> Deref for CBox<T> {
    /// [`CBox`]'s dereference target.
    type Target = T;

    /// Immutably dereferences a [`CBox`].
    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.0` points to a valid object of type `T`.
        unsafe { &*(self.0 as *const _) }
    }
}
impl<T> DerefMut for CBox<T> {
    /// Mutably dereferences a [`CBox`].
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.0` points to a valid object of type `T`.
        unsafe { &mut *self.0.cast() }
    }
}
impl<T> Drop for CBox<T> {
    /// [`CBox`]'s destructor.
    fn drop(&mut self) {
        // SAFETY:
        // - `self.0` points to a valid object of type `T`.
        // - `size` is greater than 0.
        unsafe {
            drop(self.0.cast::<T>().read());
            let size = core::mem::size_of::<T>();
            if size > 0 {
                nstd_alloc_deallocate(&mut self.0, size);
            }
        }
    }
}

/// Describes an error returned from allocation functions.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDAllocError {
    /// No error occurred.
    NSTD_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
    /// Deallocating memory failed.
    NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND,
    /// Getting a handle to a heap failed.
    NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
    /// A heap is invalid.
    NSTD_ALLOC_ERROR_INVALID_HEAP,
    /// An allocation function received input parameters that resulted in an invalid memory layout.
    NSTD_ALLOC_ERROR_INVALID_LAYOUT,
}
#[cfg(windows)]
impl From<NSTDWindowsAllocError> for NSTDAllocError {
    /// Converts an [`NSTDWindowsAllocError`] into an [`NSTDAllocError`].
    fn from(err: NSTDWindowsAllocError) -> Self {
        match err {
            NSTD_WINDOWS_ALLOC_ERROR_NONE => Self::NSTD_ALLOC_ERROR_NONE,
            NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY => Self::NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
            NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND => Self::NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND,
            NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND => Self::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND,
            NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP => Self::NSTD_ALLOC_ERROR_INVALID_HEAP,
            NSTD_WINDOWS_ALLOC_ERROR_INVALID_LAYOUT => Self::NSTD_ALLOC_ERROR_INVALID_LAYOUT,
        }
    }
}

/// A structure of function pointers making up an allocator's virtual function table.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDAllocator {
    /// An opaque pointer to the allocator's state.
    pub state: NSTDAny,
    /// Allocates a contiguous sequence of `size` bytes in memory.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// If allocation succeeds, this returns a pointer that is suitably aligned for any type with
    /// [fundamental alignment](https://en.cppreference.com/w/c/language/object#Alignment), i.e.,
    /// the returned pointer will be suitably aligned for
    /// [max_align_t](https://en.cppreference.com/w/c/types/max_align_t).
    ///
    /// Allocation will fail if `size` is greater than `NSTDInt`'s max value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `size` is zero.
    ///
    /// - The new memory buffer should be considered uninitialized.
    pub allocate: unsafe extern "C" fn(NSTDAny, NSTDUInt) -> NSTDAnyMut,
    /// Allocates a contiguous sequence of `size` bytes in memory.
    ///
    /// The initialized memory is zero-initialized.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// If allocation succeeds, this returns a pointer that is suitably aligned for any type with
    /// [fundamental alignment](https://en.cppreference.com/w/c/language/object#Alignment), i.e.,
    /// the returned pointer will be suitably aligned for
    /// [max_align_t](https://en.cppreference.com/w/c/types/max_align_t).
    ///
    /// Allocation will fail if `size` is greater than `NSTDInt`'s max value.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `size` is zero.
    ///
    /// - The new memory buffer should be considered uninitialized.
    pub allocate_zeroed: unsafe extern "C" fn(NSTDAny, NSTDUInt) -> NSTDAnyMut,
    /// Reallocates memory that was previously allocated by this allocator.
    ///
    /// Reallocation will fail if `new_size` is greater than `NSTDInt`'s max value.
    ///
    /// On successful reallocation, `ptr` will point to the new memory location and
    /// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and reallocation fails,
    /// the pointer will remain untouched and the appropriate error is returned.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
    ///
    /// - `NSTDUInt size` - The number of bytes currently allocated.
    ///
    /// - `NSTDUInt new_size` - The number of bytes to reallocate.
    ///
    /// # Returns
    ///
    /// `NSTDAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `new_size` is zero.
    ///
    /// - Behavior is undefined if `ptr` is not a value returned by this allocator.
    ///
    /// - `size` must be the same value that was used to allocate the memory buffer.
    pub reallocate:
        unsafe extern "C" fn(NSTDAny, &mut NSTDAnyMut, NSTDUInt, NSTDUInt) -> NSTDAllocError,
    /// Deallocates memory that was previously allocated by this allocator.
    ///
    /// On successful deallocation, `ptr` will be set to null and `NSTD_ALLOC_ERROR_NONE` will be
    /// returned. If this is not the case and deallocation fails, the pointer will remain untouched
    /// and the appropriate error is returned.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to
    /// null.
    ///
    /// - `NSTDUInt size` - The number of bytes currently allocated.
    ///
    /// # Returns
    ///
    /// `NSTDAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `ptr` is not a value returned by this allocator.
    ///
    /// - `size` must be the same value that was used to allocate the memory buffer.
    pub deallocate: unsafe extern "C" fn(NSTDAny, &mut NSTDAnyMut, NSTDUInt) -> NSTDAllocError,
}
/// # Safety
///
/// The allocator's state must be able to be safely *shared* between threads.
// SAFETY: The user guarantees that the state is thread-safe.
unsafe impl Send for NSTDAllocator {}
/// # Safety
///
/// The allocator's state must be able to be safely shared between threads.
// SAFETY: The user guarantees that the state is thread-safe.
unsafe impl Sync for NSTDAllocator {}

/// Forwards an `NSTD_ALLOCATOR`'s `allocate` call to `nstd_alloc_allocate`.
#[inline]
unsafe extern "C" fn allocate(_: NSTDAny, size: NSTDUInt) -> NSTDAnyMut {
    nstd_alloc_allocate(size)
}

/// Forwards an `NSTD_ALLOCATOR`'s `allocate_zeroed` call to `nstd_alloc_allocate_zeroed`.
#[inline]
unsafe extern "C" fn allocate_zeroed(_: NSTDAny, size: NSTDUInt) -> NSTDAnyMut {
    nstd_alloc_allocate_zeroed(size)
}

/// Forwards an `NSTD_ALLOCATOR`'s `reallocate` call to `nstd_alloc_reallocate`.
#[inline]
unsafe extern "C" fn reallocate(
    _: NSTDAny,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
    new_size: NSTDUInt,
) -> NSTDAllocError {
    nstd_alloc_reallocate(ptr, size, new_size)
}

/// Forwards an `NSTD_ALLOCATOR`'s `deallocate` call to `nstd_alloc_deallocate`.
#[inline]
unsafe extern "C" fn deallocate(
    _: NSTDAny,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDAllocError {
    nstd_alloc_deallocate(ptr, size)
}

/// `nstd`'s default allocator.
#[nstdapi]
pub static NSTD_ALLOCATOR: NSTDAllocator = NSTDAllocator {
    state: NSTD_NULL,
    allocate,
    allocate_zeroed,
    reallocate,
    deallocate,
};

/// The `NSTDAllocator`'s `allocate` function.
#[inline]
unsafe extern "C" fn rust_allocate(_: NSTDAny, size: NSTDUInt) -> NSTDAnyMut {
    if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
        return alloc::alloc::alloc(layout).cast();
    }
    NSTD_NULL
}

/// The `NSTDAllocator`'s `allocate_zeroed` function.
#[inline]
unsafe extern "C" fn rust_allocate_zeroed(_: NSTDAny, size: NSTDUInt) -> NSTDAnyMut {
    if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
        return alloc::alloc::alloc_zeroed(layout).cast();
    }
    NSTD_NULL
}

/// The `NSTDAllocator`'s `reallocate` function.
unsafe extern "C" fn rust_reallocate(
    _: NSTDAny,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
    new_size: NSTDUInt,
) -> NSTDAllocError {
    if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
        let new_mem = alloc::alloc::realloc((*ptr).cast(), layout, new_size);
        if new_mem.is_null() {
            return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
        }
        *ptr = new_mem.cast();
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
}

/// The `NSTDAllocator`'s `deallocate` function.
unsafe extern "C" fn rust_deallocate(
    _: NSTDAny,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDAllocError {
    if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
        alloc::alloc::dealloc((*ptr).cast(), layout);
        *ptr = NSTD_NULL;
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
}

/// Rust's [Global] [`NSTDAllocator`].
#[allow(dead_code)]
pub(crate) static GLOBAL_ALLOCATOR: NSTDAllocator = NSTDAllocator {
    state: NSTD_NULL,
    allocate: rust_allocate,
    allocate_zeroed: rust_allocate_zeroed,
    reallocate: rust_reallocate,
    deallocate: rust_deallocate,
};

/// Allocates a block of memory on the heap.
/// The number of bytes to be allocated is specified by `size`.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// - Behavior is undefined if `size` is zero.
///
/// - The new memory buffer should be considered uninitialized.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{nstd_alloc_allocate, nstd_alloc_deallocate};
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate(32);
///     assert!(!mem.is_null());
///     nstd_alloc_deallocate(&mut mem, 32);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_alloc_allocate(size: NSTDUInt) -> NSTDAnyMut {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "solid_asp3"
        ))] {
            use crate::NSTD_INT_MAX;
            match size <= NSTD_INT_MAX as _ {
                true => libc::malloc(size),
                false => NSTD_NULL,
            }
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_allocate(size)
        } else {
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                return alloc::alloc::alloc(layout).cast();
            }
            NSTD_NULL
        }
    }
}

/// Allocates a block of zero-initialized memory on the heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// Behavior is undefined if `size` is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{nstd_alloc_allocate_zeroed, nstd_alloc_deallocate};
///
/// const SIZE: usize = core::mem::size_of::<[i16; 16]>();
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate_zeroed(SIZE);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<[i16; 16]>() == [0i16; 16]);
///
///     nstd_alloc_deallocate(&mut mem, SIZE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_alloc_allocate_zeroed(size: NSTDUInt) -> NSTDAnyMut {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "solid_asp3"
        ))] {
            use crate::NSTD_INT_MAX;
            match size <= NSTD_INT_MAX as _ {
                true => libc::calloc(size, 1),
                false => NSTD_NULL,
            }
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_allocate_zeroed(size)
        } else {
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                return alloc::alloc::alloc_zeroed(layout).cast();
            }
            NSTD_NULL
        }
    }
}

/// Reallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and
/// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and allocation fails, the
/// pointer will remain untouched and the appropriate error is returned.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUInt size` - The number of bytes currently allocated.
///
/// - `NSTDUInt new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_size` is zero.
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{
///     nstd_alloc_allocate_zeroed, nstd_alloc_deallocate, nstd_alloc_reallocate,
///     NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
/// };
///
/// const SIZE: usize = core::mem::size_of::<[u64; 64]>();
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate_zeroed(SIZE);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<[u64; 64]>() == [0u64; 64]);
///
///     assert!(nstd_alloc_reallocate(&mut mem, SIZE, SIZE / 2) == NSTD_ALLOC_ERROR_NONE);
///     assert!(*mem.cast::<[u64; 32]>() == [0u64; 32]);
///
///     nstd_alloc_deallocate(&mut mem, SIZE);
/// }
/// ```
#[nstdapi]
#[cfg_attr(windows, inline)]
#[allow(unused_variables)]
pub unsafe fn nstd_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
    new_size: NSTDUInt,
) -> NSTDAllocError {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "solid_asp3"
        ))] {
            use crate::NSTD_INT_MAX;
            if new_size > NSTD_INT_MAX as _ {
                return NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT;
            }
            let new_mem = libc::realloc(*ptr, new_size);
            if new_mem.is_null() {
                return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
            }
            *ptr = new_mem;
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_reallocate(ptr, new_size).into()
        } else {
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                let new_mem = alloc::alloc::realloc((*ptr).cast(), layout, new_size);
                if new_mem.is_null() {
                    return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
                }
                *ptr = new_mem.cast();
                return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
            }
            NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
        }
    }
}

/// Deallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory, once freed the pointer is set to null.
///
/// - `NSTDUInt size` - The number of bytes to free.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `ptr` is not a value returned by `nstd_alloc_allocate[_zeroed]`.
///
/// - `size` must be the same value that was used to allocate the memory buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::alloc::{nstd_alloc_allocate, nstd_alloc_deallocate};
///
/// unsafe {
///     let mut mem = nstd_alloc_allocate(24);
///     assert!(!mem.is_null());
///     nstd_alloc_deallocate(&mut mem, 24);
/// }
/// ```
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub unsafe fn nstd_alloc_deallocate(ptr: &mut NSTDAnyMut, size: NSTDUInt) -> NSTDAllocError {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "solid_asp3"
        ))] {
            libc::free(*ptr);
            *ptr = NSTD_NULL;
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_deallocate(ptr).into()
        } else {
            if let Ok(layout) = Layout::from_size_align(size, MAX_ALIGN) {
                alloc::alloc::dealloc((*ptr).cast(), layout);
                *ptr = NSTD_NULL;
                return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
            }
            NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
        }
    }
}
