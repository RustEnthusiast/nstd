//! OS support for Windows.
#[cfg(feature = "nstd_os_windows_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_windows_alloc")))]
pub mod alloc;
#[cfg(feature = "nstd_os_windows_shared_lib")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_windows_shared_lib")))]
pub mod shared_lib;
#[cfg(feature = "nstd_os_windows_str")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_windows_str")))]
pub mod str;
use crate::NSTDInt;

/// A raw handle to a resource managed by the Windows kernel.
pub type NSTDWindowsHandle = NSTDInt;
