//! Provides common I/O operations for working with Unix file descriptors.
#![allow(unused)]
use super::{
    NSTDUnixFileDescriptor,
    NSTDUnixIOError::{self, *},
    NSTDUnixIOResult,
};
use crate::{
    alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
    core::{
        result::NSTDResult,
        slice::{
            nstd_core_slice_as_ptr, nstd_core_slice_len, nstd_core_slice_mut_as_ptr,
            nstd_core_slice_mut_len, nstd_core_slice_mut_stride, nstd_core_slice_stride, NSTDSlice,
            NSTDSliceMut,
        },
    },
    string::NSTDString,
    vec::{
        nstd_vec_cap, nstd_vec_end, nstd_vec_end_mut, nstd_vec_len, nstd_vec_reserve,
        nstd_vec_set_len, nstd_vec_stride, NSTDVec,
    },
    NSTDUInt,
};
use libc::{lseek, SEEK_CUR, SEEK_END, SEEK_SET};

/// `isize::MAX` as a [usize].
const ISIZE_MAX: usize = isize::MAX as usize;

/// Writes some `nstd` bytes to a Unix file.
///
/// # Safety
///
/// - `fd` must be a valid Unix file descriptor with write access.
///
/// - `fd` will not be locked by this operation, it is up to the runtime to ensure that access to
/// the file is properly synchronized within the process(es).
///
/// - `bytes` must be valid for reads.
pub(crate) unsafe fn write(fd: NSTDUnixFileDescriptor, bytes: &NSTDSlice) -> NSTDUnixIOResult {
    let len = nstd_core_slice_len(bytes);
    // Make sure the slice's element size is 1.
    if nstd_core_slice_stride(bytes) != 1 || len > ISIZE_MAX {
        return NSTDResult::Err(NSTD_UNIX_IO_ERROR_INVALID_INPUT);
    }
    // Check if `len` is 0.
    if len == 0 {
        return NSTDResult::Ok(0);
    }
    // Write the data.
    match libc::write(fd, nstd_core_slice_as_ptr(bytes), len) {
        -1 => NSTDResult::Err(NSTDUnixIOError::last()),
        w => NSTDResult::Ok(w as _),
    }
}

/// Writes a full `nstd` byte slice to a Unix file.
///
/// # Safety
///
/// - `fd` must be a valid Unix file descriptor with write access.
///
/// - `fd` will not be locked by this operation, it is up to the runtime to ensure that access to
/// the file is properly synchronized within the process(es).
///
/// - `bytes` must be valid for reads.
pub(crate) unsafe fn write_all(fd: NSTDUnixFileDescriptor, bytes: &NSTDSlice) -> NSTDUnixIOError {
    let len = nstd_core_slice_len(bytes);
    // Make sure the slice's element size is 1.
    if nstd_core_slice_stride(bytes) != 1 || len > ISIZE_MAX {
        return NSTD_UNIX_IO_ERROR_INVALID_INPUT;
    }
    // Write the data.
    let mut written = 0;
    let mut pos = nstd_core_slice_as_ptr(bytes);
    while written < len {
        match libc::write(fd, pos, len - written) {
            -1 => match NSTDUnixIOError::last() {
                NSTD_UNIX_IO_ERROR_INTERRUPTED => (),
                err => return err,
            },
            w => {
                written += w as NSTDUInt;
                pos = pos.offset(w);
            }
        }
    }
    NSTD_UNIX_IO_ERROR_NONE
}

/// Reads some data from a Unix file into an `nstd` byte slice.
///
/// # Safety
///
/// - `fd` must be a valid Unix file descriptor with read access.
///
/// - `fd` will not be locked by this operation, it is up to the runtime to ensure that access to
/// the file is properly synchronized within the process(es).
///
/// `buffer`'s data must be valid for writes.
pub(crate) unsafe fn read(
    fd: NSTDUnixFileDescriptor,
    buffer: &mut NSTDSliceMut,
) -> NSTDUnixIOResult {
    // Make sure the buffer's element size is 1.
    let len = nstd_core_slice_mut_len(buffer);
    if nstd_core_slice_mut_stride(buffer) != 1 || len > ISIZE_MAX {
        return NSTDResult::Err(NSTD_UNIX_IO_ERROR_INVALID_INPUT);
    }
    // Read data into `buffer`.
    match libc::read(fd, nstd_core_slice_mut_as_ptr(buffer), len) {
        -1 => NSTDResult::Err(NSTDUnixIOError::last()),
        r => NSTDResult::Ok(r as _),
    }
}

/// Extends a vector with data from a Unix file until the end of the file is reached.
///
/// This will return an error variant of `NSTD_UNIX_IO_ERROR_INVALID_INPUT` in an attempt to read
/// more than `NSTDInt::MAX` bytes.
///
/// If extending the buffer fails, an error code of `NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY` will be
/// returned. This does not mean there were no bytes read from `stream` in this case.
///
/// # Panics
///
/// This operation will panic if `buffer`'s length in bytes ends up exceeding `NSTDInt::MAX`.
///
/// # Safety
///
/// - `fd` must be a valid Unix file descriptor with read access.
///
/// - `fd` will not be locked by this operation, it is up to the runtime to ensure that access to
/// the file is properly synchronized within the process(es).
pub(crate) unsafe fn read_all(
    fd: NSTDUnixFileDescriptor,
    buffer: &mut NSTDVec,
) -> NSTDUnixIOResult {
    /// The default buffer size for piped/FIFO/socket file objects.
    const PIPE_BUF_SIZE: NSTDUInt = 32;
    // Make sure the buffer's element size is 1.
    if nstd_vec_stride(buffer) != 1 {
        return NSTDResult::Err(NSTD_UNIX_IO_ERROR_INVALID_INPUT);
    }
    // Get the number of bytes remaining in the file.
    let (mut buf_size, is_piped) = match lseek(fd, 0, SEEK_CUR) {
        -1 => match NSTDUnixIOError::last() {
            // The file is piped and cannot be used with `lseek`. Give it a default buffer size.
            NSTD_UNIX_IO_ERROR_INVALID_SEEK => (PIPE_BUF_SIZE, true),
            err => return NSTDResult::Err(err),
        },
        offset => match lseek(fd, 0, SEEK_END) {
            -1 => return NSTDResult::Err(NSTDUnixIOError::last()),
            size => match lseek(fd, offset, SEEK_SET) {
                -1 => return NSTDResult::Err(NSTDUnixIOError::last()),
                _ => ((size - offset) as _, false),
            },
        },
    };
    // Check `buf_size`.
    if buf_size > ISIZE_MAX {
        return NSTDResult::Err(NSTD_UNIX_IO_ERROR_INVALID_INPUT);
    }
    // Read data into the vector.
    let start_len = nstd_vec_len(buffer);
    loop {
        let len = nstd_vec_len(buffer);
        // Reserve extra space for the vector if the file is piped or there have not been any reads
        // yet.
        if is_piped || start_len == len {
            let reserved = nstd_vec_cap(buffer) - len;
            if reserved < buf_size
                && nstd_vec_reserve(buffer, buf_size - reserved) != NSTD_ALLOC_ERROR_NONE
            {
                return NSTDResult::Err(NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY);
            }
        }
        // Attempt to fill the rest of the vector's reserved buffer.
        match libc::read(fd, nstd_vec_end_mut(buffer), buf_size) {
            -1 => match NSTDUnixIOError::last() {
                NSTD_UNIX_IO_ERROR_INTERRUPTED => (),
                err => return NSTDResult::Err(err),
            },
            0 => return NSTDResult::Ok(len - start_len),
            r => {
                let read = r as NSTDUInt;
                nstd_vec_set_len(buffer, len + read);
                // If this is a non-piped file, make sure we don't read past the end of the file
                // next iteration.
                if !is_piped {
                    buf_size -= read;
                }
            }
        }
    }
}

/// Extends a UTF-8 encoded string with data from a Unix file until the end of the file is reached.
///
/// If an error occurs, `buffer` is left unchanged.
///
/// This will return an error variant of `NSTD_UNIX_IO_ERROR_INVALID_INPUT` in an attempt to read
/// more than `NSTDInt::MAX` bytes.
///
/// If extending the buffer fails, an error code of `NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY` will be
/// returned. This does not mean there were no bytes read from `stream` in this case.
///
/// # Panics
///
/// This operation will panic if `buffer`'s length in bytes ends up exceeding `NSTDInt::MAX`.
///
/// # Safety
///
/// - `fd` must be a valid Unix file descriptor with read access.
///
/// - `fd` will not be locked by this operation, it is up to the runtime to ensure that access to
/// the file is properly synchronized within the process(es).
pub(crate) unsafe fn read_to_string(
    fd: NSTDUnixFileDescriptor,
    buffer: &mut NSTDString,
) -> NSTDUnixIOResult {
    // Read data from the file.
    let buf = buffer.as_mut_vec();
    let start_len = nstd_vec_len(buf);
    let mut res = read_all(fd, buf);
    let read = nstd_vec_len(buf) - start_len;
    // Make sure the successfully read data is valid UTF-8.
    let read_start = nstd_vec_end(buf).sub(read) as _;
    let bytes = core::slice::from_raw_parts(read_start, read);
    if core::str::from_utf8(bytes).is_err() {
        let len = nstd_vec_len(buf);
        nstd_vec_set_len(buf, len - read);
        res = NSTDResult::Err(NSTD_UNIX_IO_ERROR_INVALID_DATA);
    }
    res
}

/// Reads enough data from a Unix file to fill the entirety of `buffer`.
///
/// # Safety
///
/// - `fd` must be a valid Unix file descriptor with read access.
///
/// - `fd` will not be locked by this operation, it is up to the runtime to ensure that access to
/// the file is properly synchronized within the process(es).
///
/// `buffer`'s data must be valid for writes.
pub(crate) unsafe fn read_exact(
    fd: NSTDUnixFileDescriptor,
    buffer: &mut NSTDSliceMut,
) -> NSTDUnixIOError {
    // Make sure the buffer's element size is 1.
    let len = nstd_core_slice_mut_len(buffer);
    if nstd_core_slice_mut_stride(buffer) != 1 || len > ISIZE_MAX {
        return NSTD_UNIX_IO_ERROR_INVALID_INPUT;
    }
    // Attempt to fill `buffer`.
    let mut read = 0;
    let mut pos = nstd_core_slice_mut_as_ptr(buffer);
    while read < len {
        match libc::read(fd, pos, len - read) {
            -1 => match NSTDUnixIOError::last() {
                NSTD_UNIX_IO_ERROR_INTERRUPTED => (),
                err => return err,
            },
            0 => return NSTD_UNIX_IO_ERROR_UNEXPECTED_EOF,
            r => {
                read += r as NSTDUInt;
                pos = pos.offset(r);
            }
        }
    }
    NSTD_UNIX_IO_ERROR_NONE
}
