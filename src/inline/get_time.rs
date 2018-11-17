#[cfg(target_arch = "x86")]
use core::arch::x86::_rdtsc;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_rdtsc;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn get_time() -> u64 {
    return _rdtsc() as u64;
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub unsafe fn get_time() -> u64 {
    return 0;
}
