use embedded_flash;
use stm32f401x::*;

const RCC: *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;

#[repr(u32)]
pub enum InputClock {
    HSI = 0,
    HSE = RCC_PLLCFGR_PLLSRC_HSE,
}

#[naked]
#[inline(always)]
pub fn hsi_ready() -> bool {
    unsafe { ((*RCC).CR & RCC_CR_HSIRDY) == RCC_CR_HSIRDY }
}

#[naked]
#[inline(always)]
pub fn disable_pll() {
    unsafe {
        (*RCC).CR &= !RCC_CR_PLLON;

        while pll_ready() {
            asm!("nop");
        }
    }
}

#[naked]
#[inline(always)]
pub fn enable_pll() {
    unsafe {
        (*RCC).CR |= RCC_CR_PLLON;

        while !pll_ready() {
            asm!("nop");
        }
    }
}

#[naked]
#[inline(always)]
pub fn use_pll() {
    embedded_flash::setup();

    unsafe {
        // use PLL
        (*RCC).CFGR |= RCC_CFGR_SW_PLL;

        // wait until cpu clock switched to PLL
        while (&(*RCC).CFGR & RCC_CFGR_SWS_PLL) != RCC_CFGR_SWS_PLL {
            asm!("nop");
        }

        // doesn't divide clock for AHB
        (*RCC).CFGR &= !RCC_CFGR_HPRE;

        // divide APB1 clock by 2 (84MHz -> 42MHz)
        (*RCC).CFGR |= RCC_CFGR_PPRE1_DIV2;

        // doesn't divide clock for APB2
        (*RCC).CFGR &= !RCC_CFGR_PPRE2;
    }
}

// Hardcode default max clock for this board
// st32f401re has 84MHz general clock
// and 48MHz peripherals clock (USB_OTG, SDIO, etc)
#[naked]
#[inline(always)]
pub fn config_pll(input: InputClock) {
    const f_pll: u32 = 84;
    const f_peripherals: u32 = 48;

    let f_pll_input = match input {
        InputClock::HSI => 16,
        InputClock::HSE => 8,
    };
    let n: u32 = 336;
    let m = &f_pll_input;
    let f_vco = &f_pll_input * (&n / m);
    let p = &f_vco / f_pll;
    let q = &f_vco / f_peripherals;

    unsafe {
        (*RCC).PLLCFGR = (q << 24) as u32 | (input as u32) | (p << 16) as u32 | (n << 6) as u32 | m;
    }
}

#[naked]
#[inline(always)]
pub fn pll_ready() -> bool {
    unsafe { (*RCC).CR & RCC_CR_PLLRDY == RCC_CR_PLLRDY }
}

pub mod AHB1 {
    use super::RCC;
    use stm32f401x::*;

    #[naked]
    #[inline(always)]
    pub fn enable_gpioa() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOAEN;
        }
    }

    #[naked]
    #[inline(always)]
    pub fn enable_gpiob() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOBEN;
        }
    }

    #[naked]
    #[inline(always)]
    pub fn enable_gpioc() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOCEN;
        }
    }

    #[naked]
    #[inline(always)]
    pub fn enable_gpiod() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIODEN;
        }
    }

    #[naked]
    #[inline(always)]
    pub fn enable_gpioe() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOEEN;
        }
    }

    #[naked]
    #[inline(always)]
    pub fn enable_gpioh() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOHEN;
        }
    }
}

pub mod APB1 {
    use super::RCC;
    use stm32f401x::*;

    #[naked]
    #[inline(always)]
    pub fn enable_pwr() {
        unsafe {
            (*RCC).APB1ENR |= RCC_APB1ENR_PWREN;
        }
    }
}
