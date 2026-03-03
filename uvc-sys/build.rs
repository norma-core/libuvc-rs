extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Only build for Linux targets - norma-core libuvc fork uses CLOCK_BOOTTIME
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "linux" {
        panic!(
            "norm-uvc only supports Linux targets (uses CLOCK_BOOTTIME), got target_os: {}",
            target_os
        );
    }

    // Use the generated headers from norm-uvc-src build (includes libuvc_config.h)
    let vendored_include = std::env::var("DEP_UVCSRC_INCLUDE").expect("DEP_UVCSRC_INCLUDE not set");

    let builder = bindgen::Builder::default().clang_arg(format!("-I{}", vendored_include));

    let bindings = builder
        .header("wrapper.h")
        .allowlist_function("uvc_.*")
        .allowlist_type("uvc_.*")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("uvc_bindings.rs"))
        .expect("Failed to write bindings");
}
