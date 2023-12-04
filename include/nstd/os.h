#ifndef NSTD_OS_H
#define NSTD_OS_H
#include "os/os.h"
#if NSTD_OS_UNIX
#    include "os/unix.h"
#endif
#if NSTD_OS_WINDOWS
#    include "os/windows.h"
#endif
#endif
