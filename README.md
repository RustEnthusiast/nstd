# NSTD
A cross-platform, fast, and safe general purpose C library written in Rust.

The library is organized as a series of modules. The top level module `nstd` encompasses the entire
crate. Each module can have their own submodules (eg. `nstd.core.def` or `nstd::core::def` with
Rust syntax).

# Modules
- `nstd` - A cross-platform, fast, and safe general purpose C library written in Rust.
    - `alloc` - Low level memory allocation.
    - `core` - The central and most important part of `nstd`.
        - `cstr` - C string processing.
        - `def` - Contains common types used throughout `nstd`.
        - `ity` - Provides functions for examining and operating on integral types.
        - `math` - Low level math operations.
        - `mem` - Contains mostly unsafe functions for interacting with raw memory.
        - `ptr` - A sized pointer to some arbitrary type.
        - `slice` - A view into a sequence of values in memory.
        - `str` - An unowned view into a UTF-8 encoded byte string.
    - `os` - Operating system specific functionality.
        - `windows` - OS support for Windows.
            - `alloc` - Low level memory allocation for Windows.
    - `vec` - A dynamically sized contiguous sequence of values.

# How to build
`nstd` let's you decide what features you want to use. Any module that falls under the top level
module has a dedicated feature flag, for example `nstd.core` has the feature flag `nstd_core` and
`nstd.alloc` has the feature flag `nstd_alloc`. To build `nstd` as a C library, use the `clib`
feature flag. The `std` feature flag enables Rust standard library support, all modules other than
`nstd.core` require this flag. `std` and `nstd_core` are enabled by default.

For example:
```
cargo build --features "clib nstd_alloc nstd_vec"
```
