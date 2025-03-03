load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")
load("//:config.bzl", "get_rust_features")

rust_binary(
    name = "rust-features",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/main.rs",
    features = get_rust_features(),
)