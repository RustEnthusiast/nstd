import os

FEATURES = ("nstd_alloc", "nstd_core", "nstd_heap_ptr",
            "nstd_os_alloc", "nstd_shared_ptr", "nstd_string", "nstd_vec")

for feature in FEATURES:
    cmd = f"cargo check --quiet --no-default-features --features \"std {feature}\""
    print(f"Running {cmd}...")
    os.system(cmd)
