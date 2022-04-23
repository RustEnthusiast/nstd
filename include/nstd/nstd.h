#ifndef NSTD_NSTD_H_INCLUDED
#define NSTD_NSTD_H_INCLUDED
#if defined(__WINDOWS__)\
    || defined(_WIN32)\
    || defined(_WIN64)\
    || defined(__WIN32__)\
    || defined(__TOS_WIN__)
#define NSTDAPI __declspec(dllexport)
#else
#define NSTDAPI
#endif
#ifdef __cplusplus
#define NSTDCPP __cplusplus
#define NSTDCPPSTART extern "C" {
#define NSTDCPPEND }
#else
#define NSTDCPPSTART
#define NSTDCPPEND
#endif
#endif
