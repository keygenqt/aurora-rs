use std::{env, path::PathBuf};

use cxx_build::CFG;

fn main() {
    // Init
    // std::thread::sleep(std::time::Duration::from_secs(60));

    let sysroot = PathBuf::from(env::var("PKG_CONFIG_SYSROOT_DIR").unwrap());
    let include = &sysroot.join("usr/include").to_string_lossy().to_string();
    let lib = &sysroot.join("usr/lib").to_string_lossy().to_string();

    // Set search
    println!("cargo:rustc-link-search={lib}");

    // C appdir
    println!("cargo:rustc-link-lib=appdir");
    bindgen::Builder::default()
        .header(format!("{include}/appdir/appdir.h"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate appdir")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("libappdir.rs"))
        .expect("Couldn't write appdir!");

    // Qt5::Qt5Core
    let qt5_core = pkg_config::probe_library("Qt5Core").unwrap();
    let qt5_core_include = qt5_core.include_paths.iter().map(PathBuf::as_path);
    CFG.exported_header_dirs.extend(qt5_core_include);

    // Qt5::Qt5Network
    let qt5_network = pkg_config::probe_library("Qt5Network").unwrap();
    let qt5_network_include = qt5_network.include_paths.iter().map(PathBuf::as_path);
    CFG.exported_header_dirs.extend(qt5_network_include);

    // cxx
    #[cfg(target_os = "linux")]
    {
        // Set static stdc++
        println!("cargo:rustc-link-lib=static=stdc++");
        // Get arch build
        let target = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        // Include static stdc++
        if target == "aarch64" {
            println!("cargo:rustc-link-search=/usr/lib/gcc-cross/aarch64-linux-gnu/12");
        }
        // @todo aarch64 qt crash
        for bridge_name in ["format", "qnetwork"] {
            cxx_build::bridge(format!("src/cxx/rust/cxx_{bridge_name}.rs"))
                .std("c++17")
                .file(format!("src/cxx/cpp/{bridge_name}/cxx_{bridge_name}.cpp"))
                .cpp_link_stdlib("stdc++")
                .cpp_link_stdlib_static(true)
                .compile(bridge_name);
            println!("cargo:rerun-if-changed=src/cxx/cpp/{bridge_name}/cxx_{bridge_name}.h");
            println!("cargo:rerun-if-changed=src/cxx/cpp/{bridge_name}/cxx_{bridge_name}.cpp");
            println!("cargo:rerun-if-changed=src/cxx/rust/cxx_{bridge_name}.rs");
        }
    }
}
