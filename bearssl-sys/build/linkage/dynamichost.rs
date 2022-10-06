pub static HEADER_PATH: &str = "wrapper.h";

pub fn configure() {
    #[cfg(not(unix))]
    panic!("linking dynamically is currently supported in only unix targets");

    println!("cargo:rustc-link-search=/lib");
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=bearssl");
}
