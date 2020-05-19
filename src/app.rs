use super::peripherals;

#[inline]
fn wait_a_bit() {
    for _ in 0..2000 { unsafe { llvm_asm!("nop"::::"volatile"); } }
}

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
    let pe9 = peripherals::gpio::PEx::new(9);
    pe9.mode_push_pull_output();

    let mut on = true;
    loop {
        if on {
            pe9.set_high();
        } else {
            pe9.set_low();
        }
        wait_a_bit();
        on = !on;
    }
}

pub fn app3() -> ! {
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