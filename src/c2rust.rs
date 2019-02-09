#![allow(unused_doc_comments, unused_mut, unused_assignments, unused_variables)]

mod libc {
    pub use std::os::raw::*;
}

use crate::xed_interface::*;


pub unsafe extern "C" fn xed_make_uint64(mut hi: u32, mut lo: u32) -> u64 {
    let mut y: xed_union64_t = xed_union64_t { byte: [0; 8] };
    y.s.lo32 = lo;
    y.s.hi32 = hi;
    return y.r#u64;
}

pub unsafe extern "C" fn xed_make_int64(mut hi: u32, mut lo: u32) -> i64 {
    let mut y: xed_union64_t = xed_union64_t { byte: [0; 8] };
    y.s.lo32 = lo;
    y.s.hi32 = hi;
    return y.r#i64;
}
unsafe extern "C" fn xed_iform_to_iclass(mut iform: xed_iform_enum_t) -> xed_iclass_enum_t {
    let mut ii: *const xed_iform_info_t = xed_iform_map(iform);
    if !ii.is_null() {
        return (*ii).iclass() as xed_iclass_enum_t;
    }
    return XED_ICLASS_INVALID;
}

pub unsafe extern "C" fn xed_operand_name(mut p: *const xed_operand_t) -> xed_operand_enum_t {
    return (*p)._name as xed_operand_enum_t;
}

pub unsafe extern "C" fn xed_operand_operand_visibility(
    mut p: *const xed_operand_t,
) -> xed_operand_visibility_enum_t {
    return (*p)._operand_visibility as xed_operand_visibility_enum_t;
}

pub unsafe extern "C" fn xed_operand_type(mut p: *const xed_operand_t) -> xed_operand_type_enum_t {
    return (*p)._type as xed_operand_type_enum_t;
}

pub unsafe extern "C" fn xed_operand_xtype(
    mut p: *const xed_operand_t,
) -> xed_operand_element_xtype_enum_t {
    return (*p)._xtype as xed_operand_element_xtype_enum_t;
}

pub unsafe extern "C" fn xed_operand_width(
    mut p: *const xed_operand_t,
) -> xed_operand_width_enum_t {
    return (*p)._oc2 as xed_operand_width_enum_t;
}

pub unsafe extern "C" fn xed_operand_nonterminal_name(
    mut p: *const xed_operand_t,
) -> xed_nonterminal_enum_t {
    if 0 != (*p)._nt {
        return (*p)._u._nt;
    }
    return XED_NONTERMINAL_INVALID;
}

pub unsafe extern "C" fn xed_operand_reg(mut p: *const xed_operand_t) -> xed_reg_enum_t {
    if xed_operand_type(p) as libc::c_uint == XED_OPERAND_TYPE_REG as libc::c_int as libc::c_uint {
        return (*p)._u._reg;
    }
    return XED_REG_INVALID;
}

pub unsafe extern "C" fn xed_operand_template_is_register(
    mut p: *const xed_operand_t,
) -> xed_uint_t {
    return (0 != (*p)._nt as libc::c_int
        || (*p)._type as libc::c_int == XED_OPERAND_TYPE_REG as libc::c_int)
        as libc::c_int as xed_uint_t;
}

pub unsafe extern "C" fn xed_operand_imm(mut p: *const xed_operand_t) -> u32 {
    if xed_operand_type(p) as libc::c_uint
        == XED_OPERAND_TYPE_IMM_CONST as libc::c_int as libc::c_uint
    {
        return (*p)._u._imm;
    }
    return 0i32 as u32;
}

pub unsafe extern "C" fn xed_operand_is_register(mut name: xed_operand_enum_t) -> xed_uint_t {
    return (name as libc::c_uint >= XED_OPERAND_REG0 as libc::c_int as libc::c_uint
        && name as libc::c_uint <= XED_OPERAND_REG8 as libc::c_int as libc::c_uint)
        as libc::c_int as xed_uint_t;
}

pub unsafe extern "C" fn xed_operand_is_memory_addressing_register(
    mut name: xed_operand_enum_t,
) -> xed_uint_t {
    return (name as libc::c_uint == XED_OPERAND_BASE0 as libc::c_int as libc::c_uint
        || name as libc::c_uint == XED_OPERAND_INDEX as libc::c_int as libc::c_uint
        || name as libc::c_uint == XED_OPERAND_SEG0 as libc::c_int as libc::c_uint
        || name as libc::c_uint == XED_OPERAND_BASE1 as libc::c_int as libc::c_uint
        || name as libc::c_uint == XED_OPERAND_SEG1 as libc::c_int as libc::c_uint)
        as libc::c_int as xed_uint_t;
}

pub unsafe extern "C" fn xed_operand_rw(mut p: *const xed_operand_t) -> xed_operand_action_enum_t {
    return (*p)._rw as xed_operand_action_enum_t;
}

pub unsafe extern "C" fn xed_inst_iform_enum(mut p: *const xed_inst_t) -> xed_iform_enum_t {
    return (*p)._iform_enum as xed_iform_enum_t;
}

pub unsafe extern "C" fn xed_inst_iclass(mut p: *const xed_inst_t) -> xed_iclass_enum_t {
    return xed_iform_to_iclass(xed_inst_iform_enum(p));
}

pub unsafe extern "C" fn xed_inst_category(mut p: *const xed_inst_t) -> xed_category_enum_t {
    return xed_iform_to_category(xed_inst_iform_enum(p));
}

pub unsafe extern "C" fn xed_inst_extension(mut p: *const xed_inst_t) -> xed_extension_enum_t {
    return xed_iform_to_extension(xed_inst_iform_enum(p));
}

pub unsafe extern "C" fn xed_inst_isa_set(mut p: *const xed_inst_t) -> xed_isa_set_enum_t {
    return xed_iform_to_isa_set(xed_inst_iform_enum(p));
}

pub unsafe extern "C" fn xed_inst_noperands(mut p: *const xed_inst_t) -> libc::c_uint {
    return (*p)._noperands as libc::c_uint;
}

pub unsafe extern "C" fn xed_inst_exception(mut p: *const xed_inst_t) -> xed_exception_enum_t {
    return (*p)._exceptions as xed_exception_enum_t;
}

pub unsafe extern "C" fn xed_flag_set_mask(mut p: *const xed_flag_set_t) -> libc::c_int {
    return (*p).flat as libc::c_int;
}

pub unsafe extern "C" fn xed3_operand_get_sib(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.sib as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_sib(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.sib = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_last_f2f3(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.last_f2f3 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_last_f2f3(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.last_f2f3 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_disp_width(mut d: *const xed_decoded_inst_t) -> u8 {
    return (*d)._operands.disp_width;
}

pub unsafe extern "C" fn xed3_operand_set_disp_width(
    mut d: *mut xed_decoded_inst_t,
    mut opval: u8,
) {
    (*d)._operands.disp_width = opval;
}

pub unsafe extern "C" fn xed3_operand_get_using_default_segment0(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.using_default_segment0 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_using_default_segment0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.using_default_segment0 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_using_default_segment1(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.using_default_segment1 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_using_default_segment1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.using_default_segment1 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_hint(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.hint as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_hint(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.hint = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_sae(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.sae as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_sae(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.sae = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mode_first_prefix(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.mode_first_prefix as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mode_first_prefix(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mode_first_prefix = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_uimm1(mut d: *const xed_decoded_inst_t) -> u8 {
    return (*d)._operands.uimm1;
}

pub unsafe extern "C" fn xed3_operand_set_uimm1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: u8,
) {
    (*d)._operands.uimm1 = opval;
}

pub unsafe extern "C" fn xed3_operand_get_uimm0(mut d: *const xed_decoded_inst_t) -> u64 {
    return (*d)._operands.uimm0;
}

pub unsafe extern "C" fn xed3_operand_set_uimm0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: u64,
) {
    (*d)._operands.uimm0 = opval;
}

pub unsafe extern "C" fn xed3_operand_get_smode(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.smode as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_smode(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.smode = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_rm(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rm as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rm(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rm = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_chip(
    mut d: *const xed_decoded_inst_t,
) -> xed_chip_enum_t {
    return (*d)._operands.chip as xed_chip_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_chip(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_chip_enum_t,
) {
    (*d)._operands.chip = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_amd3dnow(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.amd3dnow as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_amd3dnow(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.amd3dnow = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_map(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.map as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_map(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.map = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_sibindex(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.sibindex as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_sibindex(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.sibindex = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_agen(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.agen as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_agen(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.agen = opval as u8;
}
// / @ingroup IFORM
/// Return the isa_set for a given iform. This function returns valid data
/// as soon as global data is initialized. (This function does not require
/// a decoded instruction as input).

pub unsafe extern "C" fn xed3_operand_get_nominal_opcode(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.nominal_opcode as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_nominal_opcode(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.nominal_opcode = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_seg1(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.seg1 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_seg1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.seg1 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_seg0(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.seg0 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_seg0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.seg0 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_cldemote(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.cldemote as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_cldemote(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.cldemote = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_nprefixes(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.nprefixes as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_nprefixes(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.nprefixes = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_index(
    mut d: *const xed_decoded_inst_t,
) -> xed_reg_enum_t {
    return (*d)._operands.index as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_index(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.index = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_ild_f2(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.ild_f2 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_ild_f2(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.ild_f2 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_scale(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.scale as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_scale(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.scale = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_llrc(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.llrc as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_llrc(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.llrc = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_esrc(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.esrc as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_esrc(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.esrc = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_nrexes(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.nrexes as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_nrexes(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.nrexes = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_pos_sib(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.pos_sib as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_pos_sib(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.pos_sib = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_osz(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.osz as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_osz(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.osz = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_has_sib(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.has_sib as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_has_sib(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.has_sib = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_eosz(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.eosz as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_eosz(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.eosz = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_element_size(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.element_size as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_element_size(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.element_size = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_pos_disp(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.pos_disp as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_pos_disp(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.pos_disp = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_ubit(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.ubit as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_ubit(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.ubit = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vexdest210(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.vexdest210 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_vexdest210(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vexdest210 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vexdest3(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.vexdest3 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_vexdest3(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vexdest3 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_cet(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.cet as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_cet(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.cet = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_ptr(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.ptr as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_ptr(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.ptr = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_p4(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.p4 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_p4(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.p4 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_modep55c(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.modep55c as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_modep55c(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.modep55c = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_iclass(
    mut d: *const xed_decoded_inst_t,
) -> xed_iclass_enum_t {
    return (*d)._operands.iclass as xed_iclass_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_iclass(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_iclass_enum_t,
) {
    (*d)._operands.iclass = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_imm_width(mut d: *const xed_decoded_inst_t) -> u8 {
    return (*d)._operands.imm_width;
}

pub unsafe extern "C" fn xed3_operand_set_imm_width(
    mut d: *mut xed_decoded_inst_t,
    mut opval: u8,
) {
    (*d)._operands.imm_width = opval;
}

pub unsafe extern "C" fn xed3_operand_get_bcrc(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.bcrc as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_bcrc(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.bcrc = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_error(
    mut d: *const xed_decoded_inst_t,
) -> xed_error_enum_t {
    return (*d)._operands.error as xed_error_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_error(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_error_enum_t,
) {
    (*d)._operands.error = opval as u8;
}
// / @ingroup FLAGS
/// boolean test to see if flags are read, scans the flags

pub unsafe extern "C" fn xed3_operand_get_nelem(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.nelem as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_nelem(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.nelem = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_imm0signed(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.imm0signed as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_imm0signed(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.imm0signed = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_reg8(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg8 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg8(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg8 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg6(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg6 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg6(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg6 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg7(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg7 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg7(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg7 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg4(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg4 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg4(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg4 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg5(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg5 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg5(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg5 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg2(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg2 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg2(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg2 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg3(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg3 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg3(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg3 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_reg0(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg0 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg0 = opval as u16;
}
// / @ingroup OPERANDS

pub unsafe extern "C" fn xed3_operand_get_reg1(mut d: *const xed_decoded_inst_t) -> xed_reg_enum_t {
    return (*d)._operands.reg1 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.reg1 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_ild_seg(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.ild_seg as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_ild_seg(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.ild_seg = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_base1(
    mut d: *const xed_decoded_inst_t,
) -> xed_reg_enum_t {
    return (*d)._operands.base1 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_base1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.base1 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_mod(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.mod_ as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mod(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mod_ = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_disp(mut d: *const xed_decoded_inst_t) -> i64 {
    return (*d)._operands.disp as i64;
}

pub unsafe extern "C" fn xed3_operand_set_disp(mut d: *mut xed_decoded_inst_t, mut opval: i64) {
    (*d)._operands.disp = opval as u64;
}

pub unsafe extern "C" fn xed3_operand_get_rex(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rex as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rex(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rex = opval as u8;
}
// / @ingroup OPERANDS
/// Returns true if the instruction has a SIB byte.
// index to table of xed_attributes_t structures

pub unsafe extern "C" fn xed3_operand_get_rexb(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rexb as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rexb(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rexb = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_pos_imm(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.pos_imm as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_pos_imm(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.pos_imm = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_rep(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rep as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rep(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rep = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_rexw(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rexw as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rexw(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rexw = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_ild_f3(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.ild_f3 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_ild_f3(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.ild_f3 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_rexr(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rexr as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rexr(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rexr = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_encoder_preferred(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.encoder_preferred as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_encoder_preferred(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.encoder_preferred = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_reg(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.reg as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_reg(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.reg = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_pos_nominal_opcode(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.pos_nominal_opcode as xed_bits_t;
}
//<PSEUDO

pub unsafe extern "C" fn xed3_operand_set_pos_nominal_opcode(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.pos_nominal_opcode = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_rexx(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rexx as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_rexx(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rexx = opval as u8;
}
//<PSEUDO

pub unsafe extern "C" fn xed3_operand_get_prefix66(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.prefix66 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_prefix66(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.prefix66 = opval as u8;
}
//@}
//<PSEUDO

pub unsafe extern "C" fn xed3_operand_get_rexrr(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.rexrr as xed_bits_t;
}
// / @name Memory Displacements
//@{
/// @ingroup OPERANDS
/// Return the memory displacement width in BYTES
//<PSEUDO

pub unsafe extern "C" fn xed3_operand_set_rexrr(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.rexrr = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_asz(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.asz as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_asz(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.asz = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mask(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.mask as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mask(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mask = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mem1(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.mem1 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mem1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mem1 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_easz(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.easz as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_easz(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.easz = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_pos_imm1(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.pos_imm1 as xed_bits_t;
}
//<PSEUDO

pub unsafe extern "C" fn xed3_operand_set_pos_imm1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.pos_imm1 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mem_width(mut d: *const xed_decoded_inst_t) -> u16 {
    return (*d)._operands.mem_width;
}

pub unsafe extern "C" fn xed3_operand_set_mem_width(
    mut d: *mut xed_decoded_inst_t,
    mut opval: u16,
) {
    (*d)._operands.mem_width = opval;
}

pub unsafe extern "C" fn xed3_operand_get_lzcnt(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.lzcnt as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_lzcnt(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.lzcnt = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mem0(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.mem0 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mem0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mem0 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_brdisp_width(
    mut d: *const xed_decoded_inst_t,
) -> u8 {
    return (*d)._operands.brdisp_width;
}
// / @name Exceptions
//@{
/// @ingroup DEC
/// Return #xed_exception_enum_t if present for the specified instruction.
/// This is currently only used for SSE and AVX instructions.
//<PSEUDO

pub unsafe extern "C" fn xed3_operand_set_brdisp_width(
    mut d: *mut xed_decoded_inst_t,
    mut opval: u8,
) {
    (*d)._operands.brdisp_width = opval;
}

pub unsafe extern "C" fn xed3_operand_get_imm1_bytes(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.imm1_bytes as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_imm1_bytes(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.imm1_bytes = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_tzcnt(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.tzcnt as xed_bits_t;
}
// / This converts strings to #xed_reg_enum_t types.
/// @param p An enumeration element of type xed_reg_enum_t.
/// @return string
/// @ingroup ENUM

pub unsafe extern "C" fn xed3_operand_set_tzcnt(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.tzcnt = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_df64(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.df64 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_df64(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.df64 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_lock(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.lock as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_lock(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.lock = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_has_modrm(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.has_modrm as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_has_modrm(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.has_modrm = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_zeroing(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.zeroing as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_zeroing(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.zeroing = opval as u8;
}
// //////////////////////////////////////////////////
// ENCODE API
////////////////////////////////////////////////////
/// @name Encoding
//@{
/// @ingroup OPERANDS    

pub unsafe extern "C" fn xed3_operand_get_srm(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.srm as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_srm(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.srm = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vexvalid(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.vexvalid as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_vexvalid(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vexvalid = opval as u8;
}
// / @ingroup OPERANDS

pub unsafe extern "C" fn xed3_operand_get_needrex(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.needrex as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_needrex(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.needrex = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_skip_osz(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.skip_osz as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_skip_osz(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.skip_osz = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_outreg(
    mut d: *const xed_decoded_inst_t,
) -> xed_reg_enum_t {
    return (*d)._operands.outreg as xed_reg_enum_t;
}
/* the effective_address_width is only requires to be set for
*  instructions * with implicit suppressed memops or memops with no
*  base or index regs.  When base or index regs are present, XED pick
*  this up automatically from the register names.

* FIXME: make effective_address_width required by all encodes for
* unifority. Add to xed_inst[0123]() APIs??? */

pub unsafe extern "C" fn xed3_operand_set_outreg(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.outreg = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_default_seg(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.default_seg as xed_bits_t;
}
// / @ingroup OPERANDS

pub unsafe extern "C" fn xed3_operand_set_default_seg(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.default_seg = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_nseg_prefixes(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.nseg_prefixes as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_nseg_prefixes(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.nseg_prefixes = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vex_c4(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.vex_c4 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_vex_c4(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vex_c4 = opval as u8;
}
// / @name Instruction Properties and prefixes
//@{
/// @ingroup ENCHL
/// This is to specify effective address size different than the
/// default. For things with base or index regs, XED picks it up from the
/// registers. But for things that have implicit memops, or no base or index
/// reg, we must allow the user to set the address width directly.
/// @param x The #xed_encoder_instruction_t being filled in.
/// @param width_bits The intended effective address size in bits.  Values: 16, 32 or 64.

pub unsafe extern "C" fn xed3_operand_get_pos_modrm(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.pos_modrm as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_pos_modrm(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.pos_modrm = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_bcast(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.bcast as xed_bits_t;
}
// / takes bytes, not bits, as an argument
/// @ingroup OPERANDS    

pub unsafe extern "C" fn xed3_operand_set_bcast(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.bcast = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_seg_ovd(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.seg_ovd as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_seg_ovd(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.seg_ovd = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vex_prefix(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.vex_prefix as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_vex_prefix(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vex_prefix = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_dummy(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.dummy as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_dummy(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.dummy = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_norex(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.norex as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_norex(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.norex = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_roundc(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.roundc as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_roundc(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.roundc = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_sibbase(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.sibbase as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_sibbase(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.sibbase = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_need_memdisp(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.need_memdisp as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_need_memdisp(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.need_memdisp = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_out_of_bytes(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.out_of_bytes as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_out_of_bytes(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.out_of_bytes = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_imm1(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.imm1 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_imm1(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.imm1 = opval as u8;
}
// / @ingroup ENCHL
/// instruction with no operands
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits

pub unsafe extern "C" fn xed3_operand_get_imm0(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.imm0 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_imm0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.imm0 = opval as u8;
}
// / @ingroup OPERANDS
/// Set the signed immediate using a BITS length

pub unsafe extern "C" fn xed3_operand_get_no_scale_disp8(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.no_scale_disp8 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_no_scale_disp8(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.no_scale_disp8 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_relbr(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.relbr as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_relbr(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.relbr = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_base0(
    mut d: *const xed_decoded_inst_t,
) -> xed_reg_enum_t {
    return (*d)._operands.base0 as xed_reg_enum_t;
}

pub unsafe extern "C" fn xed3_operand_set_base0(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_reg_enum_t,
) {
    (*d)._operands.base0 = opval as u16;
}

pub unsafe extern "C" fn xed3_operand_get_df32(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.df32 as xed_bits_t;
}
// / @ingroup OPERANDS
/// Set the unsigned immediate using a BIT length.

pub unsafe extern "C" fn xed3_operand_set_df32(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.df32 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_realmode(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.realmode as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_realmode(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.realmode = opval as u8;
}
// / @ingroup DEC
/// Result in BYTES

pub unsafe extern "C" fn xed3_operand_get_modrm_byte(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.modrm_byte as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_modrm_byte(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.modrm_byte = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mode(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.mode as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mode(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mode = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_mpxmode(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.mpxmode as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_mpxmode(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.mpxmode = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_sibscale(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.sibscale as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_sibscale(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.sibscale = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vl(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.vl as xed_bits_t;
}
// / @ingroup OPERANDS

pub unsafe extern "C" fn xed3_operand_set_vl(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vl = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_wbnoinvd(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.wbnoinvd as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_wbnoinvd(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.wbnoinvd = opval as u8;
}
// / @ingroup OPERANDS
/// Set the operand storage field entry named 'operand_name' to the
/// register value specified by 'reg_name'.

pub unsafe extern "C" fn xed3_operand_get_max_bytes(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.max_bytes as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_max_bytes(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.max_bytes = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_modep5(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.modep5 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_modep5(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.modep5 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_first_f2f3(
    mut d: *const xed_decoded_inst_t,
) -> xed_bits_t {
    return (*d)._operands.first_f2f3 as xed_bits_t;
}
// / @ingroup DEC
/// Return the specified register operand. The specifier is of type
/// #xed_operand_enum_t .

pub unsafe extern "C" fn xed3_operand_set_first_f2f3(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.first_f2f3 = opval as u8;
}

pub unsafe extern "C" fn xed3_operand_get_vexdest4(mut d: *const xed_decoded_inst_t) -> xed_bits_t {
    return (*d)._operands.vexdest4 as xed_bits_t;
}

pub unsafe extern "C" fn xed3_operand_set_vexdest4(
    mut d: *mut xed_decoded_inst_t,
    mut opval: xed_bits_t,
) {
    (*d)._operands.vexdest4 = opval as u8;
}

pub unsafe extern "C" fn xed_state_init(
    mut p: *mut xed_state_t,
    mut arg_mmode: xed_machine_mode_enum_t,
    mut arg_ignored: xed_address_width_enum_t,
    mut arg_stack_addr_width: xed_address_width_enum_t,
) {
    (*p).mmode = arg_mmode;
    (*p).stack_addr_width = arg_stack_addr_width;
}

pub unsafe extern "C" fn xed_state_init2(
    mut p: *mut xed_state_t,
    mut arg_mmode: xed_machine_mode_enum_t,
    mut arg_stack_addr_width: xed_address_width_enum_t,
) {
    (*p).mmode = arg_mmode;
    (*p).stack_addr_width = arg_stack_addr_width;
}

pub unsafe extern "C" fn xed_state_zero(mut p: *mut xed_state_t) {
    (*p).mmode = XED_MACHINE_MODE_INVALID;
    (*p).stack_addr_width = XED_ADDRESS_WIDTH_INVALID;
}

pub unsafe extern "C" fn xed_state_get_machine_mode(
    mut p: *const xed_state_t,
) -> xed_machine_mode_enum_t {
    return (*p).mmode;
}

pub unsafe extern "C" fn xed_state_long64_mode(mut p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) as libc::c_uint
        == XED_MACHINE_MODE_LONG_64 as libc::c_int as libc::c_uint) as libc::c_int
        as xed_bool_t;
}

pub unsafe extern "C" fn xed_state_real_mode(mut p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) as libc::c_uint
        == XED_MACHINE_MODE_REAL_16 as libc::c_int as libc::c_uint) as libc::c_int
        as xed_bool_t;
}

pub unsafe extern "C" fn xed_state_mode_width_16(mut p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) as libc::c_uint
        == XED_MACHINE_MODE_LEGACY_16 as libc::c_int as libc::c_uint
        || xed_state_get_machine_mode(p) as libc::c_uint
            == XED_MACHINE_MODE_LONG_COMPAT_16 as libc::c_int as libc::c_uint)
        as libc::c_int as xed_bool_t;
}

pub unsafe extern "C" fn xed_state_mode_width_32(mut p: *const xed_state_t) -> xed_bool_t {
    return (xed_state_get_machine_mode(p) as libc::c_uint
        == XED_MACHINE_MODE_LEGACY_32 as libc::c_int as libc::c_uint
        || xed_state_get_machine_mode(p) as libc::c_uint
            == XED_MACHINE_MODE_LONG_COMPAT_32 as libc::c_int as libc::c_uint)
        as libc::c_int as xed_bool_t;
}

pub unsafe extern "C" fn xed_state_set_machine_mode(
    mut p: *mut xed_state_t,
    mut arg_mode: xed_machine_mode_enum_t,
) {
    (*p).mmode = arg_mode;
}

pub unsafe extern "C" fn xed_state_get_address_width(
    mut p: *const xed_state_t,
) -> xed_address_width_enum_t {
    let mut current_block_4: u64;
    match xed_state_get_machine_mode(p) as libc::c_uint {
        1 => return XED_ADDRESS_WIDTH_64b,
        6 => return XED_ADDRESS_WIDTH_32b,
        4 => {
            // /union of read flags
            current_block_4 = 10769607207342190892;
        }
        2 => {
            current_block_4 = 10769607207342190892;
        }
        5 => {
            // / defaults to lowercase
            // / union of written flags (includes undefined flags);
            current_block_4 = 11009491163918743684;
        }
        3 => {
            current_block_4 = 11009491163918743684;
        }
        _ => return XED_ADDRESS_WIDTH_INVALID,
    }
    match current_block_4 {
        11009491163918743684 => return XED_ADDRESS_WIDTH_16b,
        _ => return XED_ADDRESS_WIDTH_32b,
    };
}

pub unsafe extern "C" fn xed_state_set_stack_address_width(
    mut p: *mut xed_state_t,
    mut arg_addr_width: xed_address_width_enum_t,
) {
    (*p).stack_addr_width = arg_addr_width;
}

pub unsafe extern "C" fn xed_state_get_stack_address_width(
    mut p: *const xed_state_t,
) -> xed_address_width_enum_t {
    return (*p).stack_addr_width;
}

pub unsafe extern "C" fn xed_encoder_request_operand_order_entries(
    mut p: *mut xed_encoder_request_t,
) -> xed_uint_t {
    return (*p)._n_operand_order as xed_uint_t;
}

pub unsafe extern "C" fn xed_encoder_request_operands_const(
    mut p: *const xed_encoder_request_t,
) -> *const xed_operand_values_t {
    return p;
}

pub unsafe extern "C" fn xed_encoder_request_operands(
    mut p: *mut xed_encoder_request_t,
) -> *mut xed_operand_values_t {
    return p;
}

pub unsafe extern "C" fn xed_disp(
    mut displacement: u64,
    mut displacement_bits: u32,
) -> xed_enc_displacement_t {
    // / Returns the largest enclosing register for any kind of register; This
    /// is mostly useful for GPRs in 32b mode.
    ///@ingroup REGINTFC
    // / length of the output buffer. (bytes) Must be > 25 to start.
    // /< virtual interrupt pending
    // /< The index, dest and mask regs for AVX2 gathers must be different.
    //fwd-decl
    let mut x: xed_enc_displacement_t = xed_enc_displacement_t {
        displacement: 0,
        displacement_bits: 0,
    };
    x.displacement = displacement;
    x.displacement_bits = displacement_bits;
    return x;
}

pub unsafe extern "C" fn xed_relbr(
    mut brdisp: i32,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_BRDISP;
    o.u.brdisp = brdisp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_ptr(
    mut brdisp: i32,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    // / @ingroup OPERANDS
    /// Indicates if the default segment is being used.
    /// @param[in] p   the pointer to the #xed_operand_values_t structure.
    /// @param[in] i   0 or 1, indicating which memory operation.
    /// @return true if the memory operation is using the default segment for
    /// the associated addressing mode base register.
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_PTR;
    o.u.brdisp = brdisp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_reg(mut reg: xed_reg_enum_t) -> xed_encoder_operand_t {
    // / @name branches and far pointers
    //@{
    /// @ingroup ENC
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_REG;
    o.u.reg = reg;
    o.width_bits = 0i32 as u32;
    return o;
}

pub unsafe extern "C" fn xed_imm0(
    mut v: u64,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_IMM0;
    o.u.imm0 = v;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_simm0(
    mut v: i32,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    // / Returns the last element of the enumeration
    /// @return xed_operand_enum_t The last element of the enumeration.
    /// @ingroup ENUM
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_SIMM0;
    o.u.imm0 = v as u64;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_imm1(mut v: u8) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_IMM1;
    o.u.imm1 = v;
    o.width_bits = 8i32 as u32;
    return o;
}

pub unsafe extern "C" fn xed_other(
    mut operand_name: xed_operand_enum_t,
    mut value: i32,
) -> xed_encoder_operand_t {
    // / @ingroup ENC
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_OTHER;
    o.u.s.operand_name = operand_name;
    o.u.s.value = value as u32;
    o.width_bits = 0i32 as u32;
    return o;
}

pub unsafe extern "C" fn xed_seg0(mut seg0: xed_reg_enum_t) -> xed_encoder_operand_t {
    // / @name xed_inst_t Template Operand Read/Written
    //@{
    /// @ingroup DEC
    /// DEPRECATED: Returns the raw R/W action. There are many cases for conditional reads
    /// and writes. See #xed_decoded_inst_operand_action().
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_SEG0;
    o.u.reg = seg0;
    return o;
}

pub unsafe extern "C" fn xed_seg1(mut seg1: xed_reg_enum_t) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_SEG1;
    o.u.reg = seg1;
    return o;
}

pub unsafe extern "C" fn xed_mem_b(
    mut base: xed_reg_enum_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    // / @ingroup DEC
    /// If the operand is read-and-written, conditional reads and conditional writes
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = XED_REG_INVALID;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0i32 as u32;
    o.u.mem.disp.displacement = 0i32 as u64;
    o.u.mem.disp.displacement_bits = 0i32 as u32;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_mem_bd(
    mut base: xed_reg_enum_t,
    mut disp: xed_enc_displacement_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = XED_REG_INVALID;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0i32 as u32;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_mem_bisd(
    mut base: xed_reg_enum_t,
    mut index: xed_reg_enum_t,
    mut scale: xed_uint_t,
    mut disp: xed_enc_displacement_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    // / This function will attempt to encode a NOP of exactly ilen
    /// bytes. If such a NOP is not encodeable, then false will be returned.
    ///
    /// @param array the encoded instruction bytes are stored here
    /// @param  ilen the input length array.
    /// @return success/failure as a #xed_error_enum_t
    /// @ingroup ENC
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = XED_REG_INVALID;
    o.u.mem.index = index;
    o.u.mem.scale = scale;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_mem_gb(
    mut seg: xed_reg_enum_t,
    mut base: xed_reg_enum_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    // / @ingroup OPERANDS
    /// Return true if the first immediate (IMM0) is signed
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = seg;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0i32 as u32;
    o.u.mem.disp.displacement = 0i32 as u64;
    o.u.mem.disp.displacement_bits = 0i32 as u32;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_mem_gbd(
    mut seg: xed_reg_enum_t,
    mut base: xed_reg_enum_t,
    mut disp: xed_enc_displacement_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    // / @ingroup OPERANDS
    /// Return the memory displacement width in BITS
    //<PSEUDO
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = seg;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0i32 as u32;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_mem_gd(
    mut seg: xed_reg_enum_t,
    mut disp: xed_enc_displacement_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = XED_REG_INVALID;
    o.u.mem.seg = seg;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0i32 as u32;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_mem_gbisd(
    mut seg: xed_reg_enum_t,
    mut base: xed_reg_enum_t,
    mut index: xed_reg_enum_t,
    mut scale: xed_uint_t,
    mut disp: xed_enc_displacement_t,
    mut width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_INVALID,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            reg: XED_REG_INVALID,
        },
        width_bits: 0,
    };
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = seg;
    o.u.mem.index = index;
    o.u.mem.scale = scale;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

pub unsafe extern "C" fn xed_addr(
    mut x: *mut xed_encoder_instruction_t,
    mut width_bits: xed_uint_t,
) {
    (*x).effective_address_width = width_bits;
}

pub unsafe extern "C" fn xed_rep(mut x: *mut xed_encoder_instruction_t) {
    (*x).prefixes.s.set_rep(1i32 as u32);
}

pub unsafe extern "C" fn xed_repne(mut x: *mut xed_encoder_instruction_t) {
    (*x).prefixes.s.set_repne(1i32 as u32);
}

pub unsafe extern "C" fn xed_inst0(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    (*inst).noperands = 0i32 as u32;
}

pub unsafe extern "C" fn xed_inst1(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
    mut op0: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    (*inst).operands[0usize] = op0;
    (*inst).noperands = 1i32 as u32;
}
// / @ingroup DEC
/// Return the second immediate.

pub unsafe extern "C" fn xed_inst2(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
    mut op0: xed_encoder_operand_t,
    mut op1: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    (*inst).operands[0usize] = op0;
    (*inst).operands[1usize] = op1;
    (*inst).noperands = 2i32 as u32;
}

pub unsafe extern "C" fn xed_inst3(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
    mut op0: xed_encoder_operand_t,
    mut op1: xed_encoder_operand_t,
    mut op2: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    (*inst).operands[0usize] = op0;
    (*inst).operands[1usize] = op1;
    (*inst).operands[2usize] = op2;
    (*inst).noperands = 3i32 as u32;
}

pub unsafe extern "C" fn xed_inst4(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
    mut op0: xed_encoder_operand_t,
    mut op1: xed_encoder_operand_t,
    mut op2: xed_encoder_operand_t,
    mut op3: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    (*inst).operands[0usize] = op0;
    (*inst).operands[1usize] = op1;
    (*inst).operands[2usize] = op2;
    (*inst).operands[3usize] = op3;
    (*inst).noperands = 4i32 as u32;
}

pub unsafe extern "C" fn xed_inst5(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
    mut op0: xed_encoder_operand_t,
    mut op1: xed_encoder_operand_t,
    mut op2: xed_encoder_operand_t,
    mut op3: xed_encoder_operand_t,
    mut op4: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    (*inst).operands[0usize] = op0;
    (*inst).operands[1usize] = op1;
    (*inst).operands[2usize] = op2;
    (*inst).operands[3usize] = op3;
    (*inst).operands[4usize] = op4;
    (*inst).noperands = 5i32 as u32;
}

pub unsafe extern "C" fn xed_inst(
    mut inst: *mut xed_encoder_instruction_t,
    mut mode: xed_state_t,
    mut iclass: xed_iclass_enum_t,
    mut effective_operand_width: xed_uint_t,
    mut number_of_operands: xed_uint_t,
    mut operand_array: *const xed_encoder_operand_t,
) {
    // / @ingroup DEC
    /// Set the signed immediate a BITS length
    let mut i: xed_uint_t = 0;
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0i32 as u32;
    (*inst).prefixes.i = 0i32 as u32;
    i = 0i32 as xed_uint_t;
    while i < number_of_operands {
        (*inst).operands[i as usize] = *operand_array.offset(i as isize);
        i = i.wrapping_add(1)
    }
    (*inst).noperands = number_of_operands;
}

pub unsafe extern "C" fn xed_decoded_inst_valid(mut p: *const xed_decoded_inst_t) -> xed_bool_t {
    return ((*p)._inst != 0 as *const xed_inst_t) as libc::c_int as xed_bool_t;
}

pub unsafe extern "C" fn xed_decoded_inst_inst(
    mut p: *const xed_decoded_inst_t,
) -> *const xed_inst_t {
    return (*p)._inst;
}

pub unsafe extern "C" fn xed_decoded_inst_get_category(
    mut p: *const xed_decoded_inst_t,
) -> xed_category_enum_t {
    return xed_inst_category((*p)._inst);
}

pub unsafe extern "C" fn xed_decoded_inst_get_extension(
    mut p: *const xed_decoded_inst_t,
) -> xed_extension_enum_t {
    return xed_inst_extension((*p)._inst);
}

pub unsafe extern "C" fn xed_decoded_inst_get_isa_set(
    p: *const xed_decoded_inst_t,
) -> xed_isa_set_enum_t {
    return xed_inst_isa_set((*p)._inst);
}

pub unsafe extern "C" fn xed_decoded_inst_get_iclass(
    mut p: *const xed_decoded_inst_t,
) -> xed_iclass_enum_t {
    return xed_inst_iclass((*p)._inst);
}

pub unsafe extern "C" fn xed_decoded_inst_operands_const(
    mut p: *const xed_decoded_inst_t,
) -> *const xed_operand_values_t {
    return p;
}

pub unsafe extern "C" fn xed_decoded_inst_operands(
    mut p: *mut xed_decoded_inst_t,
) -> *mut xed_operand_values_t {
    return p;
}

pub unsafe extern "C" fn xed_decoded_inst_noperands(
    mut p: *const xed_decoded_inst_t,
) -> libc::c_uint {
    let mut noperands: libc::c_uint = xed_inst_noperands(xed_decoded_inst_inst(p));
    return noperands;
}

pub unsafe extern "C" fn xed_decoded_inst_set_mode(
    mut p: *mut xed_decoded_inst_t,
    mut mmode: xed_machine_mode_enum_t,
    mut stack_addr_width: xed_address_width_enum_t,
) {
    // rflags info -- index in to the 2 tables of flags information.
    // If _flag_complex is true, then the data are in the
    // xed_flags_complex_table[]. Otherwise, the data are in the
    // xed_flags_simple_table[].
    let mut dstate: xed_state_t = xed_state_s {
        mmode: XED_MACHINE_MODE_INVALID,
        stack_addr_width: XED_ADDRESS_WIDTH_INVALID,
    };
    dstate.mmode = mmode;
    dstate.stack_addr_width = stack_addr_width;
    xed_operand_values_set_mode(p, &mut dstate);
}

pub unsafe extern "C" fn xed_decoded_inst_get_length(
    mut p: *const xed_decoded_inst_t,
) -> xed_uint_t {
    return (*p)._decoded_length as xed_uint_t;
}

pub unsafe extern "C" fn xed_decoded_inst_get_byte(
    mut p: *const xed_decoded_inst_t,
    mut byte_index: xed_uint_t,
) -> u8 {
    let mut out: u8 = *(*p)._byte_array._dec.offset(byte_index as isize);
    return out;
}

pub unsafe extern "C" fn xed_decoded_inst_get_machine_mode_bits(
    mut p: *const xed_decoded_inst_t,
) -> xed_uint_t {
    //<PSEUDO
    let mut mode: u8 = xed3_operand_get_mode(p) as u8;
    if mode as libc::c_int == 2i32 {
        return 64i32 as xed_uint_t;
    }
    if mode as libc::c_int == 1i32 {
        return 32i32 as xed_uint_t;
    }
    return 16i32 as xed_uint_t;
}

pub unsafe extern "C" fn xed_decoded_inst_get_stack_address_mode_bits(
    mut p: *const xed_decoded_inst_t,
) -> xed_uint_t {
    //<PSEUDO
    let mut smode: u8 = xed3_operand_get_smode(p) as u8;
    if smode as libc::c_int == 2i32 {
        return 64i32 as xed_uint_t;
    }
    if smode as libc::c_int == 1i32 {
        return 32i32 as xed_uint_t;
    }
    return 16i32 as xed_uint_t;
}

pub unsafe extern "C" fn xed_decoded_inst_get_input_chip(
    mut p: *const xed_decoded_inst_t,
) -> xed_chip_enum_t {
    return xed3_operand_get_chip(p);
}

pub unsafe extern "C" fn xed_decoded_inst_set_input_chip(
    mut p: *mut xed_decoded_inst_t,
    mut chip: xed_chip_enum_t,
) {
    xed3_operand_set_chip(p, chip);
}

pub unsafe extern "C" fn xed_decoded_inst_get_iform_enum(
    mut p: *const xed_decoded_inst_t,
) -> xed_iform_enum_t {
    return xed_inst_iform_enum((*p)._inst);
}

pub unsafe extern "C" fn xed_decoded_inst_get_iform_enum_dispatch(
    mut p: *const xed_decoded_inst_t,
) -> libc::c_uint {
    return (xed_inst_iform_enum((*p)._inst) as libc::c_uint)
        .wrapping_sub(xed_iform_first_per_iclass(xed_inst_iclass((*p)._inst)));
}

pub unsafe extern "C" fn xed_decoded_inst_get_second_immediate(
    mut p: *const xed_decoded_inst_t,
) -> u8 {
    return xed3_operand_get_uimm1(p);
}

pub unsafe extern "C" fn xed_decoded_inst_get_user_data(
    mut p: *mut xed_decoded_inst_t,
) -> u64 {
    return (*p).u.user_data;
}

pub unsafe extern "C" fn xed_decoded_inst_set_user_data(
    mut p: *mut xed_decoded_inst_t,
    mut new_value: u64,
) {
    (*p).u.user_data = new_value;
}
