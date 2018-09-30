use asm::*;
use embedded_hal::blocking::delay::DelayMs;
use peripherals::systick;

struct SysTickTimer;

impl DelayMs<u32> for SysTickTimer {
    fn delay_ms(&mut self, ms: u32) {
        let mut counter = 0;

        systick::start(systick::Calibrated1ms);
        while counter < ms {
            while !systick::counted_to_zero() {
                nop();
            }
            counter += 1;
        }
        systick::stop();
    }
}

static mut SysTick: SysTickTimer = SysTickTimer {};

pub fn delay_ms(ms: u32) {
    unsafe {
        SysTick.delay_ms(ms);
    }
}
