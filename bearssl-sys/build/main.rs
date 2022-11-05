mod linkage;

use std::path::PathBuf;

fn main() {
    linkage::configure();

    let usize_equals_size_t = !cfg!(feature = "dont-assume-size_t-equals-uintptr_t");

    let bindings = bindgen::builder()
        .use_core()
        .ctypes_prefix("::core::ffi")
        .header(linkage::HEADER_PATH)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("(br|BR)_.*")
        .allowlist_var("(br|BR)_.*")
        .allowlist_function("(br|BR)_.*")
        .blocklist_type("__.*_t")
        .blocklist_type("size_t")
        .generate_comments(false)
        .size_t_is_usize(usize_equals_size_t)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
