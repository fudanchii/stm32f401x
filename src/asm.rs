#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}

#[inline(always)]
pub fn set_breakpoint() {
    unsafe {
        asm!("bkpt");
    }
}
