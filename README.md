# RBash Proof-of-Concept

This is merely a proof-of-concept to show how to build a Rust wrapper around CBash that also builds a Python wrapper.

To replicate, install Rust and git. Then run:

```
git clone https://github.com/GandaG/rbash.git
cd rbash
rustup override set nightly-x86_64-msvc
rustup component add rustfmt
rustup component add clippy
cargo build --release
```

### CBash Bindings

The bindings to CBash are built with a modified header and [rust-bindgen](https://github.com/rust-lang/rust-bindgen).
Make sure to fulfill its [requirements](https://rust-lang.github.io/rust-bindgen/requirements.html).
To generate the bindings:

```
cargo install bindgen
bindgen lib/cbash/CBash.h -o lib/src/bindings.rs
```

This is only required when the custom header RBash uses changes.

### Known Issues

* No error handling -> any c error leads to panic
* No documentation :)
