#ifndef NSTD_OS_OS_H
#define NSTD_OS_OS_H

/// Constant that is only set if the target operating system is Linux.
#if defined(linux)\
    || defined(__linux)\
    || defined(__linux__)\
    || defined(__gnu_linux__)
#define NSTD_OS_LINUX
#endif

/// Constant that is only set if the target operating system is macOS.
#if defined(TARGET_OS_MAC)
#define NSTD_OS_MACOS
#endif

/// Constant that is only set if the target operating system is Windows.
#if defined(__WINDOWS__)\
    || defined(_WIN16)\
    || defined(_WIN32)\
    || defined(_WIN64)\
    || defined(__WIN32__)\
    || defined(__TOS_WIN__)
#define NSTD_OS_WINDOWS
#endif

/// Constant that is only set if the target operating system is Unix based.
#if defined(NSTD_OS_LINUX)\
    || defined(NSTD_OS_MACOS)\
    || defined(unix)\
    || defined(__unix)\
    || defined(__unix__)\
    || defined(_XOPEN_SOURCE)\
    || defined(_POSIX_SOURCE)
#define NSTD_OS_UNIX
#endif

#endif
