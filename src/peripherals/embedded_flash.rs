use stm32f401x::*;

const FLASH: *mut FLASH_TypeDef = FLASH_R_BASE as *mut FLASH_TypeDef;

// Enable:
// - instruction cache
// - prefetch
// - 2 wait state latency (84MHz)
#[naked]
#[inline(always)]
pub fn setup() {
    unsafe {
        (*FLASH).ACR |= FLASH_ACR_ICEN | FLASH_ACR_PRFTEN | FLASH_ACR_LATENCY_2WS;
        while ((*FLASH).ACR & FLASH_ACR_LATENCY_2WS) != FLASH_ACR_LATENCY_2WS {
            asm!("nop");
        }
    }
}
