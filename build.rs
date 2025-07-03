extern crate pkg_config;

use std::env;
use std::path::{Path, PathBuf};

const ENV_GMSH: &str = "GMSH_LIB_DIR";

fn main() {
    if let Some(lib_dir) = env::var_os(ENV_GMSH) {
        // use env variable if defined
        println!("cargo:rerun-if-env-changed={}", ENV_GMSH);

        let lib_dir = Path::new(&lib_dir);
        let dylib_name = format!("{}gmsh{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX);
        if lib_dir.join(dylib_name).exists() || lib_dir.join("gmsh.lib").exists() {
            println!("cargo:rustc-link-search={}", lib_dir.display());
            // blocked on https://github.com/rust-lang/cargo/issues/4895
            // println!("cargo:rustc-dynamic-link-search={}", lib_dir.display())
        } else {
            println!(
                "cargo:warning={} is set to {:?}, but no shared library files were found there",
                ENV_GMSH, lib_dir
            );
        }
    } else {
        // fallback to system libraries
        match try_pkgconfig() {
            Ok(link_paths) => {
                for path in link_paths.into_iter() {
                    println!("cargo:rustc-link-search={}", path.display());
                }
            }
            Err(not_found) => {
                eprintln!("pkg-config couldn't find Gmsh library {}, ", not_found)
            }
        }
    }

    // always link the gmsh shared library at the end
    println!("cargo:rustc-link-lib=gmsh");
}

fn try_pkgconfig() -> Result<Vec<PathBuf>, String> {
    let mut pkg = pkg_config::Config::new();
    pkg.atleast_version("4.4");

    match pkg.probe("gmsh") {
        Ok(gmsh_lib) => Ok(gmsh_lib.link_paths),
        Err(err) => Err(err.to_string()),
    }
}
