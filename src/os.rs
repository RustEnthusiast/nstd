//! Operating system specific functionality.
#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

/// Constant that is only set if the target operating system is Linux.
#[cfg(target_os = "linux")]
pub const NSTD_OS_LINUX: () = ();

/// Constant that is only set if the target operating system is macOS.
#[cfg(target_os = "macos")]
pub const NSTD_OS_MACOS: () = ();

/// Constant that is only set if the target operating system is Windows.
#[cfg(windows)]
pub const NSTD_OS_WINDOWS: () = ();

/// Constant that is only set if the target operating system is Unix based.
#[cfg(unix)]
pub const NSTD_OS_UNIX: () = ();
