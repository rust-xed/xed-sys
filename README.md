[![Build Status](https://travis-ci.com/Phantomical/xed-sys.svg?branch=master)](https://travis-ci.com/Phantomical/xed-sys)

# xed-sys
Rust bindings for Intel XED.

# Cargo setup

`Cargo.toml`:
```toml
[dependencies]
xed-sys2 = { git = "https://github.com/Phantomical/xed-sys", branch = "master" }
```

# Building

In order for this crate to build, you need:
* A nightly version of Rust.
* Python version 2.7, 3.4 or later ([to build XED](https://intelxed.github.io/build-manual/)).
* clang ([to run bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html#requirements)) and ([build XED](https://intelxed.github.io/build-manual/)).

Alternatively, you may check [.travis.yml](.travis.yml) to see the dependencies installed in the CI setup.

