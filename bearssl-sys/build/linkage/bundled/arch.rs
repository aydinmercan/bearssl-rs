#[cfg(target_arch = "x86_64")]
pub fn configure(build: &mut cc::Build) {
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

#[cfg(not(target_arch = "x86_64"))]
pub fn configure(build: &mut cc::Build) {
    #[cfg(target_pointer_width = 64)]
    build.define("BR_64", "1");
}
