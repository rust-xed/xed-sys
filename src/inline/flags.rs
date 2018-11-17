use std::os::raw::c_int;
use xed_interface::*;

/// @ingroup FLAGS    
/// Return the flags as a mask
pub unsafe fn xed_flag_set_mask(p: *const xed_flag_set_t) -> c_int {
    return (*p).flat as i32;
}
