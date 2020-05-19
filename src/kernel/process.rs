#[repr(C)]
struct RegistersSavedByHardware {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub pc: u32,
    pub xpsr: u32,
}

pub struct Process {
    sp: u32,
    regs: [u32; 8],
}

impl Process {
    pub fn new(stack: &mut [u8], app_main: fn() -> !) -> Self {
        let sp = (&stack[0] as *const u8 as usize) + stack.len() - 0x20;
        let rsbh: &mut RegistersSavedByHardware = unsafe { &mut *(sp as *mut RegistersSavedByHardware) };
        rsbh.r0 = 0;
        rsbh.r1 = 0;
        rsbh.r2 = 0;
        rsbh.r3 = 0;
        rsbh.r12 = 0;
        rsbh.lr = 0;
        rsbh.pc = app_main as u32;
        rsbh.xpsr = 0x0100_0000; // T bit of EPSR = 1

        Process {
            sp: sp as u32,
            regs: [0; 8],
        }
    }

    pub fn exec(&mut self) {
        unsafe {
            llvm_asm!(
                "
                msr psp, r0
                ldmia r1, {r4-r11}
                svc 0
                stmia r1, {r4-r11}
                mrs r0, psp
                "
                :"={r0}"(self.sp)
                :"{r0}"(self.sp), "{r1}"(&self.regs)
                :"r4", "r5", "r6", "r8", "r9", "r10", "r11"
                :"volatile"
            );
        }
    }
}