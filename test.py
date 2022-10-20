import os

FEATURES = ("nstd_alloc", "nstd_core", "nstd_cstring", "nstd_fs", "nstd_heap_ptr", "nstd_io",
            "nstd_math", "nstd_os_windows_alloc", "nstd_os_windows_shared_lib", "nstd_shared_lib",
            "nstd_shared_ptr", "nstd_string", "nstd_vec")

TARGETS = ("x86_64-pc-windows-msvc", "x86_64-apple-darwin",
           "x86_64-unknown-linux-gnu", "x86_64-apple-ios", "x86_64-linux-android")

if __name__ == "__main__":
    for target in TARGETS:
        for feature in FEATURES:
            feature_tag = f"--features \"std {feature}\""
            target_tag = f"--target {target}"
            cmd = f"cargo check --quiet --no-default-features {feature_tag} {target_tag}"
            print(f"Running {cmd}...")
            os.system(cmd)
    cmd = "cargo clippy --quiet --all-features"
    print(f"Running {cmd}...")
    os.system(cmd)
