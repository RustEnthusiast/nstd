# NSTD
A cross-platform, fast, and safe general purpose C library written in Rust.

The library is organized as a series of modules. The top level module `nstd` encompasses the entire
crate. Each module can have their own submodules (eg. `nstd.io.stdout` or `nstd::io::stdout` with
Rust syntax).

# Example using C
```c
// Build nstd with `cargo build --features "clib nstd_core nstd_io"`.
#include <nstd.h>

/// Main entry point of the program.
int main()
{
    const NSTDStr output = nstd_core_str_from_raw_cstr("Hello, 🌎!");
    nstd_io_print_line(&output);
    return 0;
}
```

# Library modules
- `nstd` - A cross-platform, fast, and safe general purpose C library written in Rust.
    - `alloc` - Low level memory allocation.
    - `core` - Provides core functionality for `nstd`.
        - `cstr` - Unowned C string slices.
            - `raw` - Raw C string processing.
        - `cty` - Provides functions for examining and operating on character types.
        - `def` - Contains common types used throughout `nstd`.
        - `fty` - Provides functions for examining and operating on floating point types.
        - `ity` - Provides functions for examining and operating on integral types.
        - `math` - Low level math operations.
        - `mem` - Contains mostly unsafe functions for interacting with raw memory.
        - `ops` - Operator overloading for types and operators that may cause overflow.
        - `optional` - Represents an optional (possibly uninitialized) value.
        - `ptr` - A sized pointer to some arbitrary type.
            - `raw` - Provides useful utilities for working with raw pointers.
        - `range` - A numerical range.
        - `result` - Defines a "result" type with success and error variants.
        - `slice` - A view into a sequence of values in memory.
        - `str` - An unowned view into a UTF-8 encoded byte string.
        - `unichar` - A Unicode scalar value.
    - `cstring` - A dynamically sized, null terminated, C string.
    - `env` - Process environment management.
    - `fs` - Provides access to the file system.
        - `file` - A handle to an opened file.
    - `heap_ptr` - A pointer type for single value heap allocation.
    - `io` - Provides functionality for interacting with the standard I/O streams.
        - `stderr` - A handle to the standard error stream.
        - `stdin` - A handle to the standard input stream.
        - `stdout` - A handle to the standard output stream.
    - `math` - High level math operations.
    - `mutex` - A mutual exclusion primitive useful for protecting shared data.
    - `os` - Operating system specific functionality.
        - `unix` - Low level Unix-like operating system support.
            - `alloc` - Memory allocation for Unix-like systems.
            - `io` - Provides functionality for working with input & output on Unix platforms.
            - `shared_lib` - Provides shared library access for Unix-like systems.
        - `windows` - OS support for Windows.
            - `alloc` - Low level memory allocation for Windows.
                - `heap` - Process heap management for Windows.
            - `shared_lib` - Shared library/module access for Windows.
            - `str` - String slice extensions for Windows.
    - `proc` - Calling/Child process management.
    - `shared_lib` - Access symbols from loaded shared libraries.
    - `shared_ptr` - A reference counting smart pointer.
    - `string` - Dynamically sized UTF-8 encoded byte string.
    - `thread` - Thread spawning, joining, and detaching.
    - `vec` - A dynamically sized contiguous sequence of values.

# Platform support
`nstd.core` should support anything that rustc supports.

`nstd.os`'s child modules will only work on the operating system they target. For example,
`nstd.os.windows` will only work on Windows and `nstd.os.unix` will only work on Unix-like systems.

Other modules will work on most platforms, primarily targeting Windows, macOS,
Linux, Android, and iOS.

# Language support
This library can be accessed from any language that supports calling C code. As of now this will
need to be done manually as there are no official wrappers for the API, however somewhere around
version 0.11, the plan is to start adding official wrappers so developers from other languages
can easily use the API.

# Safety
*Please note that these safety notes (as well as the framework as a whole) are a work in progress.*

## User safety notes

- Raw pointers are unsafe to access.

- References are assumed to be valid (aligned, non-null, and non-dangling), and are safe to access.
Users can refer to the [docs](https://docs.rs/nstd-sys/latest/nstd_sys/) to see which APIs expect
or return valid references.

- Input reference data is assumed to remain unaltered by other code/threads.

- Private (non-`pub`) structure members must not be directly accessed by the user.

- Structured enum variants must be checked before they're accessed (eg. `NSTDOptional` or
`NSTDResult` types).

- Data is *moved* when using the value-copy semantic on a type that does not implement `Copy`.

- Types that do not implement the `Send` trait must not be sent between threads.

- Types that do not implement the `Sync` trait must not be shared between threads.

## Contributor safety notes

- Any operation that may cause
[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) must
be marked unsafe.

- All C function pointers taken as input by the API must be marked unsafe.

- The panic behavior is set to abort by default, as it is undefined behavior to unwind from Rust
code into foreign code (though this is
[subject to change](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html)).

# How to build
`nstd` lets you decide what features you want to use.

Any module that falls under the top level module has a dedicated feature flag, for example
`nstd.core` has the feature flag `nstd_core` and `nstd.alloc` has the feature flag `nstd_alloc`.

Each module may have additional features, for example `nstd.os` has the additional
`nstd_os_windows_alloc` feature for memory allocation on Windows, this allows other modules to use
the low level memory allocation API for Windows without enabling memory allocation for other
operating systems.

The `clib` feature flag is used to build `nstd` as a C library.

The `std` feature flag links the Rust standard library into the binary.

The `asm` feature permits the library to use assembly to optimize certain build configurations.

`std` and `nstd_core` are enabled by default.

Example:
```sh
cargo build --release --features "clib nstd_io nstd_string nstd_vec"
```

To build with all features:
```sh
cargo build --release --all-features
```

# Releases
`nstd` versions follow the Semantic Versioning rules. Each release is given a major, minor, and
patch number that makes up that version of the library (major.minor.patch).

There have not yet been any major releases for the framework as it is not yet stable.

A new minor version is released every 6 weeks, exactly 1 week after a new minor Rust release.

Patch releases are released every so often with minor fixes and additions.

See [semver.org](https://semver.org/) to learn more about Semantic Versioning.
