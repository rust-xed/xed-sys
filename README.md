[![Build Status](https://travis-ci.com/Phantomical/xed-sys.svg?branch=master)](https://travis-ci.com/Phantomical/xed-sys)

# xed-sys
Rust bindings for Intel XED.

# Cargo.toml setup

```toml
[dependencies]
xed-sys2 = { git = "https://github.com/rust-xed/xed-sys", branch = "master" }
```

# Building

In order to build this crate, you need:
* A nightly version of Rust.
* Python version 2.7, 3.4 or later ([to build XED](https://intelxed.github.io/build-manual/)).
* clang ([to build XED](https://intelxed.github.io/build-manual/) and [run bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html#requirements)).

Alternatively, you may check [.travis.yml](.travis.yml) to see the dependencies installed in the CI setup.

# Examples
You can find examples in the examples/ directory which can be compiled and run with Cargo.
For instance, the following sequence of commands builds and runs the xed-min example:

```
cd examples/xed-min
cargo run
```
