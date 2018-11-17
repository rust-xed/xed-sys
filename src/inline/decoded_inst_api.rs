use std::os::raw::c_uint;
use xed_interface::*;

/// @ingroup DEC
/// Return true if the instruction is valid
pub unsafe fn xed_decoded_inst_valid(p: *const xed_decoded_inst_t) -> xed_bool_t {
    return (!(*p)._inst.is_null()) as xed_bool_t;
}

/// @ingroup DEC
/// Return the #xed_inst_t structure for this instruction. This is the
/// route to the basic operands form information.
pub unsafe fn xed_decoded_inst_inst(p: *const xed_decoded_inst_t) -> *const xed_inst_t {
    return (*p)._inst;
}

/// @ingroup DEC
/// Return the instruction #xed_category_enum_t enumeration
pub unsafe fn xed_decoded_inst_get_category(p: *const xed_decoded_inst_t) -> xed_category_enum_t {
    assert!(!(*p)._inst.is_null());
    return xed_inst_category((*p)._inst);
}

/// @ingroup DEC
/// Return the instruction #xed_extension_enum_t enumeration
pub unsafe fn xed_decoded_inst_get_extension(p: *const xed_decoded_inst_t) -> xed_extension_enum_t {
    assert!(!(*p)._inst.is_null());
    return xed_inst_extension((*p)._inst);
}

/// @ingroup DEC
/// Return the instruction #xed_isa_set_enum_t enumeration
pub unsafe fn xed_decoded_inst_get_isa_set(p: *const xed_decoded_inst_t) -> xed_isa_set_enum_t {
    assert!(!(*p)._inst.is_null());
    return xed_inst_isa_set((*p)._inst);
}

/// @ingroup DEC
/// Return the instruction #xed_iclass_enum_t enumeration.
pub unsafe fn xed_decoded_inst_get_iclass(p: *const xed_decoded_inst_t) -> xed_iclass_enum_t {
    assert!(!(*p)._inst.is_null());
    return xed_inst_iclass((*p)._inst);
}

/// @ingroup DEC
/// Obtain a constant pointer to the operands
pub unsafe fn xed_decoded_inst_operands_const(
    p: *const xed_decoded_inst_t,
) -> *const xed_operand_values_t {
    return p as *const _;
}

/// @ingroup DEC
/// Obtain a non-constant pointer to the operands
pub unsafe fn xed_decoded_inst_operands(p: *mut xed_decoded_inst_t) -> *mut xed_operand_values_t {
    return p as *mut _;
}

/// Return the number of operands
/// @ingroup DEC
pub unsafe fn xed_decoded_inst_noperands(p: *const xed_decoded_inst_t) -> c_uint {
    return xed_inst_noperands(xed_decoded_inst_inst(p));
}

/// @ingroup DEC
/// Set the machine mode and stack addressing width directly. This is NOT a
/// full initialization; Call #xed_decoded_inst_zero() before using this if
/// you want a clean slate.
pub unsafe fn xed_decoded_inst_set_mode(
    p: *mut xed_decoded_inst_t,
    mmode: xed_machine_mode_enum_t,
    stack_addr_width: xed_address_width_enum_t,
) {
    let dstate = xed_state_t {
        mmode,
        stack_addr_width,
    };

    xed_operand_values_set_mode(p, &dstate as *const _);
}

/// @ingroup DEC
/// Return the length of the decoded  instruction in bytes.
pub unsafe fn xed_decoded_inst_get_length(p: *const xed_decoded_inst_t) -> xed_uint_t {
    return (*p)._decoded_length as u32;
}

/// @ingroup DEC
/// Read itext byte.
pub unsafe fn xed_decoded_inst_get_byte(
    p: *const xed_decoded_inst_t,
    byte_index: xed_uint_t,
) -> u8 {
    return *(*p)._byte_array._dec.offset(byte_index as isize);
}

/// @ingroup DEC
/// Returns 16/32/64 indicating the machine mode with in bits. This is
/// derived from the input mode information.
pub unsafe fn xed_decoded_inst_get_machine_mode_bits(p: *const xed_decoded_inst_t) -> xed_uint_t {
    let mode = xed3_operand_get_mode(p);
    if mode == 2 {
        return 64;
    }
    if mode == 1 {
        return 32;
    }
    return 16;
}

/// @ingroup DEC
/// Returns 16/32/64 indicating the stack addressing mode with in
/// bits. This is derived from the input mode information.
pub unsafe fn xed_decoded_inst_get_stack_address_mode_bits(
    p: *const xed_decoded_inst_t,
) -> xed_uint_t {
    let smode = xed3_operand_get_smode(p);
    if smode == 2 {
        return 64;
    }
    if smode == 1 {
        return 32;
    }
    return 16;
}

/// Return the user-specified #xed_chip_enum_t chip name, or
/// XED_CHIP_INVALID if not set.
/// @ingroup DEC
pub unsafe fn xed_decoded_inst_get_input_chip(p: *const xed_decoded_inst_t) -> xed_chip_enum_t {
    return xed3_operand_get_chip(p);
}

/// Set a user-specified #xed_chip_enum_t chip name for restricting decode
/// @ingroup DEC
pub unsafe fn xed_decoded_inst_set_input_chip(p: *mut xed_decoded_inst_t, chip: xed_chip_enum_t) {
    xed3_operand_set_chip(p, chip);
}

/// @ingroup DEC
/// Return the instruction iform enum of type #xed_iform_enum_t .
pub unsafe fn xed_decoded_inst_get_iform_enum(p: *const xed_decoded_inst_t) -> xed_iform_enum_t {
    assert!(!(*p)._inst.is_null());
    return xed_inst_iform_enum((*p)._inst);
}

/// @ingroup DEC
/// Return the instruction zero-based iform number based on masking the
/// corresponding #xed_iform_enum_t. This value is suitable for
/// dispatching. The maximum value for a particular iclass is provided by
/// #xed_iform_max_per_iclass() .
pub unsafe fn xed_decoded_inst_get_iform_enum_dispatch(p: *const xed_decoded_inst_t) -> c_uint {
    assert!(!(*p)._inst.is_null());
    return xed_inst_iform_enum((*p)._inst)
        - xed_iform_first_per_iclass(xed_inst_iclass((*p)._inst));
}

/// @ingroup DEC
/// Return the second immediate.
pub unsafe fn xed_decoded_inst_get_second_immediate(p: *const xed_decoded_inst_t) -> u8 {
    return xed3_operand_get_uimm1(p);
}

/// @ingroup DEC
/// Return a user data field for arbitrary use by the user after decoding.
pub unsafe fn xed_decoded_inst_get_user_data(p: *mut xed_decoded_inst_t) -> u64 {
    return (*p).u.user_data;
}

/// @ingroup DEC
/// Modify the user data field.
pub unsafe fn xed_decoded_inst_set_user_data(p: *mut xed_decoded_inst_t, new_value: u64) {
    (*p).u.user_data = new_value;
}
