//! Memory allocation for Unix-like systems.
use crate::{NSTDAnyMut, NSTDUInt};
#[cfg(all(feature = "asm", target_arch = "x86_64", not(target_os = "macos")))]
core::arch::global_asm!(include_str!("alloc/x86_64/alloc.asm"));
#[cfg(all(feature = "asm", target_arch = "x86_64", target_os = "macos"))]
core::arch::global_asm!(include_str!("alloc/x86_64/macos/alloc.asm"));

/// Describes an error returned from an `nstd.os.unix.alloc` function.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDUnixAllocError {
    /// No error occurred.
    NSTD_UNIX_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_UNIX_ALLOC_ERROR_OUT_OF_MEMORY,
}

extern "C" {
    /// Allocates a block of memory on the heap, returning a pointer to it.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate for the new block of memory.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
    ///
    /// # Safety
    ///
    /// See <https://man7.org/linux/man-pages/man3/malloc.3.html>.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::os::unix::alloc::{nstd_os_unix_alloc_allocate, nstd_os_unix_alloc_deallocate};
    ///
    /// unsafe {
    ///     let mut mem = nstd_os_unix_alloc_allocate(24);
    ///     assert!(!mem.is_null());
    ///     nstd_os_unix_alloc_deallocate(&mut mem);
    /// }
    /// ```
    pub fn nstd_os_unix_alloc_allocate(size: NSTDUInt) -> NSTDAnyMut;

    /// Allocates a block of zero initialized memory on the heap, returning a pointer to it.
    ///
    /// # Parameters:
    ///
    /// - `NSTDUInt size` - The number of bytes to allocate for the new block of memory.
    ///
    /// # Returns
    ///
    /// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
    ///
    /// # Safety
    ///
    /// See <https://man7.org/linux/man-pages/man3/calloc.3p.html>.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::os::unix::alloc::{
    ///     nstd_os_unix_alloc_allocate_zeroed, nstd_os_unix_alloc_deallocate,
    /// };
    ///
    /// const SIZE: usize = core::mem::size_of::<isize>();
    ///
    /// unsafe {
    ///     let mut mem = nstd_os_unix_alloc_allocate_zeroed(SIZE);
    ///     assert!(!mem.is_null());
    ///     assert!(*mem.cast::<isize>() == 0);
    ///     nstd_os_unix_alloc_deallocate(&mut mem);
    /// }
    /// ```
    pub fn nstd_os_unix_alloc_allocate_zeroed(size: NSTDUInt) -> NSTDAnyMut;

    /// Reallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the block of memory to reallocate.
    ///
    /// - `NSTDUInt new_size` - The new size of the memory block.
    ///
    /// # Returns
    ///
    /// `NSTDUnixAllocError errc` - The allocation operation error code.
    ///
    /// # Safety
    ///
    /// See <https://man7.org/linux/man-pages/man3/realloc.3p.html>.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::os::unix::alloc::{
    ///     nstd_os_unix_alloc_allocate_zeroed, nstd_os_unix_alloc_deallocate,
    ///     nstd_os_unix_alloc_reallocate, NSTDUnixAllocError::NSTD_UNIX_ALLOC_ERROR_NONE,
    /// };
    ///
    /// const SIZE: usize = core::mem::size_of::<u64>();
    /// const NEW_SIZE: usize = core::mem::size_of::<u32>();
    ///
    /// unsafe {
    ///     let mut mem = nstd_os_unix_alloc_allocate_zeroed(SIZE);
    ///     assert!(!mem.is_null());
    ///     let errc = nstd_os_unix_alloc_reallocate(&mut mem, NEW_SIZE);
    ///     assert!(errc == NSTD_UNIX_ALLOC_ERROR_NONE);
    ///     assert!(*mem.cast::<u32>() == 0);
    ///     nstd_os_unix_alloc_deallocate(&mut mem);
    /// }
    /// ```
    pub fn nstd_os_unix_alloc_reallocate(
        ptr: &mut NSTDAnyMut,
        new_size: NSTDUInt,
    ) -> NSTDUnixAllocError;

    /// Deallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
    ///
    /// # Parameters:
    ///
    /// - `NSTDAnyMut *ptr` - A pointer to the block of memory to free.
    ///
    /// # Safety
    ///
    /// See <https://man7.org/linux/man-pages/man3/free.3p.html>.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::os::unix::alloc::{nstd_os_unix_alloc_allocate, nstd_os_unix_alloc_deallocate};
    ///
    /// unsafe {
    ///     let mut mem = nstd_os_unix_alloc_allocate(32);
    ///     assert!(!mem.is_null());
    ///     nstd_os_unix_alloc_deallocate(&mut mem);
    /// }
    /// ```
    pub fn nstd_os_unix_alloc_deallocate(ptr: &mut NSTDAnyMut);
}
