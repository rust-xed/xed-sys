use xed_interface::*;

/// Constructor.
/// DEPRECATED: use #xed_state_init2().
/// The mode, and addresses widths are enumerations that specify the number
/// of bits.  In 64b mode (#XED_MACHINE_MODE_LONG_64) the address width and
/// stack address widths are 64b (#XED_ADDRESS_WIDTH_64b). In other machine
/// modes, you must specify valid addressing widths.
///
/// @param p the pointer to the #xed_state_t type
/// @param arg_mmode the machine mode of type #xed_machine_mode_enum_t
/// @param arg_ignored Ignored. The addressing width is now implied by the machine mode implied by arg_mmmode.
/// @param arg_stack_addr_width the stack address width of type #xed_address_width_enum_t (only required if not the mode is not #XED_MACHINE_MODE_LONG_64)
/// @ingroup INIT
pub unsafe fn xed_state_init(
    p: *mut xed_state_t,
    arg_mmode: xed_machine_mode_enum_t,
    _arg_ignored: xed_address_width_enum_t,
    arg_stack_addr_width: xed_address_width_enum_t,
) {
    (*p).mmode = arg_mmode;
    (*p).stack_addr_width = arg_stack_addr_width;
}

/// Constructor.
/// The mode, and addresses widths are enumerations that specify the number
/// of bits.  In 64b mode (#XED_MACHINE_MODE_LONG_64) the address width and
/// stack address widths are 64b (#XED_ADDRESS_WIDTH_64b). In other machine
/// modes, you must specify valid addressing widths.
///
/// @param p the pointer to the #xed_state_t type
/// @param arg_mmode the machine mode of type #xed_machine_mode_enum_t
/// @param arg_stack_addr_width the stack address width of type #xed_address_width_enum_t (only required if not the mode is not #XED_MACHINE_MODE_LONG_64)
/// @ingroup INIT
pub unsafe fn xed_state_init2(
    p: *mut xed_state_t,
    arg_mmode: xed_machine_mode_enum_t,
    arg_stack_addr_width: xed_address_width_enum_t,
) {
    (*p).mmode = arg_mmode;
    (*p).stack_addr_width = arg_stack_addr_width;
}

/// clear the xed_state_t
/// @ingroup INIT
pub unsafe fn xed_state_zero(p: *mut xed_state_t) {
    (*p).mmode = xed_machine_mode_enum_t_XED_MACHINE_MODE_INVALID;
    (*p).stack_addr_width = xed_address_width_enum_t_XED_ADDRESS_WIDTH_INVALID;
}

/// return the machine mode
/// @ingroup INIT
pub unsafe fn xed_state_get_machine_mode(p: *const xed_state_t) -> xed_machine_mode_enum_t {
    return (*p).mmode;
}

/// true iff the machine is in LONG_64 mode
/// @ingroup INIT
pub unsafe fn xed_state_long64_mode(p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) == xed_machine_mode_enum_t_XED_MACHINE_MODE_LONG_64)
        as xed_bool_t;
}

/// @ingroup INIT
pub unsafe fn xed_state_real_mode(p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) == xed_machine_mode_enum_t_XED_MACHINE_MODE_REAL_16)
        as xed_bool_t;
}

/// @ingroup INIT
pub unsafe fn xed_state_mode_width_16(p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) == xed_machine_mode_enum_t_XED_MACHINE_MODE_LEGACY_16
        || xed_state_get_machine_mode(p) == xed_machine_mode_enum_t_XED_MACHINE_MODE_LONG_COMPAT_16)
        as xed_bool_t;
}

/// @ingroup INIT
pub unsafe fn xed_state_mode_width_32(p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) == xed_machine_mode_enum_t_XED_MACHINE_MODE_LEGACY_32
        || xed_state_get_machine_mode(p) == xed_machine_mode_enum_t_XED_MACHINE_MODE_LONG_COMPAT_32)
        as xed_bool_t;
}

/// Set the machine mode which corresponds to the default data operand size
/// @ingroup INIT
pub unsafe fn xed_state_set_machine_mode(p: *mut xed_state_t, arg_mode: xed_machine_mode_enum_t) {
    (*p).mmode = arg_mode;
}

/// return the address width
/// @ingroup INIT
pub unsafe fn xed_state_get_address_width(p: *const xed_state_t) -> xed_address_width_enum_t {
    match xed_state_get_machine_mode(p) {
        xed_machine_mode_enum_t_XED_MACHINE_MODE_LONG_64 => {
            xed_address_width_enum_t_XED_ADDRESS_WIDTH_64b
        }
        xed_machine_mode_enum_t_XED_MACHINE_MODE_REAL_16 => {
            xed_address_width_enum_t_XED_ADDRESS_WIDTH_32b
        }
        xed_machine_mode_enum_t_XED_MACHINE_MODE_LEGACY_32 => {
            xed_address_width_enum_t_XED_ADDRESS_WIDTH_32b
        }
        xed_machine_mode_enum_t_XED_MACHINE_MODE_LONG_COMPAT_32 => {
            xed_address_width_enum_t_XED_ADDRESS_WIDTH_32b
        }
        xed_machine_mode_enum_t_XED_MACHINE_MODE_LEGACY_16 => {
            xed_address_width_enum_t_XED_ADDRESS_WIDTH_16b
        }
        xed_machine_mode_enum_t_XED_MACHINE_MODE_LONG_COMPAT_16 => {
            xed_address_width_enum_t_XED_ADDRESS_WIDTH_16b
        }
        _ => xed_address_width_enum_t_XED_ADDRESS_WIDTH_INVALID,
    }
}

/// set the STACK address width
/// @ingroup INIT
pub unsafe fn xed_state_set_stack_address_width(
    p: *mut xed_state_t,
    arg_addr_width: xed_address_width_enum_t,
) {
    (*p).stack_addr_width = arg_addr_width;
}

/// Return the STACK address width
/// @ingroup INIT
pub unsafe fn xed_stack_get_stack_address_width(p: *const xed_state_t) -> xed_address_width_enum_t {
    return (*p).stack_addr_width as xed_address_width_enum_t;
}
