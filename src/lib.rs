//! Intel XED Bindings.
//!
//! For the real docs see: <https://intelxed.github.io>
//!
//! Note that [`xed_tables_init()`][0] must be called before using the library.
//!
//! # Features
//! - `bindgen` - Don't use the bundled bindings files and instead regenerate
//!   rust bindings from scratch at compile time. You should never need to
//!   enable this manually but it will be enabled by other features.
//! - `dylib` - Link XED (and `enc2` libraries if enabled) dynamically. This
//!   allows you to work around the link errors that prevent you from linking
//!   both enc2 libraries statically.
//!
//! ## `enc2`
//! XED has the option to enable its [`enc2`][1] encoder. This contains a
//! function to generate an x86 instruction for _every single instruction
//! variant_. As such, it is slow to compile and running bindgen on the
//! resulting headers generates a mountain (20+MB) of rust code.
//!
//! The generated static libraries also unfortunately have some duplicate
//! symbols so attempting to link both will result in linker errors. To make
//! the error less confusing this library will fail to compile if multiple
//! `enc2` features are enabled.
//!
//! - `enc2-m64-a64` - Enable enc2 for 64-bit mode with 64-bit addresses.
//! - `enc2-m32-a32` - Enable enc2 for 32-bit mode with 32-bit addresses.
//!
//! [0]: crate::xed_tables_init
//! [1]: https://github.com/intelxed/xed/wiki/The-fast-encoder---enc2

#![no_std]
#![allow(
    clippy::all,
    rustdoc::all,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case
)]

extern crate core;

#[cfg(not(feature = "bindgen"))]
include!("bindings.rs");

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/xed.rs"));
