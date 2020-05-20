use cortex_m_semihosting::hprintln;

use crate::main;
use super::sections;

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    // fn Reset();
    fn NMI();
    // fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    // fn SVCall();
    fn PendSV();
    // fn SysTick();
}

#[link_section = ".vector_table"]
#[no_mangle]
pub static VECTOR_TABLE: [Vector; 15] = [
    Vector { handler: Reset },
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub unsafe extern "C" fn Reset() {
    sections::init();

    main()
}

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[no_mangle]
pub extern "C" fn HardFault() {
    hprintln!("HardFault").unwrap();
    loop{}
}

#[no_mangle]
pub unsafe extern "C" fn SVCall() {
    llvm_asm!(
        "
        cmp lr, #0xfffffff9
        bne to_kernel

        mov r0, #1
        msr CONTROL, r0
        movw lr, #0xfffd
        movt lr, #0xffff
        bx lr
    
        to_kernel:
        mov r0, #0
        msr CONTROL, r0

        movw lr, #0xfff9
        movt lr, #0xffff
        bx lr
        "
    ::::"volatile");
}

/// The `SysTick` is called when the systick interrupt occurs, signals
/// that an application executed for longer than its timeslice. If this is
/// called we want to return to the scheduler.
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    llvm_asm!(
        "
        mov r0, #0
        msr CONTROL, r0
        movw lr, #0xfff9
        movt lr, #0xffff
        bx lr
        "
    ::::"volatile");
}