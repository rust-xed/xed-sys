use xed_interface::*;

/// @ingroup IFORM
/// Return the iclass for a given iform. This
/// function returns valid data as soon as global data is initialized. (This
/// function does not require a decoded instruction as input).
pub unsafe fn xed_iform_to_iclass(iform: xed_iform_enum_t) -> xed_iclass_enum_t {
    let ii: *const xed_iform_info_t = xed_iform_map(iform);
    if !ii.is_null() {
        return (*ii).iclass();
    }
    return XED_ICLASS_INVALID;
}
