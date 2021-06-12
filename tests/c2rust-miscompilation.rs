use xed_sys::*;

// c2rust seems to miscompile this recently. This test ensures that
// we're doing the right thing.
#[test]
fn xed_state_get_address_width_miscompilation() {
    fn test_val(mmode: xed_machine_mode_enum_t) -> xed_address_width_enum_t {
        unsafe {
            let mut state = std::mem::zeroed();
            xed_state_init2(&mut state, mmode, XED_ADDRESS_WIDTH_64b);

            xed_state_get_address_width(&mut state)
        }
    }

    assert_eq!(test_val(XED_MACHINE_MODE_LONG_64), XED_ADDRESS_WIDTH_64b);
    assert_eq!(test_val(XED_MACHINE_MODE_REAL_16), XED_ADDRESS_WIDTH_32b);
    assert_eq!(test_val(XED_MACHINE_MODE_REAL_32), XED_ADDRESS_WIDTH_32b);
    assert_eq!(test_val(XED_MACHINE_MODE_LEGACY_32), XED_ADDRESS_WIDTH_32b);
    assert_eq!(test_val(XED_MACHINE_MODE_LEGACY_16), XED_ADDRESS_WIDTH_16b);
    assert_eq!(
        test_val(XED_MACHINE_MODE_LONG_COMPAT_32),
        XED_ADDRESS_WIDTH_32b
    );
    assert_eq!(
        test_val(XED_MACHINE_MODE_LONG_COMPAT_16),
        XED_ADDRESS_WIDTH_16b
    );
}
