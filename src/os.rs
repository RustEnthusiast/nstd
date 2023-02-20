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

/// Constant that is only set if the target operating system is iOS.
#[cfg(target_os = "ios")]
pub const NSTD_OS_IOS: () = ();

/// Constant that is only set if the target operating system is Android.
#[cfg(target_os = "android")]
pub const NSTD_OS_ANDROID: () = ();

/// Constant that is only set if the target operating system is DragonFly BSD.
#[cfg(target_os = "dragonfly")]
pub const NSTD_OS_DRAGONFLY: () = ();

/// Constant that is only set if the target operating system is FreeBSD.
#[cfg(target_os = "freebsd")]
pub const NSTD_OS_FREEBSD: () = ();

/// Constant that is only set if the target operating system is NetBSD.
#[cfg(target_os = "netbsd")]
pub const NSTD_OS_NETBSD: () = ();

/// Constant that is only set if the target operating system is OpenBSD.
#[cfg(target_os = "openbsd")]
pub const NSTD_OS_OPENBSD: () = ();

/// Constant that is only set if the target operating system is BSD based.
#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub const NSTD_OS_BSD: () = ();

/// Constant that is only set if the target operating system is Haiku.
#[cfg(target_os = "haiku")]
pub const NSTD_OS_HAIKU: () = ();

/// Constant that is only set if the target operating system is QNX Neutrino.
#[cfg(target_os = "nto")]
pub const NSTD_OS_NTO: () = ();

/// Constant that is only set if the target operating system is Solaris.
#[cfg(target_os = "solaris")]
pub const NSTD_OS_SOLARIS: () = ();

/// Constant that is only set if the target operating system is Unix based.
#[cfg(unix)]
pub const NSTD_OS_UNIX: () = ();
