//! Multi-format image processing.
use crate::{core::slice::NSTDSlice, NSTDUInt32};
use image::DynamicImage;
use nstdapi::nstdapi;

/// An image of any format.
pub type NSTDImage = Box<DynamicImage>;

/// Loads an image from an in-memory buffer.
///
/// # Parameters:
///
/// - `const NSTDSlice *buffer` - The image buffer.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_image_load(buffer: &NSTDSlice) -> Option<NSTDImage> {
    match image::load_from_memory(buffer.as_slice()) {
        Ok(img) => Some(Box::new(img)),
        _ => None,
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
#[nstdapi]
pub fn nstd_image_as_bytes(img: &NSTDImage) -> NSTDSlice {
    NSTDSlice::from_slice(img.as_bytes())
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
#[nstdapi]
pub fn nstd_image_width(img: &NSTDImage) -> NSTDUInt32 {
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
#[nstdapi]
pub fn nstd_image_height(img: &NSTDImage) -> NSTDUInt32 {
    img.height()
}

/// Frees image data.
///
/// # Parameters:
///
/// - `NSTDImage img` - The image.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_image_free(img: NSTDImage) {}
