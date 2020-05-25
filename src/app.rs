use aligned::{Aligned, A8};

use super::peripherals;

#[inline]
fn wait_a_bit() {
    for _ in 0..2000 { unsafe { llvm_asm!("nop"::::"volatile"); } }
}

pub static mut PROCESS_STACK1: Aligned<A8, [u8; 1024]> = Aligned([0; 1024]);
pub static mut PROCESS_STACK2: Aligned<A8, [u8; 1024]> = Aligned([0; 1024]);
pub static mut PROCESS_STACK3: Aligned<A8, [u8; 1024]> = Aligned([0; 1024]);
pub static mut PROCESS_STACK4: Aligned<A8, [u8; 1024]> = Aligned([0; 1024]);

pub fn app1() -> ! {
    let pe8 = peripherals::gpio::PEx::new(8);
    pe8.mode_push_pull_output();

    let mut on = true;
    loop {
        if on {
            pe8.set_high();
        } else {
            pe8.set_low();
        }
        wait_a_bit();
        on = !on;
    }
}

pub fn app2() -> ! {
    let pe10 = peripherals::gpio::PEx::new(10);
    pe10.mode_push_pull_output();

    let mut on = true;
    loop {
        if on {
            pe10.set_high();
        } else {
            pe10.set_low();
        }
        wait_a_bit();
        on = !on;
    }
}

pub fn app3() -> ! {
    let pe12 = peripherals::gpio::PEx::new(12);
    pe12.mode_push_pull_output();

    let mut on = true;
    loop {
        if on {
            pe12.set_high();
        } else {
            pe12.set_low();
        }
        wait_a_bit();
        on = !on;
    }
}

pub fn app4() -> ! {
    let pe14 = peripherals::gpio::PEx::new(14);
    pe14.mode_push_pull_output();

    let mut on = true;
    loop {
        if on {
            pe14.set_high();
        } else {
            pe14.set_low();
        }
        wait_a_bit();
        on = !on;
    }
}