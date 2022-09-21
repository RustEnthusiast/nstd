//! Operating system specific functionality.
#[cfg(target_os = "windows")]
pub mod windows;

/// Constant that is only set if the target operating system is Linux.
#[cfg(target_os = "linux")]
pub const NSTD_OS_LINUX: () = ();

/// Constant that is only set if the target operating system is Windows.
#[cfg(target_os = "windows")]
pub const NSTD_OS_WINDOWS: () = ();
