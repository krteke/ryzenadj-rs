use std::env;
use std::path::Path;

const RYZENADJ_DIR: &str = "vendor/RyzenAdj";

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is not set");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH is not set");

    if target_arch != "x86_64" {
        panic!("ryzenadj-sys currently only supports x86_64, got {target_arch}");
    }

    emit_rerun_rules();

    #[cfg(feature = "dynamic")]
    {
        build_dynamic(&target_os);
    }

    #[cfg(not(feature = "dynamic"))]
    {
        build_static(&target_os);
    }
}

#[cfg(not(feature = "dynamic"))]
fn build_static(target_os: &str) {
    match target_os {
        "linux" => build_static_linux(),
        "windows" => build_static_windows(),
        other => panic!("unsupported target OS: {other}"),
    }
}

#[cfg(not(feature = "dynamic"))]
fn common_c_build() -> cc::Build {
    let mut build = cc::Build::new();

    build
        .include(format!("{RYZENADJ_DIR}/lib"))
        .define("_LIBRYZENADJ_INTERNAL", None)
        .file(format!("{RYZENADJ_DIR}/lib/api.c"))
        .file(format!("{RYZENADJ_DIR}/lib/cpuid.c"))
        .file(format!("{RYZENADJ_DIR}/lib/nb_smu_ops.c"));

    build
}

#[cfg(not(feature = "dynamic"))]
fn build_static_linux() {
    let mut build = common_c_build();

    let libpci = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libpci")
        .expect("failed to find libpci");

    build
        .includes(libpci.include_paths)
        .file(format!("{RYZENADJ_DIR}/lib/linux/osdep_linux.c"))
        .file(format!("{RYZENADJ_DIR}/lib/linux/osdep_linux_mem.c"))
        .file(format!(
            "{RYZENADJ_DIR}/lib/linux/osdep_linux_smu_kernel_module.c"
        ));

    build.compile("ryzenadj");

    pkg_config::Config::new()
        .probe("libpci")
        .expect("failed to find libpci");
}

#[cfg(not(feature = "dynamic"))]
fn build_static_windows() {
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_env != "msvc" {
        panic!(
            "ryzenadj-sys currently only supports \
             x86_64-pc-windows-msvc on Windows"
        );
    }

    let core = common_c_build();

    core.compile("ryzenadj_core");

    let mut win32 = cc::Build::new();

    win32
        .cpp(true)
        .include(format!("{RYZENADJ_DIR}/lib"))
        .include(format!("{RYZENADJ_DIR}/lib/win32"))
        .define("_LIBRYZENADJ_INTERNAL", None)
        .file(format!("{RYZENADJ_DIR}/lib/win32/osdep_win32.cpp"))
        .compile("ryzenadj_win32");

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let win32_dir = Path::new(&manifest_dir).join(RYZENADJ_DIR).join("win32");

    println!("cargo:rustc-link-search=native={}", win32_dir.display());

    println!("cargo:rustc-link-lib=dylib=WinRing0x64");
}

#[cfg(feature = "dynamic")]
fn build_dynamic(target_os: &str) {
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    match target_os {
        "linux" => {}

        "windows" => {
            if target_env != "msvc" {
                panic!("ryzenadj-sys currently only supports x86_64-pc-windows-msvc on Windows");
            }
        }

        other => {
            panic!("unsupported target OS: {other}");
        }
    }

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR is not set");

    let shared_dir = Path::new(&out_dir).join("shared");

    std::fs::create_dir_all(&shared_dir)
        .expect("failed to create dynamic library output directory");

    let mut config = cmake::Config::new(RYZENADJ_DIR);

    config
        .define("BUILD_SHARED_LIBS", "ON")
        .define("PREFER_STATIC_LINKING", "OFF")
        .build_target("libryzenadj")
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", &shared_dir)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", &shared_dir)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", &shared_dir)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY_DEBUG", &shared_dir)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELEASE", &shared_dir)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELWITHDEBINFO", &shared_dir)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY_MINSIZEREL", &shared_dir)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG", &shared_dir)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE", &shared_dir)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO", &shared_dir)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL", &shared_dir)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG", &shared_dir)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE", &shared_dir)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELWITHDEBINFO", &shared_dir)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_MINSIZEREL", &shared_dir);

    config.build();

    println!("cargo:rustc-link-search=native={}", shared_dir.display());

    match target_os {
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=ryzenadj");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=libryzenadj");
        }
        _ => unreachable!(),
    }
}

fn emit_rerun_rules() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    const FILES: &[&str] = &[
        "CMakeLists.txt",
        "lib/ryzenadj.h",
        "lib/ryzenadj_priv.h",
        "lib/api.c",
        "lib/cpuid.c",
        "lib/nb_smu_ops.c",
        "lib/nb_smu_ops.h",
        "lib/linux/osdep_linux.c",
        "lib/linux/osdep_linux_mem.c",
        "lib/linux/osdep_linux_mem.h",
        "lib/linux/osdep_linux_smu_kernel_module.c",
        "lib/linux/osdep_linux_smu_kernel_module.h",
        "lib/win32/osdep_win32.cpp",
        "lib/win32/OlsApi.h",
        "lib/win32/OlsDef.h",
        "win32/WinRing0x64.lib",
    ];

    for file in FILES {
        println!("cargo:rerun-if-changed={RYZENADJ_DIR}/{file}");
    }
}
