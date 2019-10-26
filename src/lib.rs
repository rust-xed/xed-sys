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

#[allow(clippy::all)]
mod c2rust;

#[allow(clippy::all)]
mod xed_interface_inner {
    include!(concat!(env!("OUT_DIR"), "/xed_interface.rs"));
}

#[allow(clippy::all)]
pub mod xed_interface {
    pub use crate::{c2rust::*, xed_interface_inner::*};
}

#[allow(clippy::all)]
pub mod xed_version {
    include!(concat!(env!("OUT_DIR"), "/xed_version.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xed_get_copyright() {
        let copyright = unsafe {
            std::ffi::CStr::from_ptr(xed_version::xed_get_copyright())
                .to_string_lossy()
                .to_string()
        };

        assert_eq!(
            "Copyright (C) 2017, Intel Corporation. All rights reserved.",
            &copyright
        );
    }

    #[test]
    fn test_xed_version() {
        use std::ffi::CStr;

        let version_cstr = unsafe { CStr::from_ptr(xed_version::xed_get_version()) };
        let version = version_cstr.to_string_lossy();
        let git_version = CStr::from_bytes_with_nul(xed_interface::XED_GIT_VERSION).unwrap().to_string_lossy();

        assert_eq!(version, git_version);
    }

}
