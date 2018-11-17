use xed_interface::*;

/// @ingroup ENC
/// Retrieve the number of entries in the encoder operand order array
/// @return The number of entries in the encoder operand order array
pub unsafe fn xed_encoder_request_operand_order_entries(
    p: *mut xed_encoder_request_t,
) -> xed_uint_t {
    return (*p)._n_operand_order as u32;
}

///@ingroup ENC
pub unsafe fn xed_encoder_request_operands_const(
    p: *const xed_encoder_request_t,
) -> *const xed_operand_values_t {
    return p as *const _;
}

///@ingroup ENC
pub unsafe fn xed_encoder_request_operands(
    p: *mut xed_encoder_request_t,
) -> *mut xed_operand_values_t {
    return p as *mut _;
}
