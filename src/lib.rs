//! Intel XED Bindings.
//!
//! For the real docs see: https://intelxed.github.io
//!
//! Note that `xed_tables_init()` must be called before
//! using the library.

#![allow(
    non_snake_case,
    dead_code,
    non_camel_case_types,
    const_err,
    improper_ctypes,
    path_statements,
    unused_parens,
    unused_unsafe,
    no_mangle_const_items,
    non_upper_case_globals,
    unreachable_code,
    intra_doc_link_resolution_failure
)]

extern crate core;

mod c2rust;

mod xed_interface_inner {
    #[cfg(target_env = "msvc")]
    include!("xed_interface.rs");
    #[cfg(not(target_env = "msvc"))]
    include!(concat!(env!("OUT_DIR"), "/xed_interface.rs"));
}

pub mod xed_interface {
    pub use crate::c2rust::*;
    pub use crate::xed_interface_inner::*;
}

pub mod xed_version {
    #[cfg(target_env = "msvc")]
    include!("xed_version.rs");
    #[cfg(not(target_env = "msvc"))]
    include!(concat!(env!("OUT_DIR"), "/xed_version.rs"));
}
