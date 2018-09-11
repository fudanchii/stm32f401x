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
    ModeR0 = 0,
    ModeR1,
    ModeR2,
    ModeR3,
    ModeR4,
    ModeR5,
    ModeR7,
    ModeR8,
    ModeR9,
    ModeR10,
    ModeR11,
    ModeR12,
    ModeR13,
    ModeR14,
    ModeR15,
}

pub enum Mode {
    In = 0,
    Out,
    Alter,
    Analog,
}

pub trait Pin {
    fn enable(reg: Reg, mode: Mode) -> Self;
    fn disable();
    fn on(&self);
    fn off(&self);
    fn set(&self, mode: Mode);
    fn unset(&self, mode: Mode);
}

macro_rules! impl_gpio {
    ($x:ident,$group:ident,$enflag:ident) => {
        impl Pin for $x {
            fn enable(reg: Reg, mode: Mode) -> Self {
                unsafe {
                    (*RCC).AHB1ENR |= $enflag;
                    (*$group).MODER |= (mode as u32) << ((reg as u8) << 1);
                }
                $x(reg)
            }

            fn disable() {
                unsafe { (*RCC).AHB1ENR &= !$enflag; }
            }

            fn on(&self) {
                unsafe { (*$group).BSRR = 1 << (self.0 as u8); }
            }

            fn off(&self) {
                unsafe { (*$group).BSRR = 1 << (self.0 as u8 + 16); }
            }

            fn set(&self, mode: Mode) {
                unsafe { (*$group).MODER |= (mode as u32) << ((self.0 as u8) << 1); }
            }

            fn unset(&self, mode: Mode) {
                unsafe { (*$group).MODER &= !(mode as u32) << ((self.0 as u8) << 1); }
            }

        }
    }
}

#[derive(Copy, Clone)]
pub struct A(Reg);
impl_gpio!(A, GPIOA, RCC_AHB1ENR_GPIOAEN);

#[derive(Copy, Clone)]
pub struct B(Reg);
impl_gpio!(B, GPIOB, RCC_AHB1ENR_GPIOBEN);

#[derive(Copy, Clone)]
pub struct C(Reg);
impl_gpio!(C, GPIOC, RCC_AHB1ENR_GPIOBEN);

#[derive(Copy, Clone)]
pub struct D(Reg);
impl_gpio!(D, GPIOD, RCC_AHB1ENR_GPIOBEN);

#[derive(Copy, Clone)]
pub struct E(Reg);
impl_gpio!(E, GPIOE, RCC_AHB1ENR_GPIOBEN);

#[derive(Copy, Clone)]
pub struct H(Reg);
impl_gpio!(H, GPIOH, RCC_AHB1ENR_GPIOBEN);
