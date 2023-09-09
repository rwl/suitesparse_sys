extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    if cfg!(not(any(
        feature = "cxsparse",
        feature = "ldl",
        feature = "amd",
        feature = "klu",
        feature = "cholmod",
        feature = "umfpack"
    ))) {
        return;
    }
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor");

    if cfg!(feature = "dynamic") {
        #[cfg(feature = "cxsparse")]
        println!("cargo:rustc-link-lib=cxsparse");
        #[cfg(feature = "ldl")]
        println!("cargo:rustc-link-lib=ldl");
        #[cfg(feature = "amd")]
        println!("cargo:rustc-link-lib=amd");
        #[cfg(feature = "klu")]
        println!("cargo:rustc-link-lib=klu");
        #[cfg(feature = "cholmod")]
        println!("cargo:rustc-link-lib=cholmod");
        #[cfg(feature = "umfpack")]
        println!("cargo:rustc-link-lib=umfpack");
    } else {
        build_suitesparse_config(&src_dir);
        for long in [true, false] {
            #[cfg(any(feature = "amd", feature = "klu"))]
            build_amd(&src_dir, long);

            #[cfg(any(feature = "klu"))]
            build_colamd(&src_dir, long);

            #[cfg(any(feature = "klu"))]
            build_btf(&src_dir, long);

            #[cfg(any(feature = "ldl"))]
            build_ldl(&src_dir, long);

            #[cfg(feature = "cxsparse")]
            for complex in [true, false] {
                build_cxsparse(&src_dir, long, complex);
            }

            #[cfg(feature = "klu")]
            {
                build_klu_common(&src_dir, long);
                for complex in [true, false] {
                    build_klu(&src_dir, long, complex);
                }
            }

            #[cfg(any(feature = "cholmod"))]
            build_cholmod_core(&src_dir, long);

            #[cfg(feature = "umfpack")]
            build_umfpack(&src_dir, false);

            let _ = long;
        }
    }

    let config_include_path = src_dir.join("SuiteSparse_config");
    let amd_include_path = src_dir.join("AMD").join("Include");
    let colamd_include_path = src_dir.join("COLAMD").join("Include");
    let btf_include_path = src_dir.join("BTF").join("Include");

    let builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", config_include_path.to_str().unwrap(),))
        .clang_arg(format!("-I{}", amd_include_path.to_str().unwrap(),))
        .clang_arg(format!("-I{}", btf_include_path.to_str().unwrap(),))
        .clang_arg(format!("-I{}", colamd_include_path.to_str().unwrap(),))
        .allowlist_item("SUITESPARSE_.+")
        .allowlist_function("SuiteSparse_.+");

    #[cfg(feature = "cxsparse")]
    let builder = {
        let cxsparse_header_path = src_dir.join("CXSparse").join("Include").join("cs.h");
        builder
            .header(cxsparse_header_path.to_str().unwrap())
            .allowlist_item("CS_.+")
            .allowlist_function("cs_.+")
    };

    #[cfg(feature = "ldl")]
    let builder = {
        let ldl_header_path = src_dir.join("LDL").join("Include").join("ldl.h");
        builder
            .header(ldl_header_path.to_str().unwrap())
            .allowlist_item("LDL_.+")
            .allowlist_function("ldl_.+")
    };

    #[cfg(feature = "amd")]
    let builder = {
        let amd_header_path = amd_include_path.join("amd.h");
        builder
            .header(amd_header_path.to_str().unwrap())
            .allowlist_item("AMD_.+")
            .allowlist_function("amd_.+")
    };

    #[cfg(feature = "klu")]
    let builder = {
        let klu_header_path = src_dir.join("KLU").join("Include").join("klu.h");
        builder
            .header(klu_header_path.to_str().unwrap())
            .allowlist_item("KLU_.+")
            .allowlist_function("klu_.+")
            .blocklist_function("klu_(l_)?analyze")
            .blocklist_function("klu_(l_)?factor")
    };

    #[cfg(feature = "cholmod")]
    let builder = {
        let cholmod_header_path = src_dir.join("CHOLMOD").join("Include").join("cholmod.h");
        builder
            .header(cholmod_header_path.to_str().unwrap())
            .allowlist_item("CHOLMOD_.+")
            .allowlist_function("cholmod_.+")
    };

    #[cfg(feature = "umfpack")]
    let builder = {
        let umfpack_header_path = src_dir.join("UMFPACK").join("Include").join("umfpack.h");
        builder
            .header(umfpack_header_path.to_str().unwrap())
            .allowlist_item("UMFPACK_.+")
            .allowlist_function("umfpack_.+")
    };

    let bindings = builder.generate().expect("Unable to generate bindings");

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

#[cfg(any(
    feature = "cxsparse",
    feature = "ldl",
    feature = "amd",
    feature = "klu",
    feature = "cholmod",
    feature = "umfpack"
))]
fn setup_suitesparse_builder(long: bool, src_dir: &Path) -> cc::Build {
    let mut builder = cc::Build::new();

    builder.flag("-Wno-unused-parameter");
    builder.flag("-Wno-unused-variable");
    builder.flag("-Wno-unused-but-set-variable");
    builder.flag("-Wno-implicit-fallthrough");
    builder.flag("-Wno-unknown-pragmas");
    builder.flag("-Wno-sign-compare");

    if long {
        builder.define("DLONG", None);
    }

    builder.include(src_dir.join("SuiteSparse_config"));
    builder
}

#[cfg(any(feature = "amd", feature = "klu"))]
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

#[cfg(any(feature = "klu"))]
fn build_colamd(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);

    let dir = src_dir.join("COLAMD");
    let src = dir.join("Source").join("colamd.c");
    builder.include(dir.join("Include")).file(src);

    builder.compile(if long { "colamdl" } else { "colamd" });
}

#[cfg(any(feature = "klu"))]
fn build_btf(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);

    let dir = src_dir.join("BTF");
    let btf_src_dir = dir.join("Source");
    builder.include(dir.join("Include"));

    let btf_objects = ["btf_maxtrans", "btf_order", "btf_strongcomp"];
    for obj in btf_objects.iter() {
        builder.file(&btf_src_dir.join(format!("{}.c", obj)));
    }

    builder.compile(if long { "btfl" } else { "btf" });
}

#[cfg(any(feature = "ldl"))]
fn build_ldl(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);

    let dir = src_dir.join("LDL");
    let btf_src_dir = dir.join("Source");
    builder.include(dir.join("Include"));

    builder.file(&btf_src_dir.join("ldl.c"));

    builder.compile(if long { "ldll" } else { "ldl" });
}

// fn build_cxsparse_common(src_dir: &Path, long: bool) {
//     let cxsparse_src = src_dir.join("CXSparse").join("Source");
//
//     let mut builder = setup_suitesparse_builder(long, src_dir);
//     let objects = [
//         "_add",
//         "_amd",
//         "_chol",
//         "_cholsol",
//         "_compress",
//         "_convert",
//         "_counts",
//         "_cumsum",
//         "_dfs",
//         "_dmperm",
//         "_droptol",
//         "_dropzeros",
//         "_dupl",
//         "_entry",
//         "_ereach",
//         "_etree",
//         "_fkeep",
//         "_gaxpy",
//         "_happly",
//         "_house",
//         "_ipvec",
//         "_leaf",
//         "_load",
//         "_lsolve",
//         "_ltsolve",
//         "_lu",
//         "_lusol",
//         "_malloc",
//         "_maxtrans",
//         "_multiply",
//         "_norm",
//         "_permute",
//         "_pinv",
//         "_post",
//         "_print",
//         "_pvec",
//         "_qr",
//         "_qrsol",
//         "_randperm",
//         "_reach",
//         "_scatter",
//         "_scc",
//         "_schol",
//         "_spsolve",
//         "_sqr",
//         "_symperm",
//         "_tdfs",
//         "_transpose",
//         "_updown",
//         "_usolve",
//         "_util",
//         "_utsolve",
//     ];
//
//     for obj in &objects {
//         builder.file(cxsparse_src.join(format!("cs{}.c", obj)));
//     }
//
//     builder
//         .include(src_dir.join("CXSparse").join("Include"))
//         .include(src_dir.join("SuiteSparse_config"))
//         .compile(if long { "csl_common" } else { "cs_common" });
// }

#[cfg(feature = "cxsparse")]
fn build_cxsparse(src_dir: &Path, long: bool, complex: bool) {
    let cxsparse_src = src_dir.join("CXSparse").join("Source");

    let mut builder = setup_suitesparse_builder(long, src_dir);
    let mut name = "cs".to_string();

    if complex {
        builder.define("COMPLEX", None);
        name.push('c');
    } else {
        name.push('d');
    }
    if long {
        builder.define("DLONG", None);
        name.push('l');
    } else {
        name.push('i');
    }

    let objects = [
        "_add",
        "_amd",
        "_chol",
        "_cholsol",
        "_compress",
        "_convert",
        "_counts",
        "_cumsum",
        "_dfs",
        "_dmperm",
        "_droptol",
        "_dropzeros",
        "_dupl",
        "_entry",
        "_ereach",
        "_etree",
        "_fkeep",
        "_gaxpy",
        "_happly",
        "_house",
        "_ipvec",
        "_leaf",
        "_load",
        "_lsolve",
        "_ltsolve",
        "_lu",
        "_lusol",
        "_malloc",
        "_maxtrans",
        "_multiply",
        "_norm",
        "_permute",
        "_pinv",
        "_post",
        "_print",
        "_pvec",
        "_qr",
        "_qrsol",
        "_randperm",
        "_reach",
        "_scatter",
        "_scc",
        "_schol",
        "_spsolve",
        "_sqr",
        "_symperm",
        "_tdfs",
        "_transpose",
        "_updown",
        "_usolve",
        "_util",
        "_utsolve",
    ];

    for obj in &objects {
        builder.file(cxsparse_src.join(format!("cs{}.c", obj)));
    }

    builder
        .include(src_dir.join("CXSparse").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile(&name);
}

#[cfg(any(feature = "klu"))]
fn build_klu_common(src_dir: &Path, long: bool) {
    let klu_src = src_dir.join("KLU").join("Source");

    let mut builder = setup_suitesparse_builder(long, src_dir);
    let objects = [
        "_analyze",
        "_analyze_given",
        "_defaults",
        "_memory",
        "_free_symbolic",
    ];

    for obj in &objects {
        builder.file(klu_src.join(format!("klu{}.c", obj)));
    }

    builder
        .include(src_dir.join("KLU").join("Include"))
        .include(src_dir.join("AMD").join("Include"))
        .include(src_dir.join("COLAMD").join("Include"))
        .include(src_dir.join("BTF").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile(if long { "klul_common" } else { "klu_common" });
}

#[cfg(any(feature = "klu"))]
fn build_klu(src_dir: &Path, long: bool, complex: bool) {
    let klu_src = src_dir.join("KLU").join("Source");

    let mut builder = setup_suitesparse_builder(long, src_dir);
    let mut name = "klu".to_string();

    if long {
        builder.define("DLONG", None);
        name.push('l');
    }

    if complex {
        builder.define("COMPLEX", None);
        name.push('z');
    }

    let objects = [
        "",
        "_diagnostics",
        "_dump",
        "_factor",
        "_free_numeric",
        "_kernel",
        "_refactor",
        "_scale",
        "_solve",
        "_sort",
        "_tsolve",
    ];

    for obj in &objects {
        builder.file(klu_src.join(format!("klu{}.c", obj)));
    }

    builder
        .include(src_dir.join("KLU").join("Include"))
        .include(src_dir.join("AMD").join("Include"))
        .include(src_dir.join("COLAMD").join("Include"))
        .include(src_dir.join("BTF").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile(&name);
}

#[cfg(any(feature = "cholmod"))]
fn build_cholmod_core(src_dir: &Path, long: bool) {
    let cholmod_src = src_dir.join("CHOLMOD").join("Core");

    let mut builder = setup_suitesparse_builder(long, src_dir);
    let objects = [
        "_aat",
        "_add",
        "_band",
        "_change_factor",
        "_common",
        "_complex",
        "_copy",
        "_dense",
        "_error",
        "_factor",
        "_memory",
        "_sparse",
        "_transpose",
        "_triplet",
        "_version",
    ];

    for obj in &objects {
        builder.file(cholmod_src.join(format!("cholmod{}.c", obj)));
    }

    builder
        .include(src_dir.join("CHOLMOD").join("Include"))
        // .include(src_dir.join("AMD").join("Include"))
        // .include(src_dir.join("COLAMD").join("Include"))
        // .include(src_dir.join("BTF").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile(if long {
            "cholmodl_common"
        } else {
            "cholmod_common"
        });
}

#[cfg(feature = "umfpack")]
fn build_umfpack(src_dir: &Path, long: bool) {
    let umfpack_src = src_dir.join("UMFPACK").join("Source");

    let mut builder = setup_suitesparse_builder(long, src_dir);

    let objects = [
        "umf_analyze",
        "umf_apply_order",
        "umf_assemble",
        "umf_blas3_update",
        "umf_build_tuples",
        "umf_cholmod",
        "umf_colamd",
        "umf_create_element",
        "umf_dump",
        "umf_extend_front",
        "umf_free",
        "umf_fsize",
        "umf_garbage_collection",
        "umf_get_memory",
        "umf_grow_front",
        "umf_init_front",
        "umf_is_permutation",
        "umf_kernel",
        "umf_kernel_init",
        "umf_kernel_wrapup",
        "umf_local_search",
        "umf_lsolve",
        "umf_ltsolve",
        "umf_malloc",
        "umf_mem_alloc_element",
        "umf_mem_alloc_head_block",
        "umf_mem_alloc_tail_block",
        "umf_mem_free_tail_block",
        "umf_mem_init_memoryspace",
        "umfpack_col_to_triplet",
        "umfpack_copy_numeric",
        "umfpack_copy_symbolic",
        "umfpack_defaults",
        "umfpack_deserialize_numeric",
        "umfpack_deserialize_symbolic",
        "umfpack_free_numeric",
        "umfpack_free_symbolic",
        "umfpack_get_determinant",
        "umfpack_get_lunz",
        "umfpack_get_numeric",
        "umfpack_get_symbolic",
        "umfpack_load_numeric",
        "umfpack_load_symbolic",
        "umfpack_numeric",
        "umfpack_qsymbolic",
        "umfpack_report_control",
        "umfpack_report_info",
        "umfpack_report_matrix",
        "umfpack_report_numeric",
        "umfpack_report_perm",
        "umfpack_report_status",
        "umfpack_report_symbolic",
        "umfpack_report_triplet",
        "umfpack_report_vector",
        "umfpack_save_numeric",
        "umfpack_save_symbolic",
        "umfpack_scale",
        "umfpack_serialize_numeric",
        "umfpack_serialize_symbolic",
        "umfpack_solve",
        "umfpack_symbolic",
        "umfpack_tictoc",
        "umfpack_timer",
        "umfpack_transpose",
        "umfpack_triplet_to_col",
        "umf_realloc",
        "umf_report_perm",
        "umf_report_vector",
        "umf_row_search",
        "umf_scale",
        "umf_scale_column",
        "umf_set_stats",
        "umf_singletons",
        "umf_solve",
        "umf_start_front",
        "umf_store_lu",
        "umf_symbolic_usage",
        "umf_transpose",
        "umf_triplet",
        "umf_tuple_lengths",
        "umf_usolve",
        "umf_utsolve",
        "umf_valid_numeric",
        "umf_valid_symbolic",
    ];

    for obj in &objects {
        builder.file(umfpack_src.join(format!("{}.c", obj)));
    }

    builder
        .include(src_dir.join("UMFPACK").join("Include"))
        .include(src_dir.join("AMD").join("Include"))
        .include(src_dir.join("CHOLMOD").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile("umf");
}
