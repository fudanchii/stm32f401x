use stm32f401x::*;
use peripherals::RCC;

const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut GPIO_TypeDef;
const GPIOB: *mut GPIO_TypeDef = GPIOB_BASE as *mut GPIO_TypeDef;
const GPIOC: *mut GPIO_TypeDef = GPIOC_BASE as *mut GPIO_TypeDef;
const GPIOD: *mut GPIO_TypeDef = GPIOD_BASE as *mut GPIO_TypeDef;
const GPIOE: *mut GPIO_TypeDef = GPIOE_BASE as *mut GPIO_TypeDef;
const GPIOH: *mut GPIO_TypeDef = GPIOH_BASE as *mut GPIO_TypeDef;

#[derive(Copy, Clone)]
pub enum Reg {
    Moder0 = 0,
    Moder1,
    Moder2,
    Moder3,
    Moder4,
    Moder5,
    Moder7,
    Moder8,
    Moder9,
    Moder10,
    Moder11,
    Moder12,
    Moder13,
    Moder14,
    Moder15,
}

pub enum Mode {
    In = 0,
    Out,
    Alter,
    Analog,
}

pub trait gpio {
    fn enable(reg: Reg, mode: Mode) -> Self;
}

#[derive(Copy, Clone)]
pub struct A(Reg);

impl A {
    pub fn enable(reg: Reg, mode: Mode) -> Self {
        unsafe {
            (*RCC).AHB1ENR |= RCC_AHB1ENR_GPIOAEN;
            (*GPIOA).MODER |= (mode as u32) << ((reg as u8) << 1);
        }
        A(reg)
    }

    pub fn disable() {
        unsafe { (*RCC).AHB1ENR &= !RCC_AHB1ENR_GPIOAEN; }
    }

    pub fn on(&self) {
        unsafe { (*GPIOA).BSRR = 1 << (self.0 as u8); }
    }

    pub fn off(&self) {
        unsafe { (*GPIOA).BSRR = 1 << (self.0 as u8 + 16); }
    }

    pub fn set(&self, mode: Mode) {
        unsafe { (*GPIOA).MODER |= (mode as u32) << ((self.0 as u8) << 1); }
    }

    pub fn unset(&self, mode: Mode) {
        unsafe { (*GPIOA).MODER &= !(mode as u32) << ((self.0 as u8) << 1); }
    }
}
