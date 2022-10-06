mod arch;
mod os;

pub static HEADER_PATH: &str = "bundled/inc/bearssl.h";

pub fn configure() {
    let mut build = cc::Build::new();

    os::configure(&mut build);
    arch::configure(&mut build);

    build
        .include("bundled/inc")
        .files(
            std::fs::read_dir("bundled/src")
                .expect("failed to get bundled source path")
                .map(|e| e.expect("failed to read get file entry").path()),
        )
        .static_flag(true)
        .compile("bearssl");

    println!("cargo:lib_dir={}", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=bearssl");
}
