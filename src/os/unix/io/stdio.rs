//! Provides common I/O operations for working with Unix file descriptors.
use super::{
    NSTDUnixFileDescriptor,
    NSTDUnixIOError::{self, *},
};
use crate::{
    alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
    core::slice::{
        nstd_core_slice_as_ptr, nstd_core_slice_len, nstd_core_slice_mut_as_ptr,
        nstd_core_slice_mut_len, nstd_core_slice_mut_stride, nstd_core_slice_stride, NSTDSlice,
        NSTDSliceMut,
    },
    string::NSTDString,
    vec::{
        nstd_vec_cap, nstd_vec_end, nstd_vec_end_mut, nstd_vec_len, nstd_vec_reserve,
        nstd_vec_set_len, nstd_vec_stride, NSTDVec,
    },
    NSTDUInt,
};
use libc::{lseek, SEEK_CUR, SEEK_END};

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
pub(crate) unsafe fn write(
    fd: NSTDUnixFileDescriptor,
    bytes: &NSTDSlice,
) -> (NSTDUnixIOError, NSTDUInt) {
    // Check if `len` is 0.
    let len = nstd_core_slice_len(bytes);
    if len == 0 {
        return (NSTD_UNIX_IO_ERROR_NONE, 0);
    }
    // Make sure the slice's element size is 1.
    if nstd_core_slice_stride(bytes) != 1 || len > ISIZE_MAX {
        return (NSTD_UNIX_IO_ERROR_INVALID_INPUT, 0);
    }
    // Write the data.
    match libc::write(fd, nstd_core_slice_as_ptr(bytes), len) {
        -1 => (NSTDUnixIOError::last(), 0),
        w => (NSTD_UNIX_IO_ERROR_NONE, w as _),
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
    // Check if `len` is 0.
    let len = nstd_core_slice_len(bytes);
    if len == 0 {
        return NSTD_UNIX_IO_ERROR_NONE;
    }
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
) -> (NSTDUnixIOError, NSTDUInt) {
    // Make sure the buffer's element size is 1.
    let len = nstd_core_slice_mut_len(buffer);
    if nstd_core_slice_mut_stride(buffer) != 1 || len > ISIZE_MAX {
        return (NSTD_UNIX_IO_ERROR_INVALID_INPUT, 0);
    }
    // Read data into `buffer`.
    match libc::read(fd, nstd_core_slice_mut_as_ptr(buffer), len) {
        -1 => (NSTDUnixIOError::last(), 0),
        r => (NSTD_UNIX_IO_ERROR_NONE, r as _),
    }
}

/// Extends a vector with data from a Unix file until the end of the file is reached.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `buffer`'s length in bytes ends up exceeding `NSTDInt::MAX`.
///
/// - An attempt was made to read more than `NSTDInt::MAX` bytes.
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
) -> (NSTDUnixIOError, NSTDUInt) {
    /// The default buffer size for piped/FIFO/socket file objects.
    const PIPE_BUF_SIZE: NSTDUInt = 32;
    // Make sure the buffer's element size is 1.
    if nstd_vec_stride(buffer) != 1 {
        return (NSTD_UNIX_IO_ERROR_INVALID_INPUT, 0);
    }
    // Get the file size.
    let (mut buf_size, is_piped) = match lseek(fd, 0, SEEK_END) {
        -1 => match NSTDUnixIOError::last() {
            // The file is piped and cannot be used with `lseek`. Give it a default buffer size.
            NSTD_UNIX_IO_ERROR_INVALID_SEEK => (PIPE_BUF_SIZE, true),
            err => return (err, 0),
        },
        size => match lseek(fd, -size, SEEK_CUR) {
            -1 => return (NSTDUnixIOError::last(), 0),
            _ => (size as _, false),
        },
    };
    assert!(buf_size <= ISIZE_MAX);
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
                return (NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY, 0);
            }
        }
        // Attempt to fill the rest of the vector's reserved buffer.
        match libc::read(fd, nstd_vec_end_mut(buffer), buf_size) {
            -1 => match NSTDUnixIOError::last() {
                NSTD_UNIX_IO_ERROR_INTERRUPTED => (),
                err => return (err, len - start_len),
            },
            0 => return (NSTD_UNIX_IO_ERROR_NONE, len - start_len),
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
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `buffer`'s length in bytes ends up exceeding `NSTDInt::MAX`.
///
/// - An attempt was made to read more than `NSTDInt::MAX` bytes.
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
) -> (NSTDUnixIOError, NSTDUInt) {
    // Read data from the file.
    let buf = buffer.as_mut_vec();
    let (mut err, read) = read_all(fd, buf);
    // Make sure the successfully read data is valid UTF-8.
    let read_start = nstd_vec_end(buf).sub(read) as _;
    let bytes = core::slice::from_raw_parts(read_start, read);
    if let Err(_) = core::str::from_utf8(&bytes) {
        let len = nstd_vec_len(buf);
        nstd_vec_set_len(buf, len - read);
        err = NSTD_UNIX_IO_ERROR_INVALID_DATA;
    }
    (err, read)
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
