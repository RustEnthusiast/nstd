# NSTD
A cross-platform, fast, and safe general purpose C library written in Rust.

The library is organized as a series of modules. The top level module `nstd` encompasses the entire
crate. Each module can have their own submodules (eg. `nstd.core.def` or `nstd::core::def` with
Rust syntax).

# Platform support
`nstd.core` supports just about everything as it doesn't require the use of Rust's `std` crate.

`nstd.os`'s child modules will only work on the operating system they target. For example,
`nstd.os.windows` will only work on Windows and `nstd.os.linux` will only work on Linux
distributions.

Other modules will work on most platforms, primarily targeting Windows, macOS,
Linux, Android, and iOS.

# Safety notes
Because `nstd` is a C library, accessing pointer types such as `NSTDPtr` or `NSTDSlice` is
unsafe because the data being referenced is never guaranteed to be valid. Similarly, pointer
types allow mutability accross multiple instances (it is legal to have multiple `NSTDPtr`s mutate
the same value within the same scope).

# Modules
- `nstd` - A cross-platform, fast, and safe general purpose C library written in Rust.
    - `alloc` - Low level memory allocation.
    - `core` - The central and most important part of `nstd`.
        - `cstr` - C string processing.
        - `cty` - Provides functions for examining and operating on character types.
        - `def` - Contains common types used throughout `nstd`.
        - `fty` - Provides functions for examining and operating on floating point types.
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
cargo build --release --features "clib nstd_alloc nstd_vec"
```

To build with all features:
```
cargo build --release --all-features
```
