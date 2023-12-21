//! Low level memory allocation.
extern crate alloc;
#[cfg(windows)]
use crate::os::windows::alloc::{
    nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_allocate_zeroed,
    nstd_os_windows_alloc_deallocate,
    NSTDWindowsAllocError::{
        self, NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND, NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP,
        NSTD_WINDOWS_ALLOC_ERROR_INVALID_LAYOUT, NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND,
        NSTD_WINDOWS_ALLOC_ERROR_NONE, NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY,
    },
};
use crate::{
    core::{
        alloc::{
            nstd_core_alloc_layout_align, nstd_core_alloc_layout_new,
            nstd_core_alloc_layout_new_unchecked, nstd_core_alloc_layout_size, NSTDAllocLayout,
        },
        mem::{nstd_core_mem_copy, nstd_core_mem_dangling_mut},
        optional::NSTDOptional,
    },
    NSTDAny, NSTDAnyMut, NSTD_NULL,
};
use cfg_if::cfg_if;
use core::{
    alloc::Layout,
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
        match core::mem::size_of::<T>() {
            #[allow(unused_unsafe)]
            // SAFETY: This operation is safe.
            0 => unsafe { Some(Self(nstd_core_mem_dangling_mut(), PhantomData)) },
            size => {
                #[allow(unused_unsafe)]
                // SAFETY: This operation is safe.
                match unsafe { nstd_core_alloc_layout_new(size, core::mem::align_of::<T>()) } {
                    // SAFETY: `size` is greater than 0.
                    NSTDOptional::Some(layout) => match unsafe { nstd_alloc_allocate(layout) } {
                        NSTD_NULL => None,
                        mem => {
                            // SAFETY: `mem` is a non-null pointer to `size` uninitialized bytes.
                            unsafe { nstd_core_mem_copy(mem.cast(), addr_of!(value).cast(), size) };
                            core::mem::forget(value);
                            Some(Self(mem, PhantomData))
                        }
                    },
                    NSTDOptional::None => None,
                }
            }
        }
    }

    /// Moves a [`CBox`] value onto the stack.
    pub(crate) fn into_inner(self) -> T {
        // SAFETY: `self.0` points to a valid object of type `T`.
        let value = unsafe { (self.0 as *const T).read() };
        let size = core::mem::size_of::<T>();
        if size > 0 {
            let align = core::mem::align_of::<T>();
            // SAFETY:
            // - `size` is never greater than `NSTDInt`'s max value.
            // - `align` is a nonzero power of two.
            let layout = unsafe { nstd_core_alloc_layout_new_unchecked(size, align) };
            // SAFETY:
            // - `self.0` points to a valid object of type `T`.
            // - `size` is greater than 0.
            unsafe { nstd_alloc_deallocate(self.0, layout) };
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
                let align = core::mem::align_of::<T>();
                let layout = nstd_core_alloc_layout_new_unchecked(size, align);
                nstd_alloc_deallocate(self.0, layout);
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
    /// Allocates a new block of memory.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// If allocation succeeds, this returns a pointer to the new memory that is suitably aligned
    /// for `layout`'s alignment and the number of bytes allocated is at least equal to `layout`'s
    /// size.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `layout`'s size is zero.
    ///
    /// - The new memory buffer should be considered uninitialized.
    pub allocate: unsafe extern "C" fn(NSTDAny, NSTDAllocLayout) -> NSTDAnyMut,
    /// Allocates a new block of zero-initialized memory.
    ///
    /// If allocation fails, a null pointer is returned.
    ///
    /// If allocation succeeds, this returns a pointer to the new memory that is suitably aligned
    /// for `layout`'s alignment and the number of bytes allocated is at least equal to `layout`'s
    /// size.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if `layout`'s size is zero.
    pub allocate_zeroed: unsafe extern "C" fn(NSTDAny, NSTDAllocLayout) -> NSTDAnyMut,
    /// Reallocates memory that was previously allocated by this allocator.
    ///
    /// On successful reallocation, `ptr` will point to the new memory location and
    /// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and reallocation fails,
    /// the pointer will remain untouched and the appropriate error is returned.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
    ///
    /// - `NSTDAllocLayout old_layout` - Describes the previous memory layout.
    ///
    /// - `NSTDAllocLayout new_layout` - Describes the new memory layout to allocate for.
    ///
    /// # Returns
    ///
    /// `NSTDAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `new_layout`'s size is zero.
    ///
    /// - Behavior is undefined if `ptr` is not a pointer to memory allocated by this allocator.
    ///
    /// - `old_layout` must be the same value that was used to allocate the memory buffer.
    pub reallocate: unsafe extern "C" fn(
        NSTDAny,
        &mut NSTDAnyMut,
        NSTDAllocLayout,
        NSTDAllocLayout,
    ) -> NSTDAllocError,
    /// Deallocates memory that was previously allocated by this allocator.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut ptr` - A pointer to the allocated memory.
    ///
    /// - `NSTDAllocLayout layout` - Describes the layout of memory that `ptr` points to.
    ///
    /// # Returns
    ///
    /// `NSTDAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// - Behavior is undefined if `ptr` is not a pointer to memory allocated by this allocator.
    ///
    /// - `layout` must be the same value that was used to allocate the memory buffer.
    pub deallocate: unsafe extern "C" fn(NSTDAny, NSTDAnyMut, NSTDAllocLayout) -> NSTDAllocError,
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
unsafe extern "C" fn allocate(_: NSTDAny, layout: NSTDAllocLayout) -> NSTDAnyMut {
    nstd_alloc_allocate(layout)
}

/// Forwards an `NSTD_ALLOCATOR`'s `allocate_zeroed` call to `nstd_alloc_allocate_zeroed`.
#[inline]
unsafe extern "C" fn allocate_zeroed(_: NSTDAny, layout: NSTDAllocLayout) -> NSTDAnyMut {
    nstd_alloc_allocate_zeroed(layout)
}

/// Forwards an `NSTD_ALLOCATOR`'s `reallocate` call to `nstd_alloc_reallocate`.
#[inline]
unsafe extern "C" fn reallocate(
    _: NSTDAny,
    ptr: &mut NSTDAnyMut,
    old_layout: NSTDAllocLayout,
    new_layout: NSTDAllocLayout,
) -> NSTDAllocError {
    nstd_alloc_reallocate(ptr, old_layout, new_layout)
}

/// Forwards an `NSTD_ALLOCATOR`'s `deallocate` call to `nstd_alloc_deallocate`.
#[inline]
unsafe extern "C" fn deallocate(
    _: NSTDAny,
    ptr: NSTDAnyMut,
    layout: NSTDAllocLayout,
) -> NSTDAllocError {
    nstd_alloc_deallocate(ptr, layout)
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
unsafe extern "C" fn rust_allocate(_: NSTDAny, layout: NSTDAllocLayout) -> NSTDAnyMut {
    let size = nstd_core_alloc_layout_size(layout);
    let align = nstd_core_alloc_layout_align(layout);
    if let Ok(layout) = Layout::from_size_align(size, align) {
        return alloc::alloc::alloc(layout).cast();
    }
    NSTD_NULL
}

/// The `NSTDAllocator`'s `allocate_zeroed` function.
#[inline]
unsafe extern "C" fn rust_allocate_zeroed(_: NSTDAny, layout: NSTDAllocLayout) -> NSTDAnyMut {
    let size = nstd_core_alloc_layout_size(layout);
    let align = nstd_core_alloc_layout_align(layout);
    if let Ok(layout) = Layout::from_size_align(size, align) {
        return alloc::alloc::alloc_zeroed(layout).cast();
    }
    NSTD_NULL
}

/// The `NSTDAllocator`'s `reallocate` function.
unsafe extern "C" fn rust_reallocate(
    this: NSTDAny,
    ptr: &mut NSTDAnyMut,
    old_layout: NSTDAllocLayout,
    new_layout: NSTDAllocLayout,
) -> NSTDAllocError {
    if old_layout != new_layout {
        let new_mem = rust_allocate(this, new_layout);
        if new_mem.is_null() {
            return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
        }
        let old_size = nstd_core_alloc_layout_size(old_layout);
        let new_size = nstd_core_alloc_layout_size(new_layout);
        nstd_core_mem_copy(new_mem.cast(), (*ptr).cast(), old_size.min(new_size));
        rust_deallocate(this, *ptr, old_layout);
        *ptr = new_mem;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_NONE
}

/// The `NSTDAllocator`'s `deallocate` function.
unsafe extern "C" fn rust_deallocate(
    _: NSTDAny,
    ptr: NSTDAnyMut,
    layout: NSTDAllocLayout,
) -> NSTDAllocError {
    let size = nstd_core_alloc_layout_size(layout);
    let align = nstd_core_alloc_layout_align(layout);
    if let Ok(layout) = Layout::from_size_align(size, align) {
        alloc::alloc::dealloc(ptr.cast(), layout);
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

/// Allocates a new block of memory.
///
/// If allocation fails, a null pointer is returned.
///
/// If allocation succeeds, this returns a pointer to the new memory that is suitably aligned for
/// `layout`'s alignment and the number of bytes allocated is at least equal to `layout`'s size.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// - Behavior is undefined if `layout`'s size is zero.
///
/// - The new memory buffer should be considered uninitialized.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{nstd_alloc_allocate, nstd_alloc_deallocate, NSTDAllocError::NSTD_ALLOC_ERROR_NONE},
///     core::alloc::nstd_core_alloc_layout_new,
/// };
///
/// unsafe {
///     let layout = nstd_core_alloc_layout_new(32, 1).unwrap();
///     let mem = nstd_alloc_allocate(layout);
///     assert!(!mem.is_null());
///     assert!(nstd_alloc_deallocate(mem, layout) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_alloc_allocate(layout: NSTDAllocLayout) -> NSTDAnyMut {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "teeos"
        ))] {
            let size = nstd_core_alloc_layout_size(layout);
            let min_align = core::mem::size_of::<NSTDAnyMut>();
            let align = nstd_core_alloc_layout_align(layout).max(min_align);
            let mut ptr = NSTD_NULL;
            libc::posix_memalign(&mut ptr, align, size);
            ptr
        } else if #[cfg(target_os = "solid_asp3")] {
            use crate::NSTD_INT_MAX;
            let mut size = nstd_core_alloc_layout_size(layout);
            let align = nstd_core_alloc_layout_align(layout);
            #[allow(clippy::arithmetic_side_effects)]
            let off = size % align;
            #[allow(clippy::arithmetic_side_effects)]
            if off != 0 {
                size = match size.checked_add(align - off) {
                    Some(size) if size <= NSTD_INT_MAX => size,
                    _ => return NSTD_NULL,
                };
            }
            libc::aligned_alloc(align, size)
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_allocate(layout)
        } else {
            let size = nstd_core_alloc_layout_size(layout);
            let align = nstd_core_alloc_layout_align(layout);
            if let Ok(layout) = Layout::from_size_align(size, align) {
                return alloc::alloc::alloc(layout).cast();
            }
            NSTD_NULL
        }
    }
}

/// Allocates a new block of zero-initialized memory.
///
/// If allocation fails, a null pointer is returned.
///
/// If allocation succeeds, this returns a pointer to the new memory that is suitably aligned
/// for `layout`'s alignment and the number of bytes allocated is at least equal to `layout`'s
/// size.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// Behavior is undefined if `layout`'s size is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{
///         nstd_alloc_allocate_zeroed, nstd_alloc_deallocate,
///         NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     },
///     core::alloc::nstd_core_alloc_layout_new,
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<[i16; 16]>();
///     let align = core::mem::align_of::<[i16; 16]>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mem = nstd_alloc_allocate_zeroed(layout);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<[i16; 16]>() == [0i16; 16]);
///     assert!(nstd_alloc_deallocate(mem, layout) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_alloc_allocate_zeroed(layout: NSTDAllocLayout) -> NSTDAnyMut {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "solid_asp3",
            target_os = "teeos"
        ))] {
            use crate::core::mem::nstd_core_mem_zero;
            let ptr = nstd_alloc_allocate(layout);
            if !ptr.is_null() {
                nstd_core_mem_zero(ptr.cast(), nstd_core_alloc_layout_size(layout));
            }
            ptr
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_allocate_zeroed(layout)
        } else {
            let size = nstd_core_alloc_layout_size(layout);
            let align = nstd_core_alloc_layout_align(layout);
            if let Ok(layout) = Layout::from_size_align(size, align) {
                return alloc::alloc::alloc_zeroed(layout).cast();
            }
            NSTD_NULL
        }
    }
}

/// Reallocates memory that was previously allocated by this allocator.
///
/// On successful reallocation, `ptr` will point to the new memory location and
/// `NSTD_ALLOC_ERROR_NONE` will be returned. If this is not the case and reallocation fails,
/// the pointer will remain untouched and the appropriate error is returned.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDAllocLayout old_layout` - Describes the previous memory layout.
///
/// - `NSTDAllocLayout new_layout` - Describes the new memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_layout`'s size is zero.
///
/// - Behavior is undefined if `ptr` is not a pointer to memory allocated by this allocator.
///
/// - `old_layout` must be the same value that was used to allocate the memory buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{
///         nstd_alloc_allocate_zeroed, nstd_alloc_deallocate, nstd_alloc_reallocate,
///         NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     },
///     core::alloc::nstd_core_alloc_layout_new,
/// };
///
///
/// unsafe {
///     let mut size = core::mem::size_of::<[u64; 64]>();
///     let mut align = core::mem::align_of::<[u64; 64]>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mut mem = nstd_alloc_allocate_zeroed(layout);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<[u64; 64]>() == [0u64; 64]);
///
///     size = core::mem::size_of::<[u64; 32]>();
///     align = core::mem::align_of::<[u64; 32]>();
///     let new_layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     assert!(nstd_alloc_reallocate(&mut mem, layout, new_layout) == NSTD_ALLOC_ERROR_NONE);
///     assert!(*mem.cast::<[u64; 32]>() == [0u64; 32]);
///
///     assert!(nstd_alloc_deallocate(mem, new_layout) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    old_layout: NSTDAllocLayout,
    new_layout: NSTDAllocLayout,
) -> NSTDAllocError {
    if old_layout != new_layout {
        let new_mem = nstd_alloc_allocate(new_layout);
        if new_mem.is_null() {
            return NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY;
        }
        let old_size = nstd_core_alloc_layout_size(old_layout);
        let new_size = nstd_core_alloc_layout_size(new_layout);
        nstd_core_mem_copy(new_mem.cast(), (*ptr).cast(), old_size.min(new_size));
        nstd_alloc_deallocate(*ptr, old_layout);
        *ptr = new_mem;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_NONE
}

/// Deallocates memory that was previously allocated by this allocator.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the allocated memory.
///
/// - `NSTDAllocLayout layout` - Describes the layout of memory that `ptr` points to.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `ptr` is not a pointer to memory allocated by this allocator.
///
/// - `layout` must be the same value that was used to allocate the memory buffer.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::{nstd_alloc_allocate, nstd_alloc_deallocate, NSTDAllocError::NSTD_ALLOC_ERROR_NONE},
///     core::alloc::nstd_core_alloc_layout_new,
/// };
///
/// unsafe {
///     let layout = nstd_core_alloc_layout_new(24, 1).unwrap();
///     let mem = nstd_alloc_allocate(layout);
///     assert!(!mem.is_null());
///     assert!(nstd_alloc_deallocate(mem, layout) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub unsafe fn nstd_alloc_deallocate(ptr: NSTDAnyMut, layout: NSTDAllocLayout) -> NSTDAllocError {
    cfg_if! {
        if #[cfg(any(
            unix,
            any(target_env = "wasi", target_os = "wasi"),
            target_os = "solid_asp3",
            target_os = "teeos"
        ))] {
            libc::free(ptr);
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        } else if #[cfg(windows)] {
            nstd_os_windows_alloc_deallocate(ptr);
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        } else {
            let size = nstd_core_alloc_layout_size(layout);
            let align = nstd_core_alloc_layout_align(layout);
            if let Ok(layout) = Layout::from_size_align(size, align) {
                alloc::alloc::dealloc(ptr.cast(), layout);
                return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
            }
            NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT
        }
    }
}
