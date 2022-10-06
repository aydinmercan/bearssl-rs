#[cfg(unix)]
pub fn configure(build: &mut cc::Build) {
    build.define("BR_USE_UNIX_TIME", "1").define("BR_USE_URANDOM", "1");
}

#[cfg(windows)]
pub fn configure(build: &mut cc::Build) {
    build.define("BR_USE_WIN32_RAND", "1").define("BR_USE_WIN32_TIME", "1");
}
