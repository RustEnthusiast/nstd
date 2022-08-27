//! Multi-format image processing.
use crate::{
    core::slice::{nstd_core_slice_new, NSTDSlice},
    io::NSTDIOError,
    NSTDUInt32,
};
use image::{error::ImageError, DynamicImage};

/// An image of any format.
pub type NSTDImage = Box<DynamicImage>;

/// An enumeration that describes each type of `nstd.image` error.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDImageErrorType {
    /// An unknown image operation error.
    NSTD_IMAGE_ERROR_TYPE_UNKNOWN,
    /// An I/O operation failed.
    NSTD_IMAGE_ERROR_TYPE_IO,
}

/// A union of each possible error from `nstd.image`.
#[repr(C)]
pub union NSTDImageErrorData {
    /// No data.
    empty: (),
    /// I/O operation error data.
    pub io_data: NSTDIOError,
}

/// An error returned from an `nstd.image` function.
#[repr(C)]
pub struct NSTDImageError {
    /// Indicates the type of error that has occurred.
    pub errc: NSTDImageErrorType,
    /// A union of each possible error.
    pub err: NSTDImageErrorData,
}
impl NSTDImageError {
    /// Creates an instance of [NSTDImageError] from an [ImageError].
    fn from_err(err: ImageError) -> Self {
        match err {
            // An I/O operation failed.
            ImageError::IoError(err) => Self {
                errc: NSTDImageErrorType::NSTD_IMAGE_ERROR_TYPE_IO,
                err: NSTDImageErrorData {
                    io_data: NSTDIOError::from_err(err.kind()),
                },
            },
            // Another error occurred.
            _ => Self {
                errc: NSTDImageErrorType::NSTD_IMAGE_ERROR_TYPE_UNKNOWN,
                err: NSTDImageErrorData { empty: () },
            },
        }
    }
}

/// Loads an image from an in-memory buffer.
///
/// # Parameters:
///
/// - `const NSTDSlice *buffer` - The image buffer.
///
/// - `NSTDImageError *errc` - Returns as the image operation's error code.
///
/// # Returns
///
/// `NSTDImage img` - The new image, or null on error.
///
/// # Panics
///
/// This operation will panic if `buffer`'s stride is not 1.
///
/// # Safety
///
/// This operation can cause undefined behavior if `buffer`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_load(
    buffer: &NSTDSlice,
    errc: &mut NSTDImageError,
) -> Option<NSTDImage> {
    match image::load_from_memory(buffer.as_slice()) {
        Ok(img) => Some(Box::new(img)),
        Err(err) => {
            *errc = NSTDImageError::from_err(err);
            None
        }
    }
}

/// Returns an image's raw pixel data as a byte slice.
///
/// # Parameters:
///
/// - `const NSTDImage *img` - The image.
///
/// # Returns
///
/// `NSTDSlice bytes` - The image's raw pixel data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_image_as_bytes(img: &NSTDImage) -> NSTDSlice {
    let bytes = img.as_bytes();
    nstd_core_slice_new(bytes.as_ptr().cast(), 1, bytes.len())
}

/// Returns the width of an image.
///
/// # Parameters:
///
/// - `const NSTDImage *img` - The image.
///
/// # Returns
///
/// `NSTDUInt32 width` - The width of the image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_image_width(img: &NSTDImage) -> NSTDUInt32 {
    img.width()
}

/// Returns the height of an image.
///
/// # Parameters:
///
/// - `const NSTDImage *img` - The image.
///
/// # Returns
///
/// `NSTDUInt32 height` - The height of the image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_image_height(img: &NSTDImage) -> NSTDUInt32 {
    img.height()
}

/// Frees image data.
///
/// # Parameters:
///
/// - `NSTDImage img` - The image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_image_free(img: NSTDImage) {}
