//! A handle to an opened file.
use crate::{core::str::NSTDStrConst, io::NSTDIOError, NSTDUInt8};
use std::fs::File;

/// Creates the file upon opening if it does not already exist.
///
/// Either of the `NSTD_FILE_WRITE` or `NSTD_FILE_APPEND` options must also be toggled for the file
/// to be created.
pub const NSTD_FILE_CREATE: NSTDUInt8 = 0b00000001;

/// Open a file in read mode.
pub const NSTD_FILE_READ: NSTDUInt8 = 0b00000010;

/// Open a file in write mode.
pub const NSTD_FILE_WRITE: NSTDUInt8 = 0b00000100;

/// Open a file in writing mode without overwriting saved data.
pub const NSTD_FILE_APPEND: NSTDUInt8 = 0b00001000;

/// Open a file in truncate mode, this will set the file's length to 0 upon openning.
pub const NSTD_FILE_TRUNC: NSTDUInt8 = 0b00010000;

/// A handle to an opened file.
pub type NSTDFile = Box<File>;

/// Opens file on the filesystem and returns a handle to it.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - The name of the file to create.
///
/// - `NSTDUInt8 mask` - A bit mask for toggling the file's different open options.
///
/// - `NSTDIOError *errc` - Returns as the I/O operation error code.
///
/// # Returns
///
/// `NSTDFile file` - A handle to the opened file, or null if an error occurrs.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_file_open(
    name: &NSTDStrConst,
    mask: NSTDUInt8,
    errc: &mut NSTDIOError,
) -> Option<NSTDFile> {
    // Attempt to create/open the file in write mode.
    match File::options()
        .create((mask & NSTD_FILE_CREATE) != 0)
        .read((mask & NSTD_FILE_READ) != 0)
        .write((mask & NSTD_FILE_WRITE) != 0)
        .append((mask & NSTD_FILE_APPEND) != 0)
        .truncate((mask & NSTD_FILE_TRUNC) != 0)
        .open(name.as_str())
    {
        Ok(f) => {
            *errc = NSTDIOError::NSTD_IO_ERROR_NONE;
            Some(Box::new(f))
        }
        Err(err) => {
            *errc = NSTDIOError::from_err(err.kind());
            None
        }
    }
}

/// Closes a file handle.
///
/// # Parameters:
///
/// - `NSTDFile file` - The file handle to close.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_fs_file_close(file: NSTDFile) {}
