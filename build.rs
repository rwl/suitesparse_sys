extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor");

    let amd_header_path = src_dir.join("AMD").join("Include").join("amd.h");
    let config_include_path = src_dir.join("SuiteSparse_config");

    if cfg!(feature = "dynamic") {
        println!("cargo:rustc-link-lib=amd");
    } else {
        build_suitesparse_config(&src_dir);
        build_amd(&src_dir, true);
        build_amd(&src_dir, false);
    }

    let bindings = bindgen::Builder::default()
        .header(amd_header_path.to_str().unwrap())
        .allowlist_item("SUITESPARSE_.+")
        .allowlist_function("SuiteSparse_.+")
        .allowlist_item("AMD_.+")
        .allowlist_function("amd_.+")
        .clang_arg(format!("-I{}", config_include_path.to_str().unwrap()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_suitesparse_config(src_dir: &Path) {
    cc::Build::new()
        .file(
            src_dir
                .join("SuiteSparse_config")
                .join("SuiteSparse_config.c"),
        )
        .compile("suitesparseconfig");
}

fn setup_suitesparse_builder(long: bool, src_dir: &Path) -> cc::Build {
    let mut builder = cc::Build::new();

    if long {
        builder.define("DLONG", None);
    }

    builder.include(src_dir.join("SuiteSparse_config"));
    builder
}

fn build_amd(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);
    let dir = src_dir.join("AMD");
    let amd_src_dir = dir.join("Source");

    builder.include(dir.join("Include"));

    let amd_objects = [
        "amd_1",
        "amd_2",
        "amd_aat",
        "amd_control",
        "amd_defaults",
        "amd_dump",
        "amd_info",
        "amd_order",
        "amd_post_tree",
        "amd_postorder",
        "amd_preprocess",
        "amd_valid",
    ];

    for obj in amd_objects.iter() {
        builder.file(&amd_src_dir.join(format!("{obj}.c")));
    }

    builder.compile(if long { "amdl" } else { "amd" });
}
