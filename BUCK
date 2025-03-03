load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")

features = read_config("features", "features", "").split(",")

rust_binary(
    name = "rust-features",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/main.rs",
)