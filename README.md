[![Build Status](https://travis-ci.com/Phantomical/xed-sys.svg?branch=master)](https://travis-ci.com/Phantomical/xed-sys)
[![Build status](https://ci.appveyor.com/api/projects/status/u6krc5mee7sdjn7a/branch/master?svg=true)](https://ci.appveyor.com/project/Phantomical/xed-sys/branch/master)
[![Crates.io](https://img.shields.io/crates/v/xed-sys.svg)](https://crates.io/crates/xed-sys)

# xed-sys
Rust FFI bindings for Intel XED.

```rust
/// Similar to `examples/xed-min.c` from official Intel XED repository.
use xed_sys::xed_interface::*;

fn main() {
    unsafe {
        let (mmode, stack_addr_width) = (XED_MACHINE_MODE_LEGACY_32, XED_ADDRESS_WIDTH_32b);

        xed_tables_init();

        let itext: [u8; 15] = [
            0xf, 0x85, 0x99, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        for bytes in 0..16 {
            let mut xedd: xed_decoded_inst_t = ::std::mem::uninitialized();
            xed_decoded_inst_zero(&mut xedd);
            xed_decoded_inst_set_mode(&mut xedd, mmode, stack_addr_width);

            let xed_error: xed_error_enum_t = xed_decode(&mut xedd, itext.as_ptr(), bytes);
            let desc = std::ffi::CStr::from_ptr(xed_error_enum_t2str(xed_error)).to_string_lossy();
            println!("bytes={} error={}", bytes, desc);
        }
    }
}
```

## Building

In order to build this crate, you need:
* Python version 2.7, 3.4 or later ([to build XED](https://intelxed.github.io/build-manual/)).
* clang ([to build XED](https://intelxed.github.io/build-manual/) and [run bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html#requirements)).

Alternatively, you may check [.travis.yml](.travis.yml) to see the dependencies installed in the CI setup.

## Examples
You can find examples in the examples/ directory which can be compiled and run with Cargo.
For instance, the following sequence of commands builds and runs the xed-min example:

```
cd examples/xed-min
cargo run
```
