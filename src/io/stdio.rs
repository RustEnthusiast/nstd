//! Contains common I/O operations for [Read] & [Write] with `nstd` types.
#![allow(dead_code)]
use crate::{
    alloc::NSTDAllocError,
    core::{
        result::NSTDResult,
        slice::{NSTDSlice, NSTDSliceMut},
        str::nstd_core_str_from_bytes_unchecked,
    },
    io::{NSTDIOError, NSTDIOResult},
    string::{nstd_string_push_str, NSTDString},
    vec::{nstd_vec_extend, nstd_vec_stride, NSTDVec},
};
use std::io::{Read, Write};

/// Writes some `nstd` bytes to a [Write] stream.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
pub(crate) unsafe fn write<W: Write>(stream: &mut W, bytes: &NSTDSlice) -> NSTDIOResult {
    bytes.as_slice().map_or(
        NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT),
        |bytes| match stream.write(bytes) {
            Ok(w) => NSTDResult::Ok(w),
            Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
        },
    )
}

/// Writes an `nstd` byte slice to a [Write] stream.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
pub(crate) unsafe fn write_all<W: Write>(stream: &mut W, bytes: &NSTDSlice) -> NSTDIOError {
    bytes.as_slice().map_or(
        NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT,
        |bytes| match stream.write_all(bytes) {
            Ok(_) => NSTDIOError::NSTD_IO_ERROR_NONE,
            Err(err) => NSTDIOError::from_err(err.kind()),
        },
    )
}

/// Flushes a [Write] stream.
#[inline]
pub(crate) fn flush<W: Write>(stream: &mut W) -> NSTDIOError {
    if let Err(err) = stream.flush() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Reads some data from a [Read] stream into an `nstd` byte slice.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
pub(crate) unsafe fn read<R: Read>(stream: &mut R, buffer: &mut NSTDSliceMut) -> NSTDIOResult {
    buffer.as_slice_mut().map_or(
        NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT),
        |buffer| match stream.read(buffer) {
            Ok(r) => NSTDResult::Ok(r),
            Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
        },
    )
}

/// Extends an [`NSTDVec`] with data from a [Read] stream until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `stream` in this case.
pub(crate) fn read_all<R: Read>(stream: &mut R, buffer: &mut NSTDVec<'_>) -> NSTDIOResult {
    // Make sure the buffer's element size is 1.
    #[allow(unused_unsafe)]
    // SAFETY: This operation is safe.
    if unsafe { nstd_vec_stride(buffer) } != 1 {
        return NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT);
    }
    // Attempt to read data into `buffer`.
    let mut buf = Vec::new();
    match stream.read_to_end(&mut buf) {
        Ok(r) => {
            let bytes = NSTDSlice::from_slice(buf.as_slice());
            // SAFETY: `bytes` refers to `buf`'s data, which is still valid here.
            match unsafe { nstd_vec_extend(buffer, &bytes) } {
                NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDResult::Ok(r),
                _ => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY),
            }
        }
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Extends an [`NSTDString`] with UTF-8 data from a [Read] stream until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `stream` in this case.
pub(crate) fn read_to_string<R: Read>(stream: &mut R, buffer: &mut NSTDString<'_>) -> NSTDIOResult {
    // Attempt to read data into `buffer`.
    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(r) => {
            let bytes = NSTDSlice::from_slice(buf.as_bytes());
            // SAFETY: `bytes` refers to `buf`'s data, which is still valid UTF-8 here.
            unsafe {
                let str = nstd_core_str_from_bytes_unchecked(&bytes);
                match nstd_string_push_str(buffer, &str) {
                    NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDResult::Ok(r),
                    _ => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY),
                }
            }
        }
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Reads enough data from stdin to fill the entirety of `buffer`.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
pub(crate) unsafe fn read_exact<R: Read>(stream: &mut R, buffer: &mut NSTDSliceMut) -> NSTDIOError {
    buffer.as_slice_mut().map_or(
        NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT,
        |buffer| match stream.read_exact(buffer) {
            Ok(_) => NSTDIOError::NSTD_IO_ERROR_NONE,
            Err(err) => NSTDIOError::from_err(err.kind()),
        },
    )
}
