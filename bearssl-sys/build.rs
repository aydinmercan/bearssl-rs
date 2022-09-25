use std::env;
use std::path::PathBuf;

#[cfg(not(feature = "bundled"))]
static HEADER_PATH: &str = "wrapper.h";

#[cfg(feature = "bundled")]
static HEADER_PATH: &str = "bundled/inc/bearssl.h";

#[cfg(feature = "dont-assume-size_t-equals-uintptr_t")]
static CTYPES_PREFIX: &str = "::libc";

#[cfg(not(feature = "dont-assume-size_t-equals-uintptr_t"))]
static CTYPES_PREFIX: &str = "::core::ffi";

#[cfg(not(feature = "bundled"))]
fn bearssl_handle_linkage() {
    #[cfg(not(unix))]
    panic!("linking dynamically is currently supported in only unix targets");

    println!("cargo:rustc-link-search=/lib");
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=bearssl");
}

#[cfg(feature = "bundled")]
#[cfg(target_arch = "x86_64")]
fn build_configure_arch_specific(build: &mut cc::Build) {
    let compiler = build.get_compiler();

    build.define("BR_64", "1");

    if compiler.is_like_gnu() || compiler.is_like_clang() {
        build.define("BR_INT128", "1");
    } else if compiler.is_like_msvc() {
        build.define("BR_UMUL128", "1");
    }

    #[cfg(target_feature = "sse2")]
    build.define("BR_SSE2", "1");

    #[cfg(target_feature = "aes")]
    build.define("BR_AES_X86NI", "1");
}

#[cfg(feature = "bundled")]
#[cfg(not(target_arch = "x86_64"))]
fn build_configure_arch_specific(build: &mut cc::Build) {
    #[cfg(target_pointer_width = 64)]
    build.define("BR_64", "1");
}

#[cfg(feature = "bundled")]
#[cfg(unix)]
fn build_configure_os_specific(build: &mut cc::Build) {
    build.define("BR_USE_UNIX_TIME", "1").define("BR_USE_URANDOM", "1");
}

#[cfg(feature = "bundled")]
#[cfg(windows)]
fn build_configure_os_specific(build: &mut cc::Build) {
    build.define("BR_USE_WIN32_RAND", "1").define("BR_USE_WIN32_TIME", "1");
}

#[cfg(feature = "bundled")]
fn bearssl_handle_linkage() {
    let mut build = cc::Build::new();

    build_configure_os_specific(&mut build);
    build_configure_arch_specific(&mut build);

    build
        .include("bundled/inc")
        .files(
            std::fs::read_dir("bundled/src")
                .expect("failed to get bundled source path")
                .map(|e| e.expect("failed to read get file entry").path()),
        )
        .static_flag(true)
        .compile("bearssl");

    println!("cargo:lib_dir={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=bearssl");
}

fn main() {
    bearssl_handle_linkage();

    let bindings = bindgen::builder()
        .use_core()
        .ctypes_prefix(CTYPES_PREFIX)
        .header(HEADER_PATH)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("(br|BR)_.*")
        .allowlist_var("(br|BR)_.*")
        .allowlist_function("(br|BR)_.*")
        .blocklist_type("__.*_t")
        .blocklist_type("size_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
