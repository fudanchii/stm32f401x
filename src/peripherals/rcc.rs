use stm32f401x::*;

const RCC: *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;

pub enum InputClock {
    HSI = 0,
    HSE = RCC_PLLCFGR_PLLSRC_HSE,
}

pub fn hsi_ready() -> bool {
    unsafe { ((*RCC).CR & RCC_CR_HSIRDY) == RCC_CR_HSIRDY }
}

pub fn disable_pll() {
    unsafe {
        (*RCC).CR &= !RCC_CR_PLLON;
    }
}

pub fn enable_pll() {
    unsafe {
        (*RCC).CR |= RCC_CR_PLLON;
    }
}

//! Hardcode default max clock for this board
//! st32f401re has 84MHz general clock
//! and 48MHz peripherals clock (USB_OTG, SDIO, etc)
pub fn config_pll(input: InputClock) {
    let (f_pll_input, f_pll_src = match input {
        InputClock::HSI => (16, 0),
        InputClock::HSE => (8, 1),
    };
    let f_pll = 84;
    let f_peripherals = 48;
    let n = 336;
    let m = &f_pll_input;
    let f_vco = &f_pll_input * (&n / &m);
    let p = &f_vco / 84;
    let q = &f_vco / 48;

    unsafe {
        (*RCC).PLLCFGR = (q << 24) | (f_pll_src << 22) | (p << 16) | (n << 6) | m;
    }
}

pub fn pll_ready() -> bool {
    unsafe { (*RCC).CR & RCC_CR_PLLRDY == RCC_CR_PLLRDY }
}

pub mod AHB1 {
    pub fn enable_gpioa() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOAEN;
        }
    }

    pub fn enable_gpiob() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOBEN;
        }
    }

    pub fn enable_gpioc() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOCEN;
        }
    }

    pub fn enable_gpiod() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIODEN;
        }
    }

    pub fn enable_gpioe() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOEEN;
        }
    }

    pub fn enable_gpioh() {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOHEN;
        }
    }
}

pub mod APB1 {
    pub fn enable_pwr() {
        unsafe {
            (*RCC).APB1ENR |= RCC_APB1ENR_PWREN;
        }
    }
}
