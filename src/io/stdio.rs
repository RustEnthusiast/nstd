//! Contains common I/O operations for [Write] with `nstd` types.
use crate::{core::slice::NSTDSliceConst, io::NSTDIOError, NSTDUSize};
use std::io::Write;

/// Writes some of an `nstd` byte slice to a [Write] stream.
///
/// The return values are the number of bytes written to the stream and the I/O operation error
/// code respectively.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
pub(crate) unsafe fn write<W: Write>(
    stream: &mut W,
    bytes: &NSTDSliceConst,
) -> (NSTDUSize, NSTDIOError) {
    // Make sure the slice's element size is 1.
    if bytes.ptr.size != 1 {
        return (0, NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT);
    }
    // Attempt to write the bytes to stdout.
    match stream.write(bytes.as_slice()) {
        Ok(w) => (w, NSTDIOError::NSTD_IO_ERROR_NONE),
        Err(err) => (0, NSTDIOError::from_err(err.kind())),
    }
}

/// Writes an `nstd` byte slice to a [Write] stream.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
pub(crate) unsafe fn write_all<W: Write>(stream: &mut W, bytes: &NSTDSliceConst) -> NSTDIOError {
    // Make sure the slice's element size is 1.
    if bytes.ptr.size != 1 {
        return NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT;
    }
    // Attempt to write the bytes to stdout.
    if let Err(err) = stream.write_all(bytes.as_slice()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Flushes a [Write] stream.
#[inline]
pub(crate) fn flush<W: Write>(stream: &mut W) -> NSTDIOError {
    if let Err(err) = stream.flush() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}
