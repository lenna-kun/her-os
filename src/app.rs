use cortex_m_semihosting::hprintln;

pub fn app1() -> ! {
    let mut i = 0;
    loop {
        for _ in 0..10000 {}
        hprintln!("app1: {}", i).unwrap();
        i += 1;
    }
}

pub fn app2() -> ! {
    let mut i = 0;
    loop {
        for _ in 0..10000 {}
        hprintln!("app2: {}", i).unwrap();
        i += 1;
    }
}

pub fn app3() -> ! {
    let mut i = 0;
    loop {
        for _ in 0..10000 {}
        hprintln!("app3: {}", i).unwrap();
        i += 1;
    }
}