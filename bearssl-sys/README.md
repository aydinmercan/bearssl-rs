# `bearssl-sys`

Rust bindings for BearSSL.

## Version

Currently BearSSL 0.6 and latest Rust stable is supported.

## Features

* `bundled`: Build and link statically BearSSL. Needs a working C compiler and autoenables some sensible flags for the build target.
* `dont-assume-size_t-equals-uintptr_t`: Use libc's `size_t` instead of `usize`. Should only needed when `sizeof(uintptr_t) != sizeof(size_t)`.

## License

This repository is licensed under the BSD-3-Clause. Please refer to `LICENSE` and
`CREDITS.md` for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
