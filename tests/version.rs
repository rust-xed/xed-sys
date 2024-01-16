//! These tests serve as a good sanity check as to whether the output of the
//! build script is actually working and linking correctly.

use xed_sys::{xed_get_copyright, xed_get_version, XED_VERSION};

#[test]
fn test_xed_get_copyright() {
    let copyright = unsafe { std::ffi::CStr::from_ptr(xed_get_copyright()).to_string_lossy() };

    assert_eq!(
        "Copyright (C) 2022, Intel Corporation. All rights reserved.",
        &copyright
    );
}

#[test]
fn test_xed_version() {
    use std::ffi::CStr;

    let version_cstr = unsafe { CStr::from_ptr(xed_get_version()) };
    let version = version_cstr.to_string_lossy();
    let git_version = CStr::from_bytes_with_nul(XED_VERSION)
        .unwrap()
        .to_string_lossy();

    assert_eq!(version, git_version);
}
