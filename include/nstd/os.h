#ifndef NSTD_OS_H
#define NSTD_OS_H
#include "os/os.h"
#ifdef NSTD_OS_UNIX
#    include "os/unix.h"
#endif
#ifdef NSTD_OS_WINDOWS
#    include "os/windows.h"
#endif
#endif
