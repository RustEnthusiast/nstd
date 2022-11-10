/// Represents an `nstd` C module's build configuration.
#[allow(unused)]
struct CModule {
    /// The name of the module.
    name: &'static str,
    /// The module's source files.
    src: &'static [&'static str],
}
impl CModule {
    /// Builds the C module.
    #[cfg(feature = "cc")]
    #[allow(unused)]
    fn build(&self) {
        #[cfg(not(doc_cfg))]
        {
            use cc::Build;
            let mut cc = Build::new();
            cc.include("include")
                .warnings(true)
                .extra_warnings(true)
                .warnings_into_errors(true)
                .files(self.src)
                .compile(self.name);
        }
    }
}

/// Main entry point of build script.
fn main() {
    #[cfg(feature = "nstd_os_unix_alloc")]
    {
        use build_target::{Arch, Family};
        if build_target::target_family() == Ok(Family::Unix) {
            let nstd_os_unix_alloc = CModule {
                name: "nstd_os_unix_alloc_c",
                src: &["src/os/unix/alloc.c"],
            };
            if build_target::target_arch() == Ok(Arch::X86_64) {
                #[cfg(not(feature = "asm"))]
                {
                    nstd_os_unix_alloc.build();
                }
            } else {
                nstd_os_unix_alloc.build();
            }
        }
    }
}
