pub const RCC: usize = 0x40021000;

#[repr(C)]
pub struct RccRegisters {
    cr: u32,
    cfgr: u32,
    cir: u32,
    apb2rstr: u32,
    apb1rstr: u32,
    pub ahbenr: AHBENR,
    apb2enr: u32,
    apb1enr: u32,
    bdcr: u32,
    csr: u32,
    pub ahbrstr: AHBRSTR,
    cfgr2: u32,
    cfgr3: u32,
}

impl RccRegisters {
    pub fn ptr() -> *const RccRegisters {
        RCC as *const _
    }
}

pub struct AHBENR {
    register: vcell::VolatileCell<u32>,
}

pub struct AHBRSTR {
    register: vcell::VolatileCell<u32>,
}

impl AHBENR {
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let bits = self.register.get();
        self.register.set(f(bits));
    }
}

impl AHBRSTR {
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let bits = self.register.get();
        self.register.set(f(bits));
    }
}