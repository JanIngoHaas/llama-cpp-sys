use std::env;
use std::path::PathBuf;
use cmake::Config;

#[cfg(target_os = "macos")]
fn make_config() -> Config {
    let mut cfg = Config::new("external/llama.cpp");
    cfg.build_target("preinstall")
        .define("BUILD_SHARED_LIBS", "true")
        .define("LLAMA_METAL", "ON")
        .define("LLAMA_BUILD_EXAMPLES", "OFF")
        .define("LLAMA_BUILD_TESTS", "OFF")
    .define("LLAMA_BUILD_TESTS", "OFF");
    cfg
}

#[cfg(not(target_os = "macos"))]
fn make_config() -> Config {
    let mut cfg = Config::new("external/llama.cpp");
    cfg.build_target("preinstall")
        .define("BUILD_SHARED_LIBS", "true")
        .define("LLAMA_BUILD_EXAMPLES", "OFF")
        .define("LLAMA_BUILD_TESTS", "OFF");
    cfg
}

fn main() {
    let mut config = make_config();
    let dst = config
        .very_verbose(true)
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=dylib=llama");

    println!("cargo:rerun-if-changed=external/llama.cpp/llama.h");
    println!("cargo:rerun-if-changed=external/llama.cpp/common/common.h");
    println!("cargo:rerun-if-changed=external/llama.cpp/common/sampling.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Iexternal/llama.cpp")
        .header("external/llama.cpp/llama.h")
        .allowlist_function("llama_.*")
        .allowlist_type("llama_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}