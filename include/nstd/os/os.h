#ifndef NSTD_OS_OS_H
#define NSTD_OS_OS_H

/// Constant that is only set if the target operating system is Linux.
#if defined(linux) || defined(__linux) || defined(__linux__) || defined(__gnu_linux__)
#    define NSTD_OS_LINUX
#endif

/// Constant that is only set if the target operating system is macOS.
#if defined(TARGET_OS_MAC)
#    define NSTD_OS_MACOS
#endif

/// Constant that is only set if the target operating system is Windows.
#if defined(__WINDOWS__) || defined(_WIN16) || defined(_WIN32) || defined(_WIN64) \
    || defined(__WIN32__) || defined(__TOS_WIN__)
#    define NSTD_OS_WINDOWS
#endif

/// Constant that is only set if the target operating system is iOS.
#if defined(TARGET_OS_IPHONE) || defined(TARGET_IPHONE_SIMULATOR)
#    define NSTD_OS_IOS
#endif

/// Constant that is only set if the target operating system is Android.
#if defined(__ANDROID__)
#    define NSTD_OS_ANDROID
#endif

/// Constant that is only set if the target operating system is DragonFly BSD.
#if defined(__DragonFly__)
#    define NSTD_OS_DRAGONFLY
#endif

/// Constant that is only set if the target operating system is FreeBSD.
#if defined(__FreeBSD__)
#    define NSTD_OS_FREEBSD
#endif

/// Constant that is only set if the target operating system is NetBSD.
#if defined(__NETBSD__) || defined(__NetBSD__)
#    define NSTD_OS_NETBSD
#endif

/// Constant that is only set if the target operating system is OpenBSD.
#if defined(__OpenBSD__)
#    define NSTD_OS_OPENBSD
#endif

/// Constant that is only set if the target operating system is BSD based.
#if defined(NSTD_OS_DRAGONFLY) || defined(NSTD_OS_FREEBSD) || defined(NSTD_OS_NETBSD) \
    || defined(NSTD_OS_OPENBSD) || defined(BSD) || defined(_SYSTYPE_BSD)
#    define NSTD_OS_BSD
#endif

/// Constant that is only set if the target operating system is Haiku.
#if defined(__HAIKU__)
#    define NSTD_OS_HAIKU
#endif

/// Constant that is only set if the target operating system is QNX Neutrino.
#if defined(__QNX__) || defined(__QNXNTO__)
#    define NSTD_OS_NTO
#endif

/// Constant that is only set if the target operating system is Solaris.
#if defined(sun) || defined(__sun)
#    define NSTD_OS_SOLARIS
#endif

/// Constant that is only set if the target operating system is Unix based.
#if defined(NSTD_OS_LINUX) || defined(NSTD_OS_MACOS) || defined(NSTD_OS_IOS)                \
    || defined(NSTD_OS_ANDROID) || defined(NSTD_OS_BSD) || defined(NSTD_OS_HAIKU)           \
    || defined(NSTD_OS_NTO) || defined(NSTD_OS_SOLARIS) || defined(unix) || defined(__unix) \
    || defined(__unix__) || defined(_XOPEN_SOURCE) || defined(_POSIX_SOURCE)
#    define NSTD_OS_UNIX
#endif

#endif
