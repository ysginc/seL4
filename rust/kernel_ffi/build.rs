use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("kernel_ffi crate should live under rust/kernel_ffi");

    let include_root = repo_root.join("include");
    let arch_root = include_root.join("arch/riscv");
    let shims_root = manifest_dir.join("include_shims");
    let wrapper = manifest_dir.join("wrapper.h");

    println!("cargo:rerun-if-changed={}", wrapper.display());
    println!("cargo:rerun-if-changed={}", include_root.join("basic_types.h").display());
    println!("cargo:rerun-if-changed={}", arch_root.join("arch/types.h").display());
    println!("cargo:rerun-if-changed={}", shims_root.display());

    let bindings = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .clang_arg(format!("-I{}", shims_root.display()))
        .clang_arg(format!("-I{}", include_root.display()))
        .clang_arg(format!("-I{}", arch_root.display()))
        .allowlist_type("region_t")
        .allowlist_type("p_region_t")
        .allowlist_type("v_region_t")
        .allowlist_type("kernel_frame_t")
        .layout_tests(true)
        .derive_debug(true)
        .generate_comments(true)
        .generate()
        .expect("bindgen failed to generate kernel layout bindings");

    let out = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let out_file = out.join("bindings.rs");
    bindings
        .write_to_file(&out_file)
        .expect("failed to write generated bindings");
}
