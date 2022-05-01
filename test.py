import os

FEATURES = ("nstd_alloc", "nstd_core", "nstd_os_alloc")

for feature in FEATURES:
    cmd = f"cargo build --quiet --no-default-features --features \"std {feature}\""
    print(f"Running {cmd}...")
    os.system(cmd)
