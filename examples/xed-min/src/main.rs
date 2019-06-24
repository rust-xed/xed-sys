/// Replica of `examples/xed-min.c` from official Intel XED repository.
use xed_sys2::xed_interface::*;

fn main() {
    unsafe {
        let long_mode: bool = false;
        let (mmode, stack_addr_width) = if long_mode {
            (XED_MACHINE_MODE_LONG_64, XED_ADDRESS_WIDTH_64b)
        } else {
            (XED_MACHINE_MODE_LEGACY_32, XED_ADDRESS_WIDTH_32b)
        };

        xed_tables_init();

        let itext: [u8; 15] = [
            0xf, 0x85, 0x99, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        for bytes in 0..16 {
            let mut xedd: xed_decoded_inst_t = ::std::mem::uninitialized();
            xed_decoded_inst_zero(&mut xedd);
            xed_decoded_inst_set_mode(&mut xedd, mmode, stack_addr_width);

            let xed_error: xed_error_enum_t = xed_decode(&mut xedd, itext.as_ptr(), bytes);
            let error_desc =
                std::ffi::CStr::from_ptr(xed_error_enum_t2str(xed_error)).to_string_lossy();
            println!("bytes={} error={}", bytes, error_desc);
        }
    }
}
