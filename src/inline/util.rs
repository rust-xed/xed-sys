use std::mem;
use xed_interface::*;

pub unsafe fn xed_make_uint64(hi: u32, lo: u32) -> u64 {
    let mut y: xed_union64_t = mem::uninitialized();
    y.s.lo32 = lo;
    y.s.hi32 = hi;
    return y.u64;
}

pub unsafe fn xed_make_int64(hi: u32, lo: u32) -> i64 {
    let mut y: xed_union64_t = mem::uninitialized();
    y.s.lo32 = lo;
    y.s.hi32 = hi;
    return y.i64;
}
