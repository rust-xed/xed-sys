//! Intel XED Bindings.
//!
//! For the real docs see: https://intelxed.github.io
//!
//! Note that `xed_tables_init()` must be called before
//! using the library.

#![feature(extern_types)]

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
    intra_doc_link_resolution_failure,
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
    pub use crate::xed_interface_inner::*;
    pub use crate::c2rust::*;
}

pub mod xed_version {
    #[cfg(target_env = "msvc")]
    include!("xed_version.rs");
    #[cfg(not(target_env = "msvc"))]
    include!(concat!(env!("OUT_DIR"), "/xed_version.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xed_get_copyright() {
        let version = unsafe {
            std::ffi::CStr::from_ptr(xed_version::xed_get_copyright())
                .to_string_lossy()
                .to_string()
        };

        assert_eq!(
            "Copyright (C) 2017, Intel Corporation. All rights reserved.",
            &version
        );
    }

}
