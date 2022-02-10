extern crate bindgen;

mod build_support;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};

use build_support::{download, Target};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

fn build_from_source<P>(filament_source_dir: P, target: Target) -> BuildManifest
where
    P: AsRef<Path>,
{
    let sdir = filament_source_dir.as_ref();
    println!("cargo:rerun-if-env-changed=FILAMENT_BUILD_OUT_DIR");
    let out_dir = PathBuf::from(
        env::var("FILAMENT_BUILD_OUT_DIR")
            .or(env::var("OUT_DIR"))
            .unwrap(),
    );
    println!("{}", out_dir.display());
    fs::create_dir_all(&out_dir).unwrap();
    let out_dir = fs::canonicalize(out_dir).unwrap();
    let dist_dir = target.to_string();
    let filament_dst = cmake::Config::new(sdir)
        .out_dir(&out_dir)
        // Set compiler to clang
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("FILAMENT_SKIP_SAMPLES", "ON")
        .define("CMAKE_C_COMPILER", "clang")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .define("CMAKE_ASM_COMPILER", "clang")
        .define("CMAKE_INSTALL_PREFIX", "./out")
        .define("DIST_DIR", &dist_dir)
        .generator("Ninja")
        .build();

    let filament_native_lib = filament_dst.join("build/out/lib").join(dist_dir);

    let filament_license = filament_dst.join("build/out/LICENSE");
    let filament_include = filament_dst.join("build/out/include");

    let filament_link_libs = vec![
        "filament",
        "backend",
        "bluevk",
        "bluegl",
        "filabridge",
        "filaflat",
        "smol-v",
        "vkshaders",
        "utils",
    ]
    .into_iter()
    .map(|v| v.to_string())
    .collect();

    println!("cargo:rerun-if-changed=bindings.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg(format!("-I{}", filament_include.display()))
        .header("bindings.h")
        .disable_header_comment()
        .raw_line("#![allow(clippy::all)]")
        .raw_line("#![allow(unknown_lints)]")
        .raw_line("#![allow(deref_nullptr)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("include!(\"fix.rs\");")
        .allowlist_type("filament::Engine")
        .blocklist_file(filament_include.join("math/vec2.h").to_str().unwrap())
        .blocklist_file(filament_include.join("math/vec3.h").to_str().unwrap())
        .blocklist_file(filament_include.join("math/vec4.h").to_str().unwrap())
        .blocklist_file(filament_include.join("math/quat.h").to_str().unwrap())
        .blocklist_file(filament_include.join("math/mat2.h").to_str().unwrap())
        .blocklist_file(filament_include.join("math/mat3.h").to_str().unwrap())
        .blocklist_file(filament_include.join("math/mat4.h").to_str().unwrap())
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let bindings_rs = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&bindings_rs)
        .expect("Couldn't write bindings!");

    BuildManifest {
        filament_native_lib,
        filament_license,
        link_libs: filament_link_libs,
        bindings_rs,
        target: target.to_string(),
    }
}

fn unpack(package: impl AsRef<Path>) -> BuildManifest {
    let unpack_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("unpack");
    fs::create_dir_all(&unpack_dir).unwrap();

    let file = fs::File::open(package).unwrap();
    let mut tar_archive = tar::Archive::new(GzDecoder::new(file));

    tar_archive.unpack(&unpack_dir).unwrap();

    let manifest_json = unpack_dir.join("manifest.json");
    let manifest: BuildManifest =
        serde_json::from_reader(io::BufReader::new(fs::File::open(manifest_json).unwrap()))
            .unwrap();

    BuildManifest {
        filament_native_lib: unpack_dir.join(manifest.filament_native_lib),
        filament_license: unpack_dir.join(manifest.filament_license),
        link_libs: manifest.link_libs.clone(),
        bindings_rs: unpack_dir.join(manifest.bindings_rs),
        target: manifest.target.clone(),
    }
}

fn install(manifest: &BuildManifest) {
    println!(
        "cargo:rustc-link-search=native={}",
        manifest.filament_native_lib.display()
    );

    for lib in &manifest.link_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    println!("cargo:rustc-link-lib=c++");

    // Write the bindings to the src/bindings.rs file.
    let bindings_path = PathBuf::from("src").join("bindings.rs");
    fs::copy(&manifest.bindings_rs, bindings_path).unwrap();
}

fn package(manifest: &BuildManifest, output: impl AsRef<Path>) {
    let file = fs::File::create(output).unwrap();
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar_builder = tar::Builder::new(enc);

    tar_builder
        .append_file(
            "bindings.rs",
            &mut fs::File::open(&manifest.bindings_rs).unwrap(),
        )
        .unwrap();
    tar_builder
        .append_file(
            "LICENSE",
            &mut fs::File::open(&manifest.filament_license).unwrap(),
        )
        .unwrap();

    tar_builder
        .append_dir_all("lib", &manifest.filament_native_lib)
        .unwrap();

    let manifest_json = serde_json::to_string(&BuildManifest {
        filament_native_lib: PathBuf::from("lib"),
        filament_license: PathBuf::from("LICENSE"),
        link_libs: manifest.link_libs.clone(),
        bindings_rs: PathBuf::from("bindings.rs"),
        target: manifest.target.clone(),
    })
    .unwrap();
    let manifest_json_date = manifest_json.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(manifest_json_date.len() as u64);
    header.set_cksum();
    header.set_mode(0o644);
    header.set_mtime(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u64,
    );

    tar_builder
        .append_data(&mut header, "manifest.json", manifest_json_date)
        .unwrap();

    tar_builder.finish().unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BuildManifest {
    pub filament_native_lib: PathBuf,
    pub filament_license: PathBuf,
    pub link_libs: Vec<String>,
    pub bindings_rs: PathBuf,
    pub target: String,
}

fn try_from_cache(
    cache_tar_name: impl AsRef<str>,
    version: impl AsRef<str>,
) -> Option<BuildManifest> {
    println!("cargo:rerun-if-env-changed=FILAMENT_BUILD_CACHE_DIR");
    if let Ok(cache_dir) = env::var("FILAMENT_BUILD_CACHE_DIR") {
        println!("cargo:rerun-if-changed={}", cache_dir);
        let package = Path::new(&cache_dir).join(cache_tar_name.as_ref());
        if fs::File::open(&package).is_ok() {
            return Some(unpack(&package));
        }
    }

    let download_url = format!(
        "https://github.com/EYHN/filament-binaries/releases/download/{}/{}",
        version.as_ref(),
        cache_tar_name.as_ref()
    );

    println!("Downloading {}", download_url);
    if let Ok(package) = download(cache_tar_name, download_url) {
        return Some(unpack(&package));
    } else {
        println!("Download Failed")
    }

    None
}

fn main() {
    let target = Target::target();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let cache_tar_name = format!("filament-{}-{}.tar.gz", version, target.to_string());

    let build_manifest = if let Some(cache) = try_from_cache(&cache_tar_name, &version) {
        cache
    } else {
        let build_manifest = build_from_source("filament", target);

        println!("cargo:rerun-if-env-changed=FILAMENT_BUILD_CACHE_DIR");
        if let Ok(cache_dir) = env::var("FILAMENT_BUILD_CACHE_DIR") {
            fs::create_dir_all(&cache_dir).unwrap();
            let output_tar_path = Path::new(&cache_dir).join(&cache_tar_name);
            package(&build_manifest, output_tar_path);
        }

        build_manifest
    };

    install(&build_manifest)
}
