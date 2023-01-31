//! `nstd`'s build script. Compiling and linking non-Rust modules is done here.
#![allow(unused)]

/// Represents a C/C++ module's compilation configuration.
#[derive(Default)]
struct CModule {
    /// The name of the module.
    name: &'static str,
    /// The module's source files.
    src: &'static [&'static str],
    /// Set to true if this module contains C++.
    cpp: bool,
    /// Flags to attempt to pass to the compiler.
    flags: &'static [&'static str],
}
impl CModule {
    /// Compiles and links the C/C++ module.
    fn build(self) {
        #[cfg(feature = "cc")]
        {
            use cc::Build;
            if std::env::var("DOCS_RS").is_err() {
                // Create the compiler.
                let mut cc = Build::new();
                // Add compiler flags.
                for flag in self.flags {
                    cc.flag_if_supported(flag);
                }
                // Compile.
                cc.include("include")
                    .warnings(true)
                    .extra_warnings(true)
                    .cpp(self.cpp)
                    .files(self.src)
                    .compile(self.name);
            }
        }
    }
}

/// Main entry point of the build script.
fn main() {
    println!("cargo:rerun-if-changed=src/*");
    println!("cargo:rerun-if-changed=include/*");
    #[cfg(feature = "nstd_timed_mutex")]
    {
        let nstd_timed_mutex = CModule {
            name: "nstd_timed_mutex",
            src: &["src/timed_mutex.cpp"],
            cpp: true,
            flags: &["-std=c++11", "/std:c++11"],
            ..Default::default()
        };
        nstd_timed_mutex.build();
    }
}
