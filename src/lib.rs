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
    intra_doc_link_resolution_failure,
    clippy::all
)]

extern crate core;

pub use crate::_detail::{c2rust::*, xed_interface_inner::*};

// Note: Remove this when v0.4 rolls around
#[deprecated(since = "0.3.0", note = "All exports are now in the crate root")]
pub mod xed_interface {
    pub use crate::*;
}

// Note: Remove this when v0.4 rolls around
#[deprecated(since = "0.3.0", note = "All exports are now in the crate root")]
pub mod xed_version {
    pub use crate::{xed_get_copyright, xed_get_version};
}

// This module shouldn't conflict with any of the stuff exported
// in the root.
mod _detail {
    // TODO: If we support no_std we should just import
    //       the actual libc crate here or something that
    //       exports ctypes such as the cty crate.
    mod libc {
        pub(crate) use std::os::raw::*;
    }

    pub(crate) mod c2rust {
        #![allow(unused_variables, unused_assignments, unused_mut)]

        use super::libc;
        use super::xed_interface_inner::*;

        // The c2rust conversion produces code that uses these,
        // luckily binding them manually is pretty easy.
        type uint8_t = u8;
        type uint16_t = u16;
        type uint32_t = u32;
        type uint64_t = u64;

        type int8_t = i8;
        type int16_t = i16;
        type int32_t = i32;
        type int64_t = i64;

        // Manually fix up differences in anonymous type naming
        // conventions between c2rust and bindgen.
        // This should break pretty loudly if it becomes wrong
        // with a new version of XED.
        type C2RustUnnamed_6 = xed_encoder_operand_t__bindgen_ty_1;

        include!("xed-c2rust.rs");
    }

    pub(crate) mod xed_interface_inner {
        include!(concat!(env!("OUT_DIR"), "/xed_interface.rs"));
    }

    #[cfg(test)]
    mod tests {
        use crate::*;

        #[test]
        fn test_xed_get_copyright() {
            let copyright = unsafe {
                std::ffi::CStr::from_ptr(xed_get_copyright())
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

            let version_cstr = unsafe { CStr::from_ptr(xed_get_version()) };
            let version = version_cstr.to_string_lossy();
            let git_version = CStr::from_bytes_with_nul(XED_GIT_VERSION)
                .unwrap()
                .to_string_lossy();

            assert_eq!(version, git_version);
        }
    }
}
