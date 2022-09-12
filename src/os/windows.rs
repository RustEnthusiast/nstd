//! OS support for Windows.
#[cfg(feature = "nstd_os_windows_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_windows_alloc")))]
pub mod alloc;

/// Constant that is only set if the target operating system is Windows.
pub const NSTD_OS_WINDOWS: () = ();
