use stm32f401x::*;

const SysTick: *mut SysTick_Type = SysTick_BASE as *mut SysTick_Type;

pub const Calibrated1ms: u32 = 10500;

#[repr(u32)]
pub enum ClockSource {
    AHBdiv8 = 0,
    AHB = SysTick_CTRL_CLKSOURCE_Msk,
}

#[repr(u32)]
pub enum TickIntr {
    None = 0,
    Assert = SysTick_CTRL_TICKINT_Msk,
}

#[repr(u32)]
pub enum Enable {
    No = 0,
    Yes = 1,
}

// Also reset counted flag if it's read as true
pub fn counted_to_zero() -> bool {
    unsafe { if ((*SysTick).CTRL & SysTick_CTRL_COUNTFLAG_Msk) == SysTick_CTRL_COUNTFLAG_Msk {
        (*SysTick).VAL = 0;
        return true;
    }}
    false
}


pub fn setup(src: ClockSource, intr: TickIntr, en: Enable) {
    unsafe {
        (*SysTick).CTRL = (src as u32) | (intr as u32) | (en as u32);
    }
}

pub fn load(counter: u32) {
    unsafe {
        (*SysTick).LOAD = counter & 0x00FF_FFFF;
        (*SysTick).VAL = 0;
    }
}

pub fn start(counter: u32) {
    load(counter);
    setup(ClockSource::AHBdiv8, TickIntr::None, Enable::Yes);
}

pub fn stop() {
    unsafe {
        (*SysTick).CTRL &= !(Enable::Yes as u32);
    }
}

pub fn reset() {
    setup(ClockSource::AHBdiv8, TickIntr::None, Enable::No);
}
