extern crate bindgen;

mod build_support;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use build_support::{download, path_regex_escape, run_command, static_lib_filename, Target};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

fn build_from_source(target: Target) -> BuildManifest {
    let filament_source_dir = env::current_dir().unwrap().join("filament");
    println!("cargo:rerun-if-env-changed=FILAMENT_BUILD_OUT_DIR");
    let out_dir = env::current_dir().unwrap().join(
        env::var("FILAMENT_BUILD_OUT_DIR")
            .or(env::var("OUT_DIR"))
            .unwrap(),
    );
    let filament_build_dir = out_dir.join("filament");
    let filament_install_dir = filament_build_dir.join("out");

    // configure filament
    fs::create_dir_all(&filament_build_dir).unwrap();
    let mut filament_cmake = Command::new("cmake");
    filament_cmake
        .current_dir(&filament_build_dir)
        .arg(filament_source_dir.to_str().unwrap())
        .arg(format!("-DCMAKE_BUILD_TYPE={}", "Release"))
        .arg(format!("-DFILAMENT_SKIP_SAMPLES={}", "ON"))
        .arg(format!("-DFILAMENT_SKIP_SDL2={}", "ON"))
        .arg(format!("-USE_STATIC_LIBCXX={}", "OFF"))
        .arg(format!("-DFILAMENT_SUPPORTS_VULKAN={}", "ON"))
        .arg(format!(
            "-DCMAKE_INSTALL_PREFIX={}",
            filament_install_dir.to_str().unwrap()
        ))
        .arg(format!("-DDIST_DIR={}", &target.to_string()));

    if cfg!(target_os = "linux") {
        filament_cmake.env("CC", env::var("CC").unwrap_or("clang".to_string()));
        filament_cmake.env(
            "CXXFLAGS",
            env::var("CXXFLAGS").unwrap_or("-stdlib=libc++".to_string()),
        );
        filament_cmake.env("CXX", env::var("CXX").unwrap_or("clang++".to_string()));
        filament_cmake.env("ASM", env::var("ASM").unwrap_or("clang".to_string()));
    }

    if !cfg!(target_os = "windows") {
        filament_cmake.env(
            "CMAKE_GENERATOR",
            env::var("CMAKE_GENERATOR").unwrap_or("Ninja".to_string()),
        );
    }

    run_command(&mut filament_cmake, "cmake");

    // build filament
    let mut filament_cmake_install = Command::new("cmake");
    filament_cmake_install
        .current_dir(&filament_build_dir)
        .args(["--build", "."])
        .args(["--target", "install"])
        .args(["--config", "Release"]);
    filament_cmake_install.args(["--parallel", "3"]);

    run_command(&mut filament_cmake_install, "cmake");

    let filament_native_lib = filament_install_dir.join("lib").join(&target.to_string());

    let filament_license = filament_install_dir.join("LICENSE");
    let filament_include = filament_install_dir.join("include");

    let filament_link_libs = vec![
        "filament",
        "backend",
        "bluevk",
        "bluegl",
        "filabridge",
        "filaflat",
        "smol-v",
        "vkshaders",
        "geometry",
        "ibl",
        "utils",
        "filameshio",
    ]
    .into_iter()
    .map(|v| v.to_string())
    .collect();

    println!("cargo:rerun-if-changed=bindings.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", filament_include.display()))
        .use_core()
        .header("bindings.h")
        .disable_header_comment()
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: true })
        .enable_cxx_namespaces()
        .allowlist_type("filament.*")
        .allowlist_type("utils.*")
        .allowlist_type("filamesh.*")
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("vec2.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("vec3.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("vec4.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("quat.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("mat2.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("mat3.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("mat4.h")
                .to_str()
                .unwrap(),
        ))
        .blocklist_file(path_regex_escape(
            filament_include
                .join("math")
                .join("mathfwd.h")
                .to_str()
                .unwrap(),
        ))
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let bindings_code = bindings.to_string();

    let bindings_code = bindings_code
        .replace(
            "root::filament::math::double4",
            "[::std::os::raw::c_double; 4]",
        )
        .replace(
            "root::filament::math::float4",
            "[::std::os::raw::c_float; 4]",
        )
        .replace(
            "root::filament::math::short4",
            "[::std::os::raw::c_short; 4]",
        )
        .replace(
            "root::filament::math::double3",
            "[::std::os::raw::c_double; 3]",
        )
        .replace(
            "root::filament::math::float3",
            "[::std::os::raw::c_float; 3]",
        )
        .replace(
            "root::filament::math::short3",
            "[::std::os::raw::c_short; 3]",
        )
        .replace(
            "root::filament::math::double2",
            "[::std::os::raw::c_double; 2]",
        )
        .replace(
            "root::filament::math::float2",
            "[::std::os::raw::c_float; 2]",
        )
        .replace(
            "root::filament::math::short2",
            "[::std::os::raw::c_short; 2]",
        )
        .replace(
            "root::filament::math::mat4f",
            "[::std::os::raw::c_float; 16]",
        )
        .replace(
            "root::filament::math::mat4",
            "[::std::os::raw::c_double; 16]",
        )
        .replace(
            "root::filament::math::mat3f",
            "[::std::os::raw::c_float; 9]",
        )
        .replace(
            "root::filament::math::mat3",
            "[::std::os::raw::c_double; 9]",
        )
        .replace(
            "root::filament::math::mat2f",
            "[::std::os::raw::c_float; 4]",
        )
        .replace(
            "root::filament::math::mat2",
            "[::std::os::raw::c_double; 4]",
        )
        .replace(
            "root::filament::math::quatf",
            "[::std::os::raw::c_float; 4]",
        )
        .replace(
            "root::filament::math::quat",
            "[::std::os::raw::c_double; 4]",
        );

    let bindings_rs = out_dir.join("bindings.rs");
    fs::write(&bindings_rs, bindings_code).expect("Couldn't write bindings!");

    BuildManifest {
        filament_native_lib,
        filament_license,
        filament_link_libs,
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
        filament_link_libs: manifest.filament_link_libs.clone(),
        bindings_rs: unpack_dir.join(manifest.bindings_rs),
        target: manifest.target.clone(),
    }
}

fn install(manifest: &BuildManifest) {
    println!("cargo:rerun-if-env-changed=FILAMENT_NATIVE_LIB_PATH");
    println!(
        "cargo:rustc-link-search=native={}",
        env::var("FILAMENT_NATIVE_LIB_PATH")
            .unwrap_or(manifest.filament_native_lib.display().to_string())
    );

    for lib in &manifest.filament_link_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib={}", "c++");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib={}", "c++");
        println!("cargo:rustc-link-lib={}", "framework=Metal");
        println!("cargo:rustc-link-lib={}", "framework=CoreVideo");
        println!("cargo:rustc-link-lib={}", "framework=Cocoa");
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib={}", "gdi32");
        println!("cargo:rustc-link-lib={}", "user32");
        println!("cargo:rustc-link-lib={}", "opengl32");
    }

    // Write the bindings to the src/bindings.rs file.
    let bindings_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings");
    fs::create_dir_all(&bindings_dir).unwrap();
    let bindings_path = bindings_dir.join("bindings.rs");
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
        .append_dir("lib", &manifest.filament_native_lib)
        .unwrap();

    for lib_name in &manifest.filament_link_libs {
        let filename = static_lib_filename(&lib_name);
        tar_builder
            .append_file(
                format!("lib/{}", filename),
                &mut fs::File::open(&manifest.filament_native_lib.join(filename)).unwrap(),
            )
            .unwrap();
    }

    let manifest_json = serde_json::to_string(&BuildManifest {
        filament_native_lib: PathBuf::from("lib"),
        filament_license: PathBuf::from("LICENSE"),
        filament_link_libs: manifest.filament_link_libs.clone(),
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
    pub filament_link_libs: Vec<String>,
    pub bindings_rs: PathBuf,
    pub target: String,
}

fn cache(cache_tar_name: impl AsRef<str>, version: impl AsRef<str>) -> BuildManifest {
    println!("cargo:rerun-if-env-changed=FILAMENT_BUILD_CACHE_DIR");
    if let Ok(cache_dir) = env::var("FILAMENT_BUILD_CACHE_DIR") {
        println!("cargo:rerun-if-changed={}", cache_dir);
        let package = Path::new(&cache_dir).join(cache_tar_name.as_ref());
        if fs::File::open(&package).is_ok() {
            return unpack(&package);
        }
    }

    let download_url = format!(
        "https://github.com/EYHN/filament-binaries/releases/download/filament-bindings/v{}/{}",
        version.as_ref(),
        cache_tar_name.as_ref()
    );

    println!("Downloading {}", download_url);
    let package = download(cache_tar_name, download_url).expect("Download Failed");
    return unpack(&package);
}

fn main() {
    let target = Target::target();
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    let cache_tar_name = format!("filament-{}-{}.tar.gz", version, target.to_string());

    println!("cargo:rerun-if-env-changed=FILAMENT_PREBUILT");
    let use_cache = env::var("FILAMENT_PREBUILT").unwrap_or("ON".to_string()) != "OFF"
        && cfg!(feature = "prebuilt");

    let build_manifest = if use_cache {
        cache(&cache_tar_name, &version)
    } else {
        let build_manifest = build_from_source(target);

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
