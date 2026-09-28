//! this build script compiles the mt-kahypar-upstream into a static library

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=mt-kahypar-upstream");

    let dst = cmake::Config::new("mt-kahypar-upstream")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .define("KAHYPAR_DOWNLOAD_TBB", "ON")
        // KAHYPAR_STATIC_LINK_TBB requires KAHYPAR_DOWNLOAD_TBB=ON
        .define("KAHYPAR_STATIC_LINK_TBB", "ON")
        // TBB_STRICT enables -Werror in TBB's own cmake, which breaks with GCC
        // >=15
        .define("TBB_STRICT", "OFF")
        .define("KAHYPAR_USE_64_BIT_IDS", "OFF")
        // NOTE: hwloc was disabled in the original build.rs but I want to keep
        // it to see if we notice any performance benefit but it means
        // adding a dynamic dependency. .define("KAHYPAR_DISABLE_HWLOC",
        // "ON") // disable HWLOC
        .build();

    let build_dir = dst.join("build");

    println!(
        "cargo:rustc-link-search=native={}",
        build_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=mtkahypar");

    let tbb_dir = find_tbb_lib_dir(&build_dir).expect(
        "could not find TBB static library directory after cmake build",
    );
    println!("cargo:rustc-link-search=native={}", tbb_dir.display());
    println!("cargo:rustc-link-lib=static=tbb");
    println!("cargo:rustc-link-lib=static=tbbmalloc");

    println!("cargo:rustc-link-lib=dylib=hwloc");
    // Apple platforms ship libc++ only; there is no libstdc++ to link against.
    let cxx_stdlib = match env::var("CARGO_CFG_TARGET_VENDOR").as_deref() {
        Ok("apple") => "c++",
        _ => "stdc++",
    };
    println!("cargo:rustc-link-lib=dylib={cxx_stdlib}");

    println!(
        "cargo:mtkahypar_manifest_dir={}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!("cargo:mtkahypar_out_dir={}", env::var("OUT_DIR").unwrap());
}

/// TBB places its compiled libraries in a platform-specific subdirectory of the
/// cmake binary dir (e.g. `gnu_15.2_cxx11_64_release/`). Walk the top level of
/// the build dir looking for a directory that contains `libtbb.a`.
fn find_tbb_lib_dir(build_dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(build_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && path.join("libtbb.a").exists() {
            return Some(path);
        }
    }
    None
}
