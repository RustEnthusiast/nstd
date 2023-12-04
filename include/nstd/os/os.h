#ifndef NSTD_OS_OS_H
#define NSTD_OS_OS_H
#ifdef __APPLE__
#    include <TargetConditionals.h>
#endif

/// Constant that is only set if the target operating system is Linux.
#if defined(linux) || defined(__linux) || defined(__linux__) || defined(__gnu_linux__)
#    define NSTD_OS_LINUX 1
#else
#    define NSTD_OS_LINUX 0
#endif

/// Constant that is only set if the target operating system is macOS.
#if defined(TARGET_OS_MAC)
#    define NSTD_OS_MACOS 1
#else
#    define NSTD_OS_MACOS 0
#endif

/// Constant that is only set if the target operating system is Windows.
#if defined(__WINDOWS__) || defined(_WIN16) || defined(_WIN32) || defined(_WIN64) \
    || defined(__WIN32__) || defined(__TOS_WIN__)
#    define NSTD_OS_WINDOWS 1
#else
#    define NSTD_OS_WINDOWS 0
#endif

/// Constant that is only set if the target operating system is iOS.
#if defined(TARGET_OS_IPHONE) || defined(TARGET_IPHONE_SIMULATOR)
#    define NSTD_OS_IOS 1
#else
#    define NSTD_OS_IOS 0
#endif

/// Constant that is only set if the target operating system is Android.
#if defined(__ANDROID__)
#    define NSTD_OS_ANDROID 1
#else
#    define NSTD_OS_ANDROID 0
#endif

/// Constant that is only set if the target operating system is DragonFly BSD.
#if defined(__DragonFly__)
#    define NSTD_OS_DRAGONFLY 1
#else
#    define NSTD_OS_DRAGONFLY 0
#endif

/// Constant that is only set if the target operating system is FreeBSD.
#if defined(__FreeBSD__)
#    define NSTD_OS_FREEBSD 1
#else
#    define NSTD_OS_FREEBSD 0
#endif

/// Constant that is only set if the target operating system is NetBSD.
#if defined(__NETBSD__) || defined(__NetBSD__)
#    define NSTD_OS_NETBSD 1
#else
#    define NSTD_OS_NETBSD 0
#endif

/// Constant that is only set if the target operating system is OpenBSD.
#if defined(__OpenBSD__)
#    define NSTD_OS_OPENBSD 1
#else
#    define NSTD_OS_OPENBSD 0
#endif

/// Constant that is only set if the target operating system is BSD based.
#if NSTD_OS_DRAGONFLY || NSTD_OS_FREEBSD || NSTD_OS_NETBSD || NSTD_OS_OPENBSD || defined(BSD) \
    || defined(_SYSTYPE_BSD)
#    define NSTD_OS_BSD 1
#else
#    define NSTD_OS_BSD 0
#endif

/// Constant that is only set if the target operating system is Haiku.
#if defined(__HAIKU__)
#    define NSTD_OS_HAIKU 1
#else
#    define NSTD_OS_HAIKU 0
#endif

/// Constant that is only set if the target operating system is QNX Neutrino.
#if defined(__QNX__) || defined(__QNXNTO__)
#    define NSTD_OS_NTO 1
#else
#    define NSTD_OS_NTO 0
#endif

/// Constant that is only set if the target operating system is Solaris.
#if defined(sun) || defined(__sun)
#    define NSTD_OS_SOLARIS 1
#else
#    define NSTD_OS_SOLARIS 0
#endif

/// Constant that is only set if the target operating system is Unix based.
#if NSTD_OS_LINUX || NSTD_OS_MACOS || NSTD_OS_IOS || NSTD_OS_ANDROID || NSTD_OS_BSD        \
    || NSTD_OS_HAIKU || NSTD_OS_NTO || NSTD_OS_SOLARIS || defined(unix) || defined(__unix) \
    || defined(__unix__) || defined(_XOPEN_SOURCE) || defined(_POSIX_SOURCE)
#    define NSTD_OS_UNIX 1
#else
#    define NSTD_OS_UNIX 0
#endif

#endif
