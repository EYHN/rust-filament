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
    if filament_source_dir.exists() == false {
        // source dir not exist, try to clone it
        let mut git_clone = Command::new("git");
        git_clone
            .arg("clone")
            .arg("https://github.com/google/filament.git")
            .arg(&filament_source_dir)
            .arg("--depth=1");
        build_support::run_command(&mut git_clone, "git");
    }

    println!("cargo:rerun-if-env-changed=FILAMENT_BUILD_OUT_DIR");
    let out_dir = env::current_dir().unwrap().join(
        env::var("FILAMENT_BUILD_OUT_DIR")
            .or(env::var("OUT_DIR"))
            .unwrap(),
    );
    let library_out_dir = out_dir.join("lib");
    let filament_build_dir = out_dir.join("filament");
    let filament_install_dir = filament_build_dir.join("out");
    fs::create_dir_all(&library_out_dir).unwrap();

    // configure filament
    fs::create_dir_all(&filament_build_dir).unwrap();
    let mut filament_cmake = Command::new("cmake");
    filament_cmake
        .current_dir(&filament_build_dir)
        .arg(filament_source_dir.to_str().unwrap())
        .arg(format!("-DCMAKE_BUILD_TYPE={}", "Release"))
        .arg(format!("-DFILAMENT_SKIP_SAMPLES={}", "ON"))
        .arg(format!("-DFILAMENT_SKIP_SDL2={}", "ON"))
        .arg(format!("-DUSE_STATIC_LIBCXX={}", "OFF"))
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
    filament_cmake_install.args([
        "--parallel",
        &env::var("NUM_JOBS").unwrap_or(num_cpus::get().to_string()),
    ]);

    run_command(&mut filament_cmake_install, "cmake");

    let filament_native_lib = filament_install_dir.join("lib").join(&target.to_string());

    let filament_license = filament_install_dir.join("LICENSE");
    let filament_include = filament_install_dir.join("include");

    let mut filament_link_libs: Vec<String> = vec![
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
        "gltfio",
        "meshoptimizer",
        "image",
    ]
    .into_iter()
    .map(|v| v.to_string())
    .collect();

    for lib in filament_link_libs.iter() {
        fs::copy(
            filament_native_lib.join(static_lib_filename(lib)),
            library_out_dir.join(static_lib_filename(lib)),
        )
        .unwrap();
    }

    // build c++ bindings library
    let mut cc_build = cc::Build::new();
    cc_build.file("bindings.cpp");
    cc_build.include(&filament_include);
    cc_build.cpp(true);
    cc_build.static_crt(true);
    cc_build.target(&target.to_string());
    cc_build.out_dir(&library_out_dir);
    cc_build.cargo_metadata(false);
    cc_build.warnings(false);

    if cfg!(target_os = "linux") {
        cc_build.compiler(PathBuf::from("clang++"));
    }
    if cfg!(target_os = "windows") {
        cc_build.flag("/std:c++latest");
    } else {
        cc_build.flag("-std=c++17");
        cc_build.cpp_set_stdlib("c++");
    }

    cc_build.compile("bindings");
    filament_link_libs.push("bindings".to_owned());

    println!("cargo:rerun-if-changed=bindings.cpp");

    let mut bindgen = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", filament_include.display()))
        .use_core()
        .size_t_is_usize(true)
        .header("bindings.cpp")
        .disable_header_comment()
        .allowlist_type("filament.*")
        .allowlist_type("utils.*")
        .allowlist_type("filamesh.*")
        .allowlist_type("gltf.*")
        .allowlist_type("image.*")
        .allowlist_function("helper_.*")
        .opaque_type("std::basic_string")
        .opaque_type("std::basic_string_value_type")
        .opaque_type("std::unique_ptr")
        .raw_line(include_str!("src/fix.rs"))
        .layout_tests(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-v");

    let block_file = |bindgen: bindgen::Builder, lib, file| {
        bindgen.blocklist_file(path_regex_escape(
            filament_include.join(lib).join(file).to_str().unwrap(),
        ))
    };

    bindgen = block_file(bindgen, "utils", "Slice.h");
    bindgen = block_file(bindgen, "math", "vec2.h");
    bindgen = block_file(bindgen, "math", "vec3.h");
    bindgen = block_file(bindgen, "math", "vec4.h");
    bindgen = block_file(bindgen, "math", "quat.h");
    bindgen = block_file(bindgen, "math", "mat2.h");
    bindgen = block_file(bindgen, "math", "mat3.h");
    bindgen = block_file(bindgen, "math", "mat4.h");
    bindgen = block_file(bindgen, "math", "mathfwd.h");
    bindgen = block_file(bindgen, "math", "TMatHelpers.h");
    bindgen = block_file(bindgen, "math", "TQuatHelpers.h");
    bindgen = block_file(bindgen, "math", "TVecHelpers.h");

    let bindings = bindgen.generate().expect("Unable to generate bindings");

    let bindings_code = bindings.to_string();

    let bindings_rs = out_dir.join("bindings.rs");
    fs::write(&bindings_rs, bindings_code).expect("Couldn't write bindings!");

    BuildManifest {
        link_search_dir: library_out_dir,
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
        link_search_dir: unpack_dir.join(manifest.link_search_dir),
        filament_license: unpack_dir.join(manifest.filament_license),
        link_libs: manifest.link_libs.clone(),
        bindings_rs: unpack_dir.join(manifest.bindings_rs),
        target: manifest.target.clone(),
    }
}

fn install(manifest: &BuildManifest) {
    println!(
        "cargo:rustc-link-search=native={}",
        manifest.link_search_dir.display().to_string()
    );

    for lib in &manifest.link_libs {
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
        .append_dir("lib", &manifest.link_search_dir)
        .unwrap();

    for lib_name in &manifest.link_libs {
        let filename = static_lib_filename(&lib_name);
        tar_builder
            .append_file(
                format!("lib/{}", filename),
                &mut fs::File::open(&manifest.link_search_dir.join(filename)).unwrap(),
            )
            .unwrap();
    }

    let manifest_json = serde_json::to_string(&BuildManifest {
        link_search_dir: PathBuf::from("lib"),
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
    pub link_search_dir: PathBuf,
    pub filament_license: PathBuf,
    pub link_libs: Vec<String>,
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
