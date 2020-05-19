use core::ptr::{read_volatile, write_volatile};

const SYST_CSR: usize = 0xe000_e010;
const SYST_RVR: usize = 0xe000_e014;
const SYST_CVR: usize = 0xe000_e018;
const SYST_CALIB: usize = 0xe000_e01c;

pub fn set_timer_us(us :u32) {
    unsafe {
        write_volatile(SYST_CVR as *mut u32, 0);
        let tenms = read_volatile(SYST_CALIB as *const u32) & 0x00ff_ffff;
        let reload = (tenms as u64 * 100) * us as u64 / 1_000_000;
        write_volatile(SYST_RVR as *mut u32, reload as u32);
        write_volatile(SYST_CSR as *mut u32, 0b111);
    }
}