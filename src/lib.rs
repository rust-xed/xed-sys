
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
    unreachable_code)]

pub mod xed_interface {
    include!(concat!(env!("OUT_DIR"), "/xed_interface.rs"));
}

pub mod xed_version {
    include!(concat!(env!("OUT_DIR"), "/xed_version.rs"));
}
