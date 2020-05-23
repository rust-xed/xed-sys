use xed_sys::*;

#[test]
fn functions_exist() {
    unsafe {
        // Just ensuring that this compiles
        let _ = xed_isa_set_is_valid_for_chip(XED_ISA_SET_AES, XED_CHIP_AMD);
    }
}
