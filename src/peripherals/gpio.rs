use stm32f401x::*;
use peripherals::RCC;

const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut GPIO_TypeDef;
const GPIOB: *mut GPIO_TypeDef = GPIOB_BASE as *mut GPIO_TypeDef;
const GPIOC: *mut GPIO_TypeDef = GPIOC_BASE as *mut GPIO_TypeDef;
const GPIOD: *mut GPIO_TypeDef = GPIOD_BASE as *mut GPIO_TypeDef;
const GPIOE: *mut GPIO_TypeDef = GPIOE_BASE as *mut GPIO_TypeDef;
const GPIOH: *mut GPIO_TypeDef = GPIOH_BASE as *mut GPIO_TypeDef;

#[derive(Clone, Copy)]
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

pub trait Pin<PinMode> {
    fn enable(reg: &Reg) -> PinMode;
    fn disable();
}

pub struct Input<Group>(Reg, Group);
pub struct Output<Group>(Reg, Group);

macro_rules! impl_gpio {
    ($x:ident,$group:ident,$enflag:ident) => {
        impl Pin<Output<$x>> for $x {
            fn enable(reg: &Reg) -> Output<$x> {
                unsafe {
                    (*RCC).AHB1ENR |= $enflag;
                    (*$group).MODER |= 1 << ((*reg as u8) << 1);
                }
                Output(*reg, $x())
            }

            fn disable() {
                unsafe { (*RCC).AHB1ENR &= !$enflag; }
            }
        }

        impl Pin<Input<$x>> for $x {
            fn enable(reg: &Reg) -> Input<$x> {
                unsafe {
                    let moder = &(*$group).MODER;
                    (*RCC).AHB1ENR |= $enflag;
                    (*$group).MODER ^= moder & (3 << *reg as u8);
                }
                Input(*reg, $x())
            }

            fn disable() {
                unsafe { (*RCC).AHB1ENR &= !$enflag; }
            }
        }

        impl Output<$x> {
            pub fn on(&self) {
                unsafe { (*$group).BSRR = 1 << self.0 as u8; }
            }

            pub fn off(&self) {
                unsafe { (*$group).BSRR = 1 << (self.0 as u8 + 16); }
            }

            pub fn pullup(&self) -> &Self {
                unsafe {
                    let pupdr = &(*$group).PUPDR;
                    let tmp = (pupdr & (3 << ((self.0 as u8) << 1))) ^ pupdr;
                    (*$group).PUPDR = tmp | (1 << ((self.0 as u8) << 1));
                }
                self
            }

            pub fn pulldown(&self) -> &Self {
                unsafe {
                    let pupdr = &(*$group).PUPDR;
                    let tmp = (pupdr & (3 << ((self.0 as u8) << 1))) ^ pupdr;
                    (*$group).PUPDR = tmp | (2 << ((self.0 as u8) << 1));
                }
                self
            }

            pub fn pull_reset(&self) -> &Self {
                unsafe {
                    let pupdr = &(*$group).PUPDR;
                    (*$group).PUPDR ^= pupdr & (3 << ((self.0 as u8) << 1));
                }
                self
            }

            pub fn push_pull(&self) -> &Self {
                unsafe {
                    let otyper = &(*$group).OTYPER;
                    (*$group).OTYPER ^= otyper & (1 << self.0 as u8); 
                }
                self
            }

            pub fn open_drain(&self) -> &Self {
                unsafe {
                    (*$group).OTYPER |= 1 << self.0 as u8;
                }
                self
            }
        }
    }
}

pub struct A();
impl_gpio!(A, GPIOA, RCC_AHB1ENR_GPIOAEN);

pub struct B();
impl_gpio!(B, GPIOB, RCC_AHB1ENR_GPIOBEN);

pub struct C();
impl_gpio!(C, GPIOC, RCC_AHB1ENR_GPIOBEN);

pub struct D();
impl_gpio!(D, GPIOD, RCC_AHB1ENR_GPIOBEN);

pub struct E();
impl_gpio!(E, GPIOE, RCC_AHB1ENR_GPIOBEN);

pub struct H();
impl_gpio!(H, GPIOH, RCC_AHB1ENR_GPIOBEN);
