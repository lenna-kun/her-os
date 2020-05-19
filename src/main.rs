#![feature(llvm_asm)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod app;
mod kernel;
mod peripherals;
mod sections;
mod systick;
mod exceptions;

use kernel::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe fn() -> ! = reset_handler;

#[no_mangle]
pub unsafe fn reset_handler() -> ! {
    
    sections::init();

    systick::init();

    peripherals::gpio::init();

    #[link_section = ".app_stack"]
    static mut APP_STACK1: [u8; 1024] = [0; 1024];
    #[link_section = ".app_stack"]
    static mut APP_STACK2: [u8; 1024] = [0; 1024];
    #[link_section = ".app_stack"]
    static mut APP_STACK3: [u8; 1024] = [0; 1024];

    let process1 = Process::new(&mut APP_STACK1, app::app1);
    let process2 = Process::new(&mut APP_STACK2, app::app2);
    let process3 = Process::new(&mut APP_STACK3, app::app3);

    let mut kernel = Kernel::new();

    kernel.load_process(process1).unwrap();
    kernel.load_process(process2).unwrap();
    kernel.load_process(process3).unwrap();

    kernel.run_scheduling()
}