#[repr(C)]
struct ContextBasicFrame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub return_address: u32,
    pub xpsr: u32,
}

pub struct Process {
    psp: u32,
    regs: [u32; 8],
}

impl Process {
    pub fn new(stack: &mut [u8], app_main: fn() -> !) -> Self {
        let psp = (&stack[0] as *const u8 as usize) + stack.len() - 0x20;
        let cbf: &mut ContextBasicFrame = unsafe { &mut *(psp as *mut ContextBasicFrame) };
        cbf.r0 = 0;
        cbf.r1 = 0;
        cbf.r2 = 0;
        cbf.r3 = 0;
        cbf.r12 = 0;
        cbf.lr = 0;
        cbf.return_address = app_main as u32;
        cbf.xpsr = 0x0100_0000; // T bit of EPSR = 1

        Process {
            psp: psp as u32,
            regs: [0; 8],
        }
    }

    pub fn switch_to(&mut self) {
        unsafe {
            llvm_asm!(
                "
                msr psp, r0
                ldmia r1, {r4-r11}
                svc 0
                stmia r1, {r4-r11}
                mrs r0, psp
                "
                :"={r0}"(self.psp)
                :"{r0}"(self.psp), "{r1}"(&self.regs)
                :"r4", "r5", "r6", "r8", "r9", "r10", "r11"
                :"volatile"
            );
        }
    }
}