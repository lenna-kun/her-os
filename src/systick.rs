use core::ptr::{read_volatile, write_volatile};

const SYST_CSR: usize = 0xe000_e010;
const SYST_RVR: usize = 0xe000_e014;
const SYST_CVR: usize = 0xe000_e018;
const SYST_CALIB: usize = 0xe000_e01c;

pub fn init() {
    unsafe {
        write_volatile(SYST_CVR as *mut u32, 0);
        let tenms = read_volatile(SYST_CALIB as *const u32) & 0x00ff_ffff;
        write_volatile(SYST_RVR as *mut u32, tenms * 10);
        write_volatile(SYST_CSR as *mut u32, 0b111);
    }
}