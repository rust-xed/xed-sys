//! Intel XED Bindings.
//!
//! For the real docs see: <https://intelxed.github.io>
//!
//! Note that [`xed_tables_init()`][0] must be called before using the library.
//!
//! # Features
//!
//! - `bindgen` - Don't use the bundled bindings files and instead regenerate
//!   rust bindings from scratch at compile time. You should never need to
//!   enable this manually but it will be enabled by other features.
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
