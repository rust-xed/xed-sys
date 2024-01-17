[![Build Status](https://travis-ci.com/Phantomical/xed-sys.svg?branch=master)](https://travis-ci.com/Phantomical/xed-sys)
[![Build status](https://ci.appveyor.com/api/projects/status/u6krc5mee7sdjn7a/branch/master?svg=true)](https://ci.appveyor.com/project/Phantomical/xed-sys/branch/master)
[![Crates.io](https://img.shields.io/crates/v/xed-sys.svg)](https://crates.io/crates/xed-sys)

# xed-sys
Rust FFI bindings for [Intel XED](https://intelxed.github.io/).

```rust
/// Similar to `examples/xed-min.c` from official Intel XED repository.
use xed_sys::*;

fn main() {
    unsafe {
        let (mmode, stack_addr_width) = (XED_MACHINE_MODE_LEGACY_32, XED_ADDRESS_WIDTH_32b);

        xed_tables_init();

        let itext: [u8; 15] = [
            0xf, 0x85, 0x99, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        for bytes in 0..16 {
            let mut xedd = ::std::mem::MaybeUninit::<xed_decoded_inst_t>::uninit();
            xed_decoded_inst_zero(xedd.as_mut_ptr());
            xed_decoded_inst_set_mode(xedd.as_mut_ptr(), mmode, stack_addr_width);

            let xed_error: xed_error_enum_t = xed_decode(xedd.as_mut_ptr(), itext.as_ptr(), bytes);
            let desc = std::ffi::CStr::from_ptr(xed_error_enum_t2str(xed_error)).to_string_lossy();
            println!("bytes={} error={}", bytes, desc);
        }
    }
}
```

## Building

In order to build this crate, you need:
* Python version 3.8 or later ([to build XED](https://intelxed.github.io/build-manual/)).
* A C compiler.

If you have the `bindgen` feature enabled then you will also need:
* clang [to run bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html#requirements).

## Examples
You can find usage examples in the examples/ directory.

These examples are meant to be executed with cargo.  For instance, to run the example named `xed_min`:

```
# cd to the crate's root directory
cargo run --example xed_min
```
