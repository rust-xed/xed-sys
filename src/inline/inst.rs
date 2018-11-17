use std::os::raw::*;
use xed_interface::*;

/// @ingroup DEC
pub unsafe fn xed_operand_name(p: *const xed_operand_t) -> xed_operand_enum_t {
    return (*p)._name as u32;
}

/// @ingroup DEC
pub unsafe fn xed_operand_operand_visibility(
    p: *const xed_operand_t,
) -> xed_operand_visibility_enum_t {
    return (*p)._operand_visibility as u32;
}

/// @ingroup DEC
/// @return The #xed_operand_type_enum_t of the operand template.
/// This is probably not what you want.
pub unsafe fn xed_operand_type(p: *const xed_operand_t) -> xed_operand_type_enum_t {
    return (*p)._type as u32;
}

/// @ingroup DEC
/// @return The #xed_operand_element_xtype_enum_t of the operand template.
/// This is probably not what you want.
pub unsafe fn xed_operand_xtype(p: *const xed_operand_t) -> xed_operand_element_xtype_enum_t {
    return (*p)._xtype as u32;
}

/// @ingroup DEC
pub unsafe fn xed_opearand_width(p: *const xed_operand_t) -> xed_operand_width_enum_t {
    return (*p)._oc2 as u32;
}

/// @ingroup DEC
pub unsafe fn xed_opreand_nonterminal_name(p: *const xed_operand_t) -> xed_nonterminal_enum_t {
    if (*p)._nt != 0 {
        return (*p)._u._nt;
    }
    return xed_nonterminal_enum_t_XED_NONTERMINAL_INVALID;
}
/// @ingroup DEC
/// Careful with this one -- use #xed_decoded_inst_get_reg()! This one is
/// probably not what you think it is. It is only used for hard-coded
/// registers implicit in the instruction encoding. Most likely you want to
/// get the #xed_operand_enum_t and then look up the instruction using
/// #xed_decoded_inst_get_reg(). The hard-coded registers are also available
/// that way.
/// @param p  an operand template,  #xed_operand_t.
/// @return  the implicit or suppressed registers, type #xed_reg_enum_t
pub unsafe fn xed_operand_reg(p: *const xed_operand_t) -> xed_reg_enum_t {
    if xed_operand_type(p) == xed_operand_type_enum_t_XED_OPERAND_TYPE_REG {
        return (*p)._u._reg;
    }
    return xed_reg_enum_t_XED_REG_INVALID;
}

/// @ingroup DEC
/// Careful with this one; See #xed_operand_is_register().
/// @param p  an operand template,  #xed_operand_t.
/// @return 1 if the operand template represents are register-type
/// operand.
///
///  Related functions:
///   Use #xed_decoded_inst_get_reg() to get the decoded name of /// the
///   register, #xed_reg_enum_t. Use #xed_operand_is_register() to test
///   #xed_operand_enum_t names.
pub unsafe fn xed_operand_template_is_register(p: *const xed_operand_t) -> xed_uint_t {
    return ((*p)._nt != 0 || (*p)._type as u32 == xed_operand_type_enum_t_XED_OPERAND_TYPE_REG)
        as u32;
}

/// @ingroup DEC
/// @param p  an operand template,  #xed_operand_t.
/// These operands represent branch displacements, memory displacements and
/// various immediates
pub unsafe fn xed_operand_imm(p: *const xed_operand_t) -> xed_uint_t {
    if xed_operand_type(p) == xed_operand_type_enum_t_XED_OPERAND_TYPE_IMM_CONST {
        return (*p)._u._imm;
    }
    return 0;
}

/// @ingroup DEC
/// Tests the enum for inclusion in XED_OPERAND_REG0 through XED_OPERAND_REG15.
/// @param name the operand name, type #xed_operand_enum_t
/// @return 1 if the operand name is REG0...REG15, 0 otherwise.
///
///Note there are other registers for memory addressing; See
/// #xed_operand_is_memory_addressing_register .
pub unsafe fn xed_operand_is_register(name: xed_operand_enum_t) -> xed_uint_t {
    return (name >= xed_operand_enum_t_XED_OPERAND_REG0
        && name <= xed_operand_enum_t_XED_OPERAND_REG8) as u32;
}

/// @ingroup DEC
/// Tests the enum for inclusion in XED_OPERAND_{BASE0,BASE1,INDEX,SEG0,SEG1}
/// @param name the operand name, type #xed_operand_enum_t
/// @return 1 if the operand name is for a memory addressing register operand, 0
/// otherwise. See also #xed_operand_is_register .
pub unsafe fn xed_operand_is_memory_addressing_register(name: xed_operand_enum_t) -> xed_uint_t {
    return (name == xed_operand_enum_t_XED_OPERAND_BASE0
        || name == xed_operand_enum_t_XED_OPERAND_INDEX
        || name == xed_operand_enum_t_XED_OPERAND_SEG0
        || name == xed_operand_enum_t_XED_OPERAND_BASE1
        || name == xed_operand_enum_t_XED_OPERAND_SEG1) as u32;
}

/// @ingroup DEC
/// DEPRECATED: Returns the raw R/W action. There are many cases for conditional reads
/// and writes. See #xed_decoded_inst_operand_action().
pub unsafe fn xed_operand_rw(p: *const xed_operand_t) -> xed_operand_action_enum_t {
    return (*p)._rw as u32;
}

pub unsafe fn xed_inst_iform_enum(p: *const xed_inst_t) -> xed_iform_enum_t {
    return (*p)._iform_enum as u32;
}

pub unsafe fn xed_inst_iclass(p: *const xed_inst_t) -> xed_iclass_enum_t {
    return xed_iform_to_iclass(xed_inst_iform_enum(p));
}

pub unsafe fn xed_inst_category(p: *const xed_inst_t) -> xed_category_enum_t {
    return xed_iform_to_category(xed_inst_iform_enum(p));
}

pub unsafe fn xed_inst_extension(p: *const xed_inst_t) -> xed_extension_enum_t {
    return xed_iform_to_extension(xed_inst_iform_enum(p));
}

pub unsafe fn xed_inst_isa_set(p: *const xed_inst_t) -> xed_isa_set_enum_t {
    return xed_iform_to_isa_set(xed_inst_iform_enum(p));
}

///@ingroup DEC
/// Number of instruction operands
pub unsafe fn xed_inst_noperands(p: *const xed_inst_t) -> c_uint {
    return (*p)._noperands as u32;
}

/// @ingroup DEC
/// Return #xed_exception_enum_t if present for the specified instruction.
/// This is currently only used for SSE and AVX instructions.
pub unsafe fn xed_inst_exception(p: *const xed_inst_t) -> xed_exception_enum_t {
    return (*p)._exceptions as u32;
}
