#include "xed.h"

// Static wrappers

uint64_t xed_make_uint64_xed_sys_inline(uint32_t hi, uint32_t lo) { return xed_make_uint64(hi, lo); }
int64_t xed_make_int64_xed_sys_inline(uint32_t hi, uint32_t lo) { return xed_make_int64(hi, lo); }
xed_iclass_enum_t xed_iform_to_iclass_xed_sys_inline(xed_iform_enum_t iform) { return xed_iform_to_iclass(iform); }
xed_operand_enum_t xed_operand_name_xed_sys_inline(const xed_operand_t *p) { return xed_operand_name(p); }
xed_operand_visibility_enum_t xed_operand_operand_visibility_xed_sys_inline(const xed_operand_t *p) { return xed_operand_operand_visibility(p); }
xed_operand_type_enum_t xed_operand_type_xed_sys_inline(const xed_operand_t *p) { return xed_operand_type(p); }
xed_operand_element_xtype_enum_t xed_operand_xtype_xed_sys_inline(const xed_operand_t *p) { return xed_operand_xtype(p); }
xed_operand_width_enum_t xed_operand_width_xed_sys_inline(const xed_operand_t *p) { return xed_operand_width(p); }
xed_nonterminal_enum_t xed_operand_nonterminal_name_xed_sys_inline(const xed_operand_t *p) { return xed_operand_nonterminal_name(p); }
xed_reg_enum_t xed_operand_reg_xed_sys_inline(const xed_operand_t *p) { return xed_operand_reg(p); }
xed_uint_t xed_operand_template_is_register_xed_sys_inline(const xed_operand_t *p) { return xed_operand_template_is_register(p); }
uint32_t xed_operand_imm_xed_sys_inline(const xed_operand_t *p) { return xed_operand_imm(p); }
xed_uint_t xed_operand_is_register_xed_sys_inline(xed_operand_enum_t name) { return xed_operand_is_register(name); }
xed_uint_t xed_operand_is_memory_addressing_register_xed_sys_inline(xed_operand_enum_t name) { return xed_operand_is_memory_addressing_register(name); }
xed_operand_action_enum_t xed_operand_rw_xed_sys_inline(const xed_operand_t *p) { return xed_operand_rw(p); }
xed_iform_enum_t xed_inst_iform_enum_xed_sys_inline(const xed_inst_t *p) { return xed_inst_iform_enum(p); }
xed_iclass_enum_t xed_inst_iclass_xed_sys_inline(const xed_inst_t *p) { return xed_inst_iclass(p); }
xed_category_enum_t xed_inst_category_xed_sys_inline(const xed_inst_t *p) { return xed_inst_category(p); }
xed_extension_enum_t xed_inst_extension_xed_sys_inline(const xed_inst_t *p) { return xed_inst_extension(p); }
xed_isa_set_enum_t xed_inst_isa_set_xed_sys_inline(const xed_inst_t *p) { return xed_inst_isa_set(p); }
unsigned int xed_inst_noperands_xed_sys_inline(const xed_inst_t *p) { return xed_inst_noperands(p); }
xed_exception_enum_t xed_inst_exception_xed_sys_inline(const xed_inst_t *p) { return xed_inst_exception(p); }
unsigned int xed_flag_set_mask_xed_sys_inline(const xed_flag_set_t *p) { return xed_flag_set_mask(p); }
xed_bits_t xed3_operand_get_seg_ovd_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_seg_ovd(d); }
void xed3_operand_set_seg_ovd_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_seg_ovd(d, opval); }
xed_bits_t xed3_operand_get_hint_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_hint(d); }
void xed3_operand_set_hint_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_hint(d, opval); }
xed_bits_t xed3_operand_get_encode_force_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_encode_force(d); }
void xed3_operand_set_encode_force_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_encode_force(d, opval); }
xed_bits_t xed3_operand_get_lock_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_lock(d); }
void xed3_operand_set_lock_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_lock(d, opval); }
xed_bits_t xed3_operand_get_need_memdisp_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_need_memdisp(d); }
void xed3_operand_set_need_memdisp_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_need_memdisp(d, opval); }
int64_t xed3_operand_get_disp_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_disp(d); }
void xed3_operand_set_disp_xed_sys_inline(xed_decoded_inst_t *d, int64_t opval) { xed3_operand_set_disp(d, opval); }
uint8_t xed3_operand_get_disp_width_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_disp_width(d); }
void xed3_operand_set_disp_width_xed_sys_inline(xed_decoded_inst_t *d, uint8_t opval) { xed3_operand_set_disp_width(d, opval); }
uint8_t xed3_operand_get_brdisp_width_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_brdisp_width(d); }
void xed3_operand_set_brdisp_width_xed_sys_inline(xed_decoded_inst_t *d, uint8_t opval) { xed3_operand_set_brdisp_width(d, opval); }
xed_bits_t xed3_operand_get_df32_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_df32(d); }
void xed3_operand_set_df32_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_df32(d, opval); }
xed_bits_t xed3_operand_get_df64_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_df64(d); }
void xed3_operand_set_df64_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_df64(d, opval); }
xed_bits_t xed3_operand_get_norex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_norex(d); }
void xed3_operand_set_norex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_norex(d, opval); }
xed_bits_t xed3_operand_get_norex2_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_norex2(d); }
void xed3_operand_set_norex2_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_norex2(d, opval); }
xed_bits_t xed3_operand_get_needrex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_needrex(d); }
void xed3_operand_set_needrex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_needrex(d, opval); }
xed_bits_t xed3_operand_get_rex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rex(d); }
void xed3_operand_set_rex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rex(d, opval); }
xed_bits_t xed3_operand_get_rexw_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexw(d); }
void xed3_operand_set_rexw_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexw(d, opval); }
xed_bits_t xed3_operand_get_rexr_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexr(d); }
void xed3_operand_set_rexr_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexr(d, opval); }
xed_bits_t xed3_operand_get_rexx_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexx(d); }
void xed3_operand_set_rexx_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexx(d, opval); }
xed_bits_t xed3_operand_get_rexb_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexb(d); }
void xed3_operand_set_rexb_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexb(d, opval); }
xed_bits_t xed3_operand_get_rex2_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rex2(d); }
void xed3_operand_set_rex2_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rex2(d, opval); }
xed_bits_t xed3_operand_get_rexb4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexb4(d); }
void xed3_operand_set_rexb4_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexb4(d, opval); }
xed_bits_t xed3_operand_get_rexx4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexx4(d); }
void xed3_operand_set_rexx4_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexx4(d, opval); }
xed_bits_t xed3_operand_get_rexr4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rexr4(d); }
void xed3_operand_set_rexr4_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rexr4(d, opval); }
xed_bits_t xed3_operand_get_has_egpr_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_has_egpr(d); }
void xed3_operand_set_has_egpr_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_has_egpr(d, opval); }
xed_bits_t xed3_operand_get_rep_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rep(d); }
void xed3_operand_set_rep_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rep(d, opval); }
xed_bits_t xed3_operand_get_osz_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_osz(d); }
void xed3_operand_set_osz_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_osz(d, opval); }
xed_bits_t xed3_operand_get_prefix66_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_prefix66(d); }
void xed3_operand_set_prefix66_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_prefix66(d, opval); }
xed_bits_t xed3_operand_get_asz_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_asz(d); }
void xed3_operand_set_asz_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_asz(d, opval); }
xed_bits_t xed3_operand_get_eosz_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_eosz(d); }
void xed3_operand_set_eosz_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_eosz(d, opval); }
xed_bits_t xed3_operand_get_easz_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_easz(d); }
void xed3_operand_set_easz_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_easz(d, opval); }
xed_bits_t xed3_operand_get_skip_osz_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_skip_osz(d); }
void xed3_operand_set_skip_osz_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_skip_osz(d, opval); }
xed_bits_t xed3_operand_get_mod_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mod(d); }
void xed3_operand_set_mod_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mod(d, opval); }
xed_bits_t xed3_operand_get_reg_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg(d); }
void xed3_operand_set_reg_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_reg(d, opval); }
xed_bits_t xed3_operand_get_srm_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_srm(d); }
void xed3_operand_set_srm_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_srm(d, opval); }
xed_bits_t xed3_operand_get_rm_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_rm(d); }
void xed3_operand_set_rm_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_rm(d, opval); }
xed_bits_t xed3_operand_get_realmode_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_realmode(d); }
void xed3_operand_set_realmode_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_realmode(d, opval); }
xed_chip_enum_t xed3_operand_get_chip_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_chip(d); }
void xed3_operand_set_chip_xed_sys_inline(xed_decoded_inst_t *d, xed_chip_enum_t opval) { xed3_operand_set_chip(d, opval); }
xed_bits_t xed3_operand_get_mode_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mode(d); }
void xed3_operand_set_mode_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mode(d, opval); }
xed_bits_t xed3_operand_get_smode_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_smode(d); }
void xed3_operand_set_smode_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_smode(d, opval); }
xed_bits_t xed3_operand_get_modep5_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_modep5(d); }
void xed3_operand_set_modep5_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_modep5(d, opval); }
xed_bits_t xed3_operand_get_modep55c_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_modep55c(d); }
void xed3_operand_set_modep55c_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_modep55c(d, opval); }
xed_bits_t xed3_operand_get_p4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_p4(d); }
void xed3_operand_set_p4_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_p4(d, opval); }
xed_bits_t xed3_operand_get_lzcnt_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_lzcnt(d); }
void xed3_operand_set_lzcnt_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_lzcnt(d, opval); }
xed_bits_t xed3_operand_get_tzcnt_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_tzcnt(d); }
void xed3_operand_set_tzcnt_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_tzcnt(d, opval); }
xed_bits_t xed3_operand_get_mode_first_prefix_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mode_first_prefix(d); }
void xed3_operand_set_mode_first_prefix_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mode_first_prefix(d, opval); }
xed_bits_t xed3_operand_get_mode_short_ud0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mode_short_ud0(d); }
void xed3_operand_set_mode_short_ud0_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mode_short_ud0(d, opval); }
xed_bits_t xed3_operand_get_imm0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_imm0(d); }
void xed3_operand_set_imm0_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_imm0(d, opval); }
xed_bits_t xed3_operand_get_imm1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_imm1(d); }
void xed3_operand_set_imm1_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_imm1(d, opval); }
xed_bits_t xed3_operand_get_imm0signed_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_imm0signed(d); }
void xed3_operand_set_imm0signed_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_imm0signed(d, opval); }
uint64_t xed3_operand_get_uimm0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_uimm0(d); }
void xed3_operand_set_uimm0_xed_sys_inline(xed_decoded_inst_t *d, uint64_t opval) { xed3_operand_set_uimm0(d, opval); }
uint8_t xed3_operand_get_uimm1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_uimm1(d); }
void xed3_operand_set_uimm1_xed_sys_inline(xed_decoded_inst_t *d, uint8_t opval) { xed3_operand_set_uimm1(d, opval); }
uint8_t xed3_operand_get_imm_width_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_imm_width(d); }
void xed3_operand_set_imm_width_xed_sys_inline(xed_decoded_inst_t *d, uint8_t opval) { xed3_operand_set_imm_width(d, opval); }
xed_bits_t xed3_operand_get_using_default_segment0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_using_default_segment0(d); }
void xed3_operand_set_using_default_segment0_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_using_default_segment0(d, opval); }
xed_bits_t xed3_operand_get_using_default_segment1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_using_default_segment1(d); }
void xed3_operand_set_using_default_segment1_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_using_default_segment1(d, opval); }
xed_bits_t xed3_operand_get_default_seg_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_default_seg(d); }
void xed3_operand_set_default_seg_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_default_seg(d, opval); }
xed_reg_enum_t xed3_operand_get_seg0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_seg0(d); }
void xed3_operand_set_seg0_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_seg0(d, opval); }
xed_reg_enum_t xed3_operand_get_base0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_base0(d); }
void xed3_operand_set_base0_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_base0(d, opval); }
xed_reg_enum_t xed3_operand_get_index_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_index(d); }
void xed3_operand_set_index_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_index(d, opval); }
xed_bits_t xed3_operand_get_scale_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_scale(d); }
void xed3_operand_set_scale_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_scale(d, opval); }
xed_bits_t xed3_operand_get_need_sib_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_need_sib(d); }
void xed3_operand_set_need_sib_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_need_sib(d, opval); }
xed_bits_t xed3_operand_get_sibscale_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_sibscale(d); }
void xed3_operand_set_sibscale_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_sibscale(d, opval); }
xed_bits_t xed3_operand_get_sibbase_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_sibbase(d); }
void xed3_operand_set_sibbase_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_sibbase(d, opval); }
xed_bits_t xed3_operand_get_sibindex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_sibindex(d); }
void xed3_operand_set_sibindex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_sibindex(d, opval); }
xed_reg_enum_t xed3_operand_get_seg1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_seg1(d); }
void xed3_operand_set_seg1_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_seg1(d, opval); }
xed_reg_enum_t xed3_operand_get_base1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_base1(d); }
void xed3_operand_set_base1_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_base1(d, opval); }
xed_bits_t xed3_operand_get_mem0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mem0(d); }
void xed3_operand_set_mem0_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mem0(d, opval); }
xed_bits_t xed3_operand_get_mem1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mem1(d); }
void xed3_operand_set_mem1_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mem1(d, opval); }
uint16_t xed3_operand_get_mem_width_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mem_width(d); }
void xed3_operand_set_mem_width_xed_sys_inline(xed_decoded_inst_t *d, uint16_t opval) { xed3_operand_set_mem_width(d, opval); }
xed_bits_t xed3_operand_get_agen_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_agen(d); }
void xed3_operand_set_agen_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_agen(d, opval); }
xed_bits_t xed3_operand_get_relbr_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_relbr(d); }
void xed3_operand_set_relbr_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_relbr(d, opval); }
xed_bits_t xed3_operand_get_absbr_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_absbr(d); }
void xed3_operand_set_absbr_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_absbr(d, opval); }
xed_bits_t xed3_operand_get_ptr_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_ptr(d); }
void xed3_operand_set_ptr_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_ptr(d, opval); }
xed_reg_enum_t xed3_operand_get_reg0_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg0(d); }
void xed3_operand_set_reg0_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg0(d, opval); }
xed_reg_enum_t xed3_operand_get_reg1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg1(d); }
void xed3_operand_set_reg1_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg1(d, opval); }
xed_reg_enum_t xed3_operand_get_reg2_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg2(d); }
void xed3_operand_set_reg2_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg2(d, opval); }
xed_reg_enum_t xed3_operand_get_reg3_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg3(d); }
void xed3_operand_set_reg3_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg3(d, opval); }
xed_reg_enum_t xed3_operand_get_reg4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg4(d); }
void xed3_operand_set_reg4_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg4(d, opval); }
xed_reg_enum_t xed3_operand_get_reg5_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg5(d); }
void xed3_operand_set_reg5_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg5(d, opval); }
xed_reg_enum_t xed3_operand_get_reg6_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg6(d); }
void xed3_operand_set_reg6_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg6(d, opval); }
xed_reg_enum_t xed3_operand_get_reg7_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg7(d); }
void xed3_operand_set_reg7_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg7(d, opval); }
xed_reg_enum_t xed3_operand_get_reg8_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg8(d); }
void xed3_operand_set_reg8_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg8(d, opval); }
xed_reg_enum_t xed3_operand_get_reg9_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_reg9(d); }
void xed3_operand_set_reg9_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_reg9(d, opval); }
xed_reg_enum_t xed3_operand_get_outreg_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_outreg(d); }
void xed3_operand_set_outreg_xed_sys_inline(xed_decoded_inst_t *d, xed_reg_enum_t opval) { xed3_operand_set_outreg(d, opval); }
xed_bits_t xed3_operand_get_encoder_preferred_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_encoder_preferred(d); }
void xed3_operand_set_encoder_preferred_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_encoder_preferred(d, opval); }
xed_error_enum_t xed3_operand_get_error_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_error(d); }
void xed3_operand_set_error_xed_sys_inline(xed_decoded_inst_t *d, xed_error_enum_t opval) { xed3_operand_set_error(d, opval); }
xed_iclass_enum_t xed3_operand_get_iclass_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_iclass(d); }
void xed3_operand_set_iclass_xed_sys_inline(xed_decoded_inst_t *d, xed_iclass_enum_t opval) { xed3_operand_set_iclass(d, opval); }
xed_bits_t xed3_operand_get_nelem_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nelem(d); }
void xed3_operand_set_nelem_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nelem(d, opval); }
xed_bits_t xed3_operand_get_element_size_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_element_size(d); }
void xed3_operand_set_element_size_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_element_size(d, opval); }
xed_bits_t xed3_operand_get_map_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_map(d); }
void xed3_operand_set_map_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_map(d, opval); }
xed_bits_t xed3_operand_get_out_of_bytes_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_out_of_bytes(d); }
void xed3_operand_set_out_of_bytes_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_out_of_bytes(d, opval); }
xed_bits_t xed3_operand_get_first_f2f3_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_first_f2f3(d); }
void xed3_operand_set_first_f2f3_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_first_f2f3(d, opval); }
xed_bits_t xed3_operand_get_last_f2f3_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_last_f2f3(d); }
void xed3_operand_set_last_f2f3_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_last_f2f3(d, opval); }
xed_bits_t xed3_operand_get_ild_f2_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_ild_f2(d); }
void xed3_operand_set_ild_f2_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_ild_f2(d, opval); }
xed_bits_t xed3_operand_get_ild_f3_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_ild_f3(d); }
void xed3_operand_set_ild_f3_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_ild_f3(d, opval); }
xed_bits_t xed3_operand_get_max_bytes_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_max_bytes(d); }
void xed3_operand_set_max_bytes_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_max_bytes(d, opval); }
xed_bits_t xed3_operand_get_ild_seg_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_ild_seg(d); }
void xed3_operand_set_ild_seg_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_ild_seg(d, opval); }
xed_bits_t xed3_operand_get_nseg_prefixes_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nseg_prefixes(d); }
void xed3_operand_set_nseg_prefixes_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nseg_prefixes(d, opval); }
xed_bits_t xed3_operand_get_nrexes_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nrexes(d); }
void xed3_operand_set_nrexes_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nrexes(d, opval); }
xed_bits_t xed3_operand_get_nprefixes_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nprefixes(d); }
void xed3_operand_set_nprefixes_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nprefixes(d, opval); }
xed_bits_t xed3_operand_get_nominal_opcode_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nominal_opcode(d); }
void xed3_operand_set_nominal_opcode_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nominal_opcode(d, opval); }
xed_bits_t xed3_operand_get_pos_nominal_opcode_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_pos_nominal_opcode(d); }
void xed3_operand_set_pos_nominal_opcode_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_pos_nominal_opcode(d, opval); }
xed_bits_t xed3_operand_get_has_modrm_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_has_modrm(d); }
void xed3_operand_set_has_modrm_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_has_modrm(d, opval); }
xed_bits_t xed3_operand_get_has_sib_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_has_sib(d); }
void xed3_operand_set_has_sib_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_has_sib(d, opval); }
xed_bits_t xed3_operand_get_pos_modrm_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_pos_modrm(d); }
void xed3_operand_set_pos_modrm_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_pos_modrm(d, opval); }
xed_bits_t xed3_operand_get_pos_sib_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_pos_sib(d); }
void xed3_operand_set_pos_sib_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_pos_sib(d, opval); }
xed_bits_t xed3_operand_get_pos_disp_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_pos_disp(d); }
void xed3_operand_set_pos_disp_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_pos_disp(d, opval); }
xed_bits_t xed3_operand_get_pos_imm_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_pos_imm(d); }
void xed3_operand_set_pos_imm_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_pos_imm(d, opval); }
xed_bits_t xed3_operand_get_pos_imm1_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_pos_imm1(d); }
void xed3_operand_set_pos_imm1_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_pos_imm1(d, opval); }
xed_bits_t xed3_operand_get_imm1_bytes_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_imm1_bytes(d); }
void xed3_operand_set_imm1_bytes_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_imm1_bytes(d, opval); }
xed_bits_t xed3_operand_get_modrm_byte_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_modrm_byte(d); }
void xed3_operand_set_modrm_byte_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_modrm_byte(d, opval); }
xed_bits_t xed3_operand_get_esrc_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_esrc(d); }
void xed3_operand_set_esrc_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_esrc(d, opval); }
xed_bits_t xed3_operand_get_vexvalid_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vexvalid(d); }
void xed3_operand_set_vexvalid_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vexvalid(d, opval); }
xed_bits_t xed3_operand_get_dummy_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_dummy(d); }
void xed3_operand_set_dummy_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_dummy(d, opval); }
xed_bits_t xed3_operand_get_no_evex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_no_evex(d); }
void xed3_operand_set_no_evex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_no_evex(d, opval); }
xed_bits_t xed3_operand_get_no_vex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_no_vex(d); }
void xed3_operand_set_no_vex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_no_vex(d, opval); }
xed_bits_t xed3_operand_get_no_apx_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_no_apx(d); }
void xed3_operand_set_no_apx_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_no_apx(d, opval); }
xed_bits_t xed3_operand_get_amd3dnow_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_amd3dnow(d); }
void xed3_operand_set_amd3dnow_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_amd3dnow(d, opval); }
xed_bits_t xed3_operand_get_mpxmode_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mpxmode(d); }
void xed3_operand_set_mpxmode_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mpxmode(d, opval); }
xed_bits_t xed3_operand_get_cet_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_cet(d); }
void xed3_operand_set_cet_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_cet(d, opval); }
xed_bits_t xed3_operand_get_cldemote_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_cldemote(d); }
void xed3_operand_set_cldemote_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_cldemote(d, opval); }
xed_bits_t xed3_operand_get_vexdest3_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vexdest3(d); }
void xed3_operand_set_vexdest3_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vexdest3(d, opval); }
xed_bits_t xed3_operand_get_vexdest210_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vexdest210(d); }
void xed3_operand_set_vexdest210_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vexdest210(d, opval); }
xed_bits_t xed3_operand_get_vl_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vl(d); }
void xed3_operand_set_vl_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vl(d, opval); }
xed_bits_t xed3_operand_get_vex_prefix_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vex_prefix(d); }
void xed3_operand_set_vex_prefix_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vex_prefix(d, opval); }
xed_bits_t xed3_operand_get_vex_c4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vex_c4(d); }
void xed3_operand_set_vex_c4_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vex_c4(d, opval); }
xed_bits_t xed3_operand_get_bcast_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_bcast(d); }
void xed3_operand_set_bcast_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_bcast(d, opval); }
xed_bits_t xed3_operand_get_must_use_evex_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_must_use_evex(d); }
void xed3_operand_set_must_use_evex_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_must_use_evex(d, opval); }
xed_bits_t xed3_operand_get_zeroing_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_zeroing(d); }
void xed3_operand_set_zeroing_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_zeroing(d, opval); }
xed_bits_t xed3_operand_get_llrc_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_llrc(d); }
void xed3_operand_set_llrc_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_llrc(d, opval); }
xed_bits_t xed3_operand_get_bcrc_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_bcrc(d); }
void xed3_operand_set_bcrc_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_bcrc(d, opval); }
xed_bits_t xed3_operand_get_vexdest4_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vexdest4(d); }
void xed3_operand_set_vexdest4_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vexdest4(d, opval); }
xed_bits_t xed3_operand_get_mask_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_mask(d); }
void xed3_operand_set_mask_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_mask(d, opval); }
xed_bits_t xed3_operand_get_roundc_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_roundc(d); }
void xed3_operand_set_roundc_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_roundc(d, opval); }
xed_bits_t xed3_operand_get_sae_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_sae(d); }
void xed3_operand_set_sae_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_sae(d, opval); }
xed_bits_t xed3_operand_get_vl_ign_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_vl_ign(d); }
void xed3_operand_set_vl_ign_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_vl_ign(d, opval); }
xed_bits_t xed3_operand_get_ubit_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_ubit(d); }
void xed3_operand_set_ubit_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_ubit(d, opval); }
xed_bits_t xed3_operand_get_wbnoinvd_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_wbnoinvd(d); }
void xed3_operand_set_wbnoinvd_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_wbnoinvd(d, opval); }
xed_bits_t xed3_operand_get_evvspace_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_evvspace(d); }
void xed3_operand_set_evvspace_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_evvspace(d, opval); }
xed_bits_t xed3_operand_get_nd_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nd(d); }
void xed3_operand_set_nd_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nd(d, opval); }
xed_bits_t xed3_operand_get_nf_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_nf(d); }
void xed3_operand_set_nf_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_nf(d, opval); }
xed_bits_t xed3_operand_get_scc_xed_sys_inline(const xed_decoded_inst_t *d) { return xed3_operand_get_scc(d); }
void xed3_operand_set_scc_xed_sys_inline(xed_decoded_inst_t *d, xed_bits_t opval) { xed3_operand_set_scc(d, opval); }
void xed_state_init_xed_sys_inline(xed_state_t *p, xed_machine_mode_enum_t arg_mmode, xed_address_width_enum_t arg_ignored, xed_address_width_enum_t arg_stack_addr_width) { xed_state_init(p, arg_mmode, arg_ignored, arg_stack_addr_width); }
void xed_state_init2_xed_sys_inline(xed_state_t *p, xed_machine_mode_enum_t arg_mmode, xed_address_width_enum_t arg_stack_addr_width) { xed_state_init2(p, arg_mmode, arg_stack_addr_width); }
void xed_state_zero_xed_sys_inline(xed_state_t *p) { xed_state_zero(p); }
xed_machine_mode_enum_t xed_state_get_machine_mode_xed_sys_inline(const xed_state_t *p) { return xed_state_get_machine_mode(p); }
xed_bool_t xed_state_long64_mode_xed_sys_inline(const xed_state_t *p) { return xed_state_long64_mode(p); }
xed_bool_t xed_state_real_mode_xed_sys_inline(const xed_state_t *p) { return xed_state_real_mode(p); }
xed_bool_t xed_state_mode_width_16_xed_sys_inline(const xed_state_t *p) { return xed_state_mode_width_16(p); }
xed_bool_t xed_state_mode_width_32_xed_sys_inline(const xed_state_t *p) { return xed_state_mode_width_32(p); }
void xed_state_set_machine_mode_xed_sys_inline(xed_state_t *p, xed_machine_mode_enum_t arg_mode) { xed_state_set_machine_mode(p, arg_mode); }
xed_address_width_enum_t xed_state_get_address_width_xed_sys_inline(const xed_state_t *p) { return xed_state_get_address_width(p); }
void xed_state_set_stack_address_width_xed_sys_inline(xed_state_t *p, xed_address_width_enum_t arg_addr_width) { xed_state_set_stack_address_width(p, arg_addr_width); }
xed_address_width_enum_t xed_state_get_stack_address_width_xed_sys_inline(const xed_state_t *p) { return xed_state_get_stack_address_width(p); }
xed_uint_t xed_encoder_request_operand_order_entries_xed_sys_inline(xed_encoder_request_t *p) { return xed_encoder_request_operand_order_entries(p); }
const xed_operand_values_t * xed_encoder_request_operands_const_xed_sys_inline(const xed_encoder_request_t *p) { return xed_encoder_request_operands_const(p); }
xed_operand_values_t * xed_encoder_request_operands_xed_sys_inline(xed_encoder_request_t *p) { return xed_encoder_request_operands(p); }
xed_enc_displacement_t xed_disp_xed_sys_inline(int64_t displacement, uint32_t displacement_bits) { return xed_disp(displacement, displacement_bits); }
xed_encoder_operand_t xed_relbr_xed_sys_inline(int32_t brdisp, xed_uint_t width_bits) { return xed_relbr(brdisp, width_bits); }
xed_encoder_operand_t xed_absbr_xed_sys_inline(int32_t brdisp, xed_uint_t width_bits) { return xed_absbr(brdisp, width_bits); }
xed_encoder_operand_t xed_ptr_xed_sys_inline(int32_t brdisp, xed_uint_t width_bits) { return xed_ptr(brdisp, width_bits); }
xed_encoder_operand_t xed_reg_xed_sys_inline(xed_reg_enum_t reg) { return xed_reg(reg); }
xed_encoder_operand_t xed_imm0_xed_sys_inline(uint64_t v, xed_uint_t width_bits) { return xed_imm0(v, width_bits); }
xed_encoder_operand_t xed_simm0_xed_sys_inline(int32_t v, xed_uint_t width_bits) { return xed_simm0(v, width_bits); }
xed_encoder_operand_t xed_imm1_xed_sys_inline(uint8_t v) { return xed_imm1(v); }
xed_encoder_operand_t xed_other_xed_sys_inline(xed_operand_enum_t operand_name, int32_t value) { return xed_other(operand_name, value); }
xed_encoder_operand_t xed_seg0_xed_sys_inline(xed_reg_enum_t seg0) { return xed_seg0(seg0); }
xed_encoder_operand_t xed_seg1_xed_sys_inline(xed_reg_enum_t seg1) { return xed_seg1(seg1); }
xed_encoder_operand_t xed_mem_b_xed_sys_inline(xed_reg_enum_t base, xed_uint_t width_bits) { return xed_mem_b(base, width_bits); }
xed_encoder_operand_t xed_mem_bd_xed_sys_inline(xed_reg_enum_t base, xed_enc_displacement_t disp, xed_uint_t width_bits) { return xed_mem_bd(base, disp, width_bits); }
xed_encoder_operand_t xed_mem_bisd_xed_sys_inline(xed_reg_enum_t base, xed_reg_enum_t index, xed_uint_t scale, xed_enc_displacement_t disp, xed_uint_t width_bits) { return xed_mem_bisd(base, index, scale, disp, width_bits); }
xed_encoder_operand_t xed_mem_gb_xed_sys_inline(xed_reg_enum_t seg, xed_reg_enum_t base, xed_uint_t width_bits) { return xed_mem_gb(seg, base, width_bits); }
xed_encoder_operand_t xed_mem_gbd_xed_sys_inline(xed_reg_enum_t seg, xed_reg_enum_t base, xed_enc_displacement_t disp, xed_uint_t width_bits) { return xed_mem_gbd(seg, base, disp, width_bits); }
xed_encoder_operand_t xed_mem_gd_xed_sys_inline(xed_reg_enum_t seg, xed_enc_displacement_t disp, xed_uint_t width_bits) { return xed_mem_gd(seg, disp, width_bits); }
xed_encoder_operand_t xed_mem_gbisd_xed_sys_inline(xed_reg_enum_t seg, xed_reg_enum_t base, xed_reg_enum_t index, xed_uint_t scale, xed_enc_displacement_t disp, xed_uint_t width_bits) { return xed_mem_gbisd(seg, base, index, scale, disp, width_bits); }
void xed_addr_xed_sys_inline(xed_encoder_instruction_t *x, xed_uint_t width_bits) { xed_addr(x, width_bits); }
void xed_rep_xed_sys_inline(xed_encoder_instruction_t *x) { xed_rep(x); }
void xed_repne_xed_sys_inline(xed_encoder_instruction_t *x) { xed_repne(x); }
void xed_inst0_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width) { xed_inst0(inst, mode, iclass, effective_operand_width); }
void xed_inst1_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width, xed_encoder_operand_t op0) { xed_inst1(inst, mode, iclass, effective_operand_width, op0); }
void xed_inst2_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width, xed_encoder_operand_t op0, xed_encoder_operand_t op1) { xed_inst2(inst, mode, iclass, effective_operand_width, op0, op1); }
void xed_inst3_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width, xed_encoder_operand_t op0, xed_encoder_operand_t op1, xed_encoder_operand_t op2) { xed_inst3(inst, mode, iclass, effective_operand_width, op0, op1, op2); }
void xed_inst4_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width, xed_encoder_operand_t op0, xed_encoder_operand_t op1, xed_encoder_operand_t op2, xed_encoder_operand_t op3) { xed_inst4(inst, mode, iclass, effective_operand_width, op0, op1, op2, op3); }
void xed_inst5_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width, xed_encoder_operand_t op0, xed_encoder_operand_t op1, xed_encoder_operand_t op2, xed_encoder_operand_t op3, xed_encoder_operand_t op4) { xed_inst5(inst, mode, iclass, effective_operand_width, op0, op1, op2, op3, op4); }
void xed_inst_xed_sys_inline(xed_encoder_instruction_t *inst, xed_state_t mode, xed_iclass_enum_t iclass, xed_uint_t effective_operand_width, xed_uint_t number_of_operands, const xed_encoder_operand_t *operand_array) { xed_inst(inst, mode, iclass, effective_operand_width, number_of_operands, operand_array); }
xed_bool_t xed_decoded_inst_valid_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_valid(p); }
const xed_inst_t * xed_decoded_inst_inst_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_inst(p); }
xed_category_enum_t xed_decoded_inst_get_category_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_category(p); }
xed_extension_enum_t xed_decoded_inst_get_extension_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_extension(p); }
xed_isa_set_enum_t xed_decoded_inst_get_isa_set_xed_sys_inline(const const xed_decoded_inst_t *const p) { return xed_decoded_inst_get_isa_set(p); }
xed_iclass_enum_t xed_decoded_inst_get_iclass_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_iclass(p); }
const xed_operand_values_t * xed_decoded_inst_operands_const_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_operands_const(p); }
xed_operand_values_t * xed_decoded_inst_operands_xed_sys_inline(xed_decoded_inst_t *p) { return xed_decoded_inst_operands(p); }
unsigned int xed_decoded_inst_noperands_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_noperands(p); }
void xed_decoded_inst_set_mode_xed_sys_inline(xed_decoded_inst_t *p, xed_machine_mode_enum_t mmode, xed_address_width_enum_t stack_addr_width) { xed_decoded_inst_set_mode(p, mmode, stack_addr_width); }
xed_uint_t xed_decoded_inst_get_length_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_length(p); }
uint8_t xed_decoded_inst_get_byte_xed_sys_inline(const xed_decoded_inst_t *p, xed_uint_t byte_index) { return xed_decoded_inst_get_byte(p, byte_index); }
xed_uint_t xed_decoded_inst_get_machine_mode_bits_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_machine_mode_bits(p); }
xed_uint_t xed_decoded_inst_get_stack_address_mode_bits_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_stack_address_mode_bits(p); }
xed_chip_enum_t xed_decoded_inst_get_input_chip_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_input_chip(p); }
void xed_decoded_inst_set_input_chip_xed_sys_inline(xed_decoded_inst_t *p, xed_chip_enum_t chip) { xed_decoded_inst_set_input_chip(p, chip); }
xed_iform_enum_t xed_decoded_inst_get_iform_enum_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_iform_enum(p); }
unsigned int xed_decoded_inst_get_iform_enum_dispatch_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_iform_enum_dispatch(p); }
uint8_t xed_decoded_inst_get_second_immediate_xed_sys_inline(const xed_decoded_inst_t *p) { return xed_decoded_inst_get_second_immediate(p); }
uint64_t xed_decoded_inst_get_user_data_xed_sys_inline(xed_decoded_inst_t *p) { return xed_decoded_inst_get_user_data(p); }
void xed_decoded_inst_set_user_data_xed_sys_inline(xed_decoded_inst_t *p, uint64_t new_value) { xed_decoded_inst_set_user_data(p, new_value); }
