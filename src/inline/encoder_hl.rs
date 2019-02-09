use std::mem;
use xed_interface::*;

/// @ingroup ENCHL
/// a memory displacement (not for branches)
/// @param displacement The value of the displacement
/// @param displacement_bits The width of the displacement in bits. Typically 8 or 32.
/// @returns #xed_enc_displacement_t
pub unsafe fn xed_disp(displacement: u64, displacement_bits: u32) -> xed_enc_displacement_t {
    return xed_enc_displacement_t {
        displacement,
        displacement_bits,
    };
}

/// @ingroup ENCHL
/// a relative branch displacement operand
/// @param brdisp The branch displacement
/// @param width_bits The width of the displacement in bits. Typically 8 or 32.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_relbr(brdisp: i32, width_bits: xed_uint_t) -> xed_encoder_operand_t {
    return xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_BRDISP,
        width_bits,
        u: xed_encoder_operand_t__bindgen_ty_1 { brdisp },
    };
}

/// @ingroup ENCHL
/// a relative displacement for a PTR operand -- the subsequent imm0 holds
///the 16b selector
/// @param brdisp The displacement for a far pointer operand
/// @param width_bits The width of the far pointr displacement in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_ptr(brdisp: i32, width_bits: xed_uint_t) -> xed_encoder_operand_t {
    return xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_PTR,
        width_bits,
        u: xed_encoder_operand_t__bindgen_ty_1 { brdisp },
    };
}

/// @ingroup ENCHL
/// a register operand
/// @param reg A #xed_reg_enum_t register operand
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_reg(reg: xed_reg_enum_t) -> xed_encoder_operand_t {
    return xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_REG,
        width_bits: 0,
        u: xed_encoder_operand_t__bindgen_ty_1 { reg },
    };
}

/// @ingroup ENCHL
/// a first immediate operand (known as IMM0)
/// @param v An immdediate operand.
/// @param width_bits The immediate width in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_imm0(v: u64, width_bits: xed_uint_t) -> xed_encoder_operand_t {
    return xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_IMM0,
        width_bits,
        u: xed_encoder_operand_t__bindgen_ty_1 { imm0: v },
    };
}

/// @ingroup ENCHL
/// an 32b signed immediate operand
/// @param v An signed immdediate operand.
/// @param width_bits The immediate width in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_simm0(v: u32, width_bits: xed_uint_t) -> xed_encoder_operand_t {
    return xed_encoder_operand_t {
        type_: XED_ENCODER_OPERAND_TYPE_SIMM0,
        width_bits,
        u: xed_encoder_operand_t__bindgen_ty_1 {
            // Comment from original source
            /* sign conversion: we store the int32 in an uint64. It gets sign
            extended.  Later we convert it to the right width_bits for the
            instruction. The maximum width_bits of a signed immediate is currently
            32b. */
            imm0: v as u64,
        },
    };
}

/// @ingroup ENCHL
/// The 2nd immediate operand (known as IMM1) for rare instructions that require it.
/// @param v The 2nd immdediate (byte-width) operand
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_imm1(v: u8) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_IMM1;
    o.width_bits = 8;
    o.u.imm1 = v;
    return o;
}

/// @ingroup ENCHL
/// an operand storage field name and value
pub unsafe fn xed_other(operand_name: xed_operand_enum_t, value: u32) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_OTHER;
    o.width_bits = 0;
    o.u.s.operand_name = operand_name;
    o.u.s.value = value;
    return o;
}

/// @ingroup ENCHL
/// seg reg override for implicit suppressed memory ops
pub unsafe fn xed_seg0(seg0: xed_reg_enum_t) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_SEG0;
    o.u.reg = seg0;
    o.width_bits = 0;
    return o;
}

/// @ingroup ENCHL
/// seg reg override for implicit suppressed memory ops
pub unsafe fn xed_seg1(seg1: xed_reg_enum_t) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_SEG1;
    o.u.reg = seg1;
    o.width_bits = 0;
    return o;
}

/// @ingroup ENCHL
/// memory operand - base only
/// @param base The base register
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_b(base: xed_reg_enum_t, width_bits: xed_uint_t) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = XED_REG_INVALID;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0;
    o.u.mem.disp.displacement = 0;
    o.u.mem.disp.displacement_bits = 0;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// memory operand - base and displacement only
/// @param base The base register
/// @param disp The displacement
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_bd(
    base: xed_reg_enum_t,
    disp: xed_enc_displacement_t,
    width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = XED_REG_INVALID;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// memory operand - base, index, scale, displacement
/// @param base The base register
/// @param index The index register
/// @param scale The scale for the index register value
/// @param disp The displacement
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_bisd(
    base: xed_reg_enum_t,
    index: xed_reg_enum_t,
    scale: xed_uint_t,
    disp: xed_enc_displacement_t,
    width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = XED_REG_INVALID;
    o.u.mem.index = index;
    o.u.mem.scale = scale;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// memory operand - segment and base only
/// @param seg The segment override register
/// @param base The base register
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_gb(
    seg: xed_reg_enum_t,
    base: xed_reg_enum_t,
    width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = seg;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0;
    o.u.mem.disp.displacement = 0;
    o.u.mem.disp.displacement_bits = 0;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// memory operand - segment, base and displacement only
/// @param seg The segment override register
/// @param base The base register
/// @param disp The displacement
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_gbd(
    seg: xed_reg_enum_t,
    base: xed_reg_enum_t,
    disp: xed_enc_displacement_t,
    width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = seg;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// memory operand - segment and displacement only
/// @param seg The segment override register
/// @param disp The displacement
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_gd(
    seg: xed_reg_enum_t,
    disp: xed_enc_displacement_t,
    width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = XED_REG_INVALID;
    o.u.mem.seg = seg;
    o.u.mem.index = XED_REG_INVALID;
    o.u.mem.scale = 0;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// memory operand - segment, base, index, scale, and displacement
/// @param seg The segment override register
/// @param base The base register
/// @param index The index register
/// @param scale The scale for the index register value
/// @param disp The displacement
/// @param width_bits The length of the memory reference in bits.
/// @returns xed_encoder_operand_t An operand.
pub unsafe fn xed_mem_gbisd(
    seg: xed_reg_enum_t,
    base: xed_reg_enum_t,
    index: xed_reg_enum_t,
    scale: xed_uint_t,
    disp: xed_enc_displacement_t,
    width_bits: xed_uint_t,
) -> xed_encoder_operand_t {
    let mut o: xed_encoder_operand_t = mem::uninitialized();
    o.type_ = XED_ENCODER_OPERAND_TYPE_MEM;
    o.u.mem.base = base;
    o.u.mem.seg = seg;
    o.u.mem.index = index;
    o.u.mem.scale = scale;
    o.u.mem.disp = disp;
    o.width_bits = width_bits;
    return o;
}

/// @ingroup ENCHL
/// This is to specify effective address size different than the
/// default. For things with base or index regs, XED picks it up from the
/// registers. But for things that have implicit memops, or no base or index
/// reg, we must allow the user to set the address width directly.
/// @param x The #xed_encoder_instruction_t being filled in.
/// @param width_bits The intended effective address size in bits.  Values: 16, 32 or 64.
pub unsafe fn xed_addr(x: *mut xed_encoder_instruction_t, width_bits: xed_uint_t) {
    (*x).effective_address_width = width_bits;
}

/// @ingroup ENCHL
/// To add a REP (0xF3) prefix.
/// @param x The #xed_encoder_instruction_t being filled in.
pub unsafe fn xed_rep(x: *mut xed_encoder_instruction_t) {
    (*x).prefixes.s.set_rep(1);
}

/// @ingroup ENCHL
/// To add a REPNE (0xF2) prefix.
/// @param x The #xed_encoder_instruction_t being filled in.
pub unsafe fn xed_repne(x: *mut xed_encoder_instruction_t) {
    (*x).prefixes.s.set_repne(1);
}

/// @ingroup ENCHL
/// instruction with no operands
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
pub unsafe fn xed_inst0(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;
    (*inst).noperands = 0;
}

/// @ingroup ENCHL
/// instruction with one operand
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
/// @param op0 the operand
pub unsafe fn xed_inst1(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
    op0: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;
    (*inst).noperands = 1;
    (*inst).operands[0] = op0;
}

/// @ingroup ENCHL
/// instruction with two operands
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
/// @param op0 the 1st operand
/// @param op1 the 2nd operand
pub unsafe fn xed_inst2(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
    op0: xed_encoder_operand_t,
    op1: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;
    (*inst).noperands = 1;
    (*inst).operands[0] = op0;
    (*inst).operands[1] = op1;
}

/// @ingroup ENCHL
/// instruction with three operands
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
/// @param op0 the 1st operand
/// @param op1 the 2nd operand
/// @param op2 the 3rd operand
pub unsafe fn xed_inst3(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
    op0: xed_encoder_operand_t,
    op1: xed_encoder_operand_t,
    op2: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;
    (*inst).noperands = 1;
    (*inst).operands[0] = op0;
    (*inst).operands[1] = op1;
    (*inst).operands[2] = op2;
}

/// @ingroup ENCHL
/// instruction with four operands
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
/// @param op0 the 1st operand
/// @param op1 the 2nd operand
/// @param op2 the 3rd operand
/// @param op3 the 4th operand
pub unsafe fn xed_inst4(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
    op0: xed_encoder_operand_t,
    op1: xed_encoder_operand_t,
    op2: xed_encoder_operand_t,
    op3: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;
    (*inst).noperands = 1;
    (*inst).operands[0] = op0;
    (*inst).operands[1] = op1;
    (*inst).operands[2] = op2;
    (*inst).operands[3] = op3;
}

/// @ingroup ENCHL
/// instruction with five operands
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
/// @param op0 the 1st operand
/// @param op1 the 2nd operand
/// @param op2 the 3rd operand
/// @param op3 the 4th operand
/// @param op4 the 5th operand
pub unsafe fn xed_inst5(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
    op0: xed_encoder_operand_t,
    op1: xed_encoder_operand_t,
    op2: xed_encoder_operand_t,
    op3: xed_encoder_operand_t,
    op4: xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;
    (*inst).noperands = 1;
    (*inst).operands[0] = op0;
    (*inst).operands[1] = op1;
    (*inst).operands[2] = op2;
    (*inst).operands[3] = op3;
    (*inst).operands[4] = op4;
}

/// @ingroup ENCHL
/// instruction with an array of operands. The maximum number is
/// XED_ENCODER_OPERANDS_MAX. The array's contents are copied.
/// @param inst The #xed_encoder_instruction_t to be filled in
/// @param mode  The xed_state_t including the machine mode and stack address width.
/// @param iclass The #xed_iclass_enum_t
/// @param effective_operand_width in bits
/// @param number_of_operands length of the subsequent array
/// @param operand_array An array of #xed_encoder_operand_t objects
pub unsafe fn xed_inst(
    inst: *mut xed_encoder_instruction_t,
    mode: xed_state_t,
    iclass: xed_iclass_enum_t,
    effective_operand_width: xed_uint_t,
    number_of_operands: xed_uint_t,
    operand_array: *const xed_encoder_operand_t,
) {
    (*inst).mode = mode;
    (*inst).iclass = iclass;
    (*inst).effective_operand_width = effective_operand_width;
    (*inst).effective_address_width = 0;
    (*inst).prefixes.i = 0;

    assert!(number_of_operands < XED_ENCODER_OPERANDS_MAX);
    for i in 0..number_of_operands {
        (*inst).operands[i as usize] = *operand_array.offset(i as isize);
    }

    (*inst).noperands = number_of_operands;
}
