#![feature(llvm_asm)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod app;
mod kernel;
mod peripherals;
mod sections;
mod exceptions;

use kernel::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub fn main() -> ! {
    peripherals::systick::set_timer_us(10000);

    peripherals::gpio::init();

    unsafe {
        let process1 = Process::new(&mut app::APP_STACK1, app::app1);
        let process2 = Process::new(&mut app::APP_STACK2, app::app2);
        let process3 = Process::new(&mut app::APP_STACK3, app::app3);
        let process4 = Process::new(&mut app::APP_STACK4, app::app4);

        let mut kernel = Kernel::new();

        kernel.load_process(process1).unwrap();
        kernel.load_process(process2).unwrap();
        kernel.load_process(process3).unwrap();
        kernel.load_process(process4).unwrap();

        kernel.run_scheduling()
    }
}