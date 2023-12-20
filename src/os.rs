//! Operating system specific functionality.
#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

/// Constant that is only set if the target operating system is Linux.
#[cfg(target_os = "linux")]
pub const NSTD_OS_LINUX: u8 = 1;
/// Constant that is only set if the target operating system is Linux.
#[cfg(not(target_os = "linux"))]
pub const NSTD_OS_LINUX: u8 = 0;

/// Constant that is only set if the target operating system is macOS.
#[cfg(target_os = "macos")]
pub const NSTD_OS_MACOS: u8 = 1;
/// Constant that is only set if the target operating system is macOS.
#[cfg(not(target_os = "macos"))]
pub const NSTD_OS_MACOS: u8 = 0;

/// Constant that is only set if the target operating system is Windows.
#[cfg(windows)]
pub const NSTD_OS_WINDOWS: u8 = 1;
/// Constant that is only set if the target operating system is Windows.
#[cfg(not(windows))]
pub const NSTD_OS_WINDOWS: u8 = 0;

/// Constant that is only set if the target operating system is iOS.
#[cfg(target_os = "ios")]
pub const NSTD_OS_IOS: u8 = 1;
/// Constant that is only set if the target operating system is iOS.
#[cfg(not(target_os = "ios"))]
pub const NSTD_OS_IOS: u8 = 0;

/// Constant that is only set if the target operating system is Android.
#[cfg(target_os = "android")]
pub const NSTD_OS_ANDROID: u8 = 1;
/// Constant that is only set if the target operating system is Android.
#[cfg(not(target_os = "android"))]
pub const NSTD_OS_ANDROID: u8 = 0;

/// Constant that is only set if the target operating system is DragonFly BSD.
#[cfg(target_os = "dragonfly")]
#[allow(clippy::doc_markdown)]
pub const NSTD_OS_DRAGONFLY: u8 = 1;
/// Constant that is only set if the target operating system is DragonFly BSD.
#[cfg(not(target_os = "dragonfly"))]
#[allow(clippy::doc_markdown)]
pub const NSTD_OS_DRAGONFLY: u8 = 0;

/// Constant that is only set if the target operating system is FreeBSD.
#[cfg(target_os = "freebsd")]
pub const NSTD_OS_FREEBSD: u8 = 1;
/// Constant that is only set if the target operating system is FreeBSD.
#[cfg(not(target_os = "freebsd"))]
pub const NSTD_OS_FREEBSD: u8 = 0;

/// Constant that is only set if the target operating system is NetBSD.
#[cfg(target_os = "netbsd")]
#[allow(clippy::doc_markdown)]
pub const NSTD_OS_NETBSD: u8 = 1;
/// Constant that is only set if the target operating system is NetBSD.
#[cfg(not(target_os = "netbsd"))]
#[allow(clippy::doc_markdown)]
pub const NSTD_OS_NETBSD: u8 = 0;

/// Constant that is only set if the target operating system is OpenBSD.
#[cfg(target_os = "openbsd")]
#[allow(clippy::doc_markdown)]
pub const NSTD_OS_OPENBSD: u8 = 1;
/// Constant that is only set if the target operating system is OpenBSD.
#[cfg(not(target_os = "openbsd"))]
#[allow(clippy::doc_markdown)]
pub const NSTD_OS_OPENBSD: u8 = 0;

/// Constant that is only set if the target operating system is BSD based.
#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub const NSTD_OS_BSD: u8 = 1;
/// Constant that is only set if the target operating system is BSD based.
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
)))]
pub const NSTD_OS_BSD: u8 = 0;

/// Constant that is only set if the target operating system is Haiku.
#[cfg(target_os = "haiku")]
pub const NSTD_OS_HAIKU: u8 = 1;
/// Constant that is only set if the target operating system is Haiku.
#[cfg(not(target_os = "haiku"))]
pub const NSTD_OS_HAIKU: u8 = 0;

/// Constant that is only set if the target operating system is QNX Neutrino.
#[cfg(target_os = "nto")]
pub const NSTD_OS_NTO: u8 = 1;
/// Constant that is only set if the target operating system is QNX Neutrino.
#[cfg(not(target_os = "nto"))]
pub const NSTD_OS_NTO: u8 = 0;

/// Constant that is only set if the target operating system is Solaris.
#[cfg(target_os = "solaris")]
pub const NSTD_OS_SOLARIS: u8 = 1;
/// Constant that is only set if the target operating system is Solaris.
#[cfg(not(target_os = "solaris"))]
pub const NSTD_OS_SOLARIS: u8 = 0;

/// Constant that is only set if the target operating system is Unix based.
#[cfg(unix)]
pub const NSTD_OS_UNIX: u8 = 1;
/// Constant that is only set if the target operating system is Unix based.
#[cfg(not(unix))]
pub const NSTD_OS_UNIX: u8 = 0;
