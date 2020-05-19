use super::rcc;

pub const GPIOE: usize = 0x48001000;

pub fn init() {
    let rcc = unsafe { &*rcc::RccRegisters::ptr() };

    rcc.ahbenr.modify(|r| r | (0b1 << 21));
    rcc.ahbrstr.modify(|r| r | (0b1 << 21));
    rcc.ahbrstr.modify(|r| r & !(0b1 << 21));
}

#[repr(C)]
pub struct GpioeRegisters {
    pub moder: MODER,
    pub otyper: OTYPER,
    ospeedr: u32,
    pupdr: u32,
    idr: u32,
    odr: u32,
    pub bsrr: BSRR,
    lckr: u32,
    afrl: u32,
    afrh: u32,
    brr: u32,
}

impl GpioeRegisters {
    pub fn ptr() -> *const GpioeRegisters {
        GPIOE as *const _
    }
}

pub struct MODER {
    register: vcell::VolatileCell<u32>,
}

pub struct OTYPER {
    register: vcell::VolatileCell<u32>,
}

pub struct BSRR {
    register: vcell::VolatileCell<u32>,
}

impl MODER {
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let bits = self.register.get();
        self.register.set(f(bits));
    }
}

impl OTYPER {
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let bits = self.register.get();
        self.register.set(f(bits));
    }
}

impl BSRR {
    #[inline]
    pub fn write(&self, w: u32)
    {
        self.register.set(w);
    }
}

pub struct PEx {
    x: u8,
}

impl PEx {
    pub fn new(x: u8) -> Self {
        PEx {
            x: x,
        }
    }

    /// Configures the pin to operate as an push pull output pin
    #[inline]
    pub fn mode_push_pull_output(&self) {
        let gpioe = unsafe { &*GpioeRegisters::ptr() };

        let offset = 2 * self.x;

        // general purpose output mode
        let mode = 0b01;
        gpioe.moder.modify(|r| (r & !(0b11 << offset)) | (mode << offset));

        // push pull output
        gpioe.otyper.modify(|r| r & !(0b1 << self.x));
    }

    #[inline]
    pub fn set_high(&self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*GpioeRegisters::ptr()).bsrr.write(1 << self.x) }
    }

    #[inline]
    pub fn set_low(&self) {
        unsafe { (*GpioeRegisters::ptr()).bsrr.write(1 << (16 + self.x)) }
    }
}