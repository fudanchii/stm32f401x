use rcc;
use stm32f401x::*;

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
    fn enable(&'static self, reg: Reg) -> PinMode;
}

pub struct Input<Group: 'static>(Reg, &'static Group);
pub struct Output<Group: 'static>(Reg, &'static Group);
pub struct Analog<Group: 'static>(Reg, &'static Group);
pub struct Alternate<Group: 'static>(Reg, &'static Group);

pub struct A();
pub struct B();
pub struct C();
pub struct D();
pub struct E();
pub struct H();

impl A {
    fn enable() -> A {
        rcc::AHB1::enable_gpioa();
        A()
    }
}

impl B {
    fn enable() -> B {
        rcc::AHB1::enable_gpiob();
        B()
    }
}

impl C {
    fn enable() -> C {
        rcc::AHB1::enable_gpioc();
        C()
    }
}

impl D {
    fn enable() -> D {
        rcc::AHB1::enable_gpiod();
        D()
    }
}

impl E {
    fn enable() -> E {
        rcc::AHB1::enable_gpioe();
        E()
    }
}

macro_rules! impl_gpio {
    ($x:ident,$group:ident) => {
        impl $x {
            fn float(pin: &u8) {
                unsafe {
                    let pupdr = &(*$group).PUPDR;
                    (*$group).PUPDR ^= pupdr & (3u32 << (pin << 1));
                }
            }

            fn pullup(pin: &u8) {
                unsafe {
                    let pupdr = &(*$group).PUPDR;
                    let tmp = (pupdr & (3u32 << (pin << 1))) ^ pupdr;
                    (*$group).PUPDR = tmp | (1u32 << (pin << 1));
                }
            }

            fn pulldown(pin: &u8) {
                unsafe {
                    let pupdr = &(*$group).PUPDR;
                    let tmp = (pupdr & (3u32 << (pin << 1))) ^ pupdr;
                    (*$group).PUPDR = tmp | (2u32 << (pin << 1));
                }
            }

            fn push_pull(pin: &u8) {
                unsafe {
                    let otyper = &(*$group).OTYPER;
                    (*$group).OTYPER ^= otyper & (1u32 << pin);
                }
            }

            fn open_drain(pin: &u8) {
                unsafe {
                    (*$group).OTYPER |= 1u32 << pin;
                }
            }

            fn speed_low(pin: &u8) {
                unsafe {
                    let ospeedr = &(*$group).OSPEEDR;
                    (*$group).OSPEEDR ^= ospeedr & (3u32 << (pin << 1));
                }
            }

            fn speed_medium(pin: &u8) {
                unsafe {
                    let ospeedr = &(*$group).OSPEEDR;
                    (*$group).OSPEEDR ^= ospeedr & (3u32 << (pin << 1));
                    (*$group).OSPEEDR |= 1u32 << (pin << 1);
                }
            }

            fn speed_fast(pin: &u8) {
                unsafe {
                    let ospeedr = &(*$group).OSPEEDR;
                    (*$group).OSPEEDR ^= ospeedr & (3u32 << (pin << 1));
                    (*$group).OSPEEDR |= 2u32 << (pin << 1);
                }
            }

            fn speed_high(pin: &u8) {
                unsafe {
                    (*$group).OSPEEDR |= 3u32 << (pin << 1);
                }
            }
        }

        impl Pin<Output<$x>> for $x {
            fn enable(&'static self, reg: Reg) -> Output<$x> {
                unsafe {
                    let moder = &(*$group).MODER;
                    (*$group).MODER ^= moder & (3 << ((reg as u8) << 1));
                    (*$group).MODER |= 1 << ((reg as u8) << 1);
                }
                Output(reg, self)
            }
        }

        impl Pin<Input<$x>> for $x {
            fn enable(&'static self, reg: Reg) -> Input<$x> {
                unsafe {
                    let moder = &(*$group).MODER;
                    (*$group).MODER ^= moder & (3 << ((reg as u8) << 1));
                }
                Input(reg, self)
            }
        }

        impl Pin<Alternate<$x>> for $x {
            fn enable(&'static self, reg: Reg) -> Alternate<$x> {
                unsafe {
                    let moder = &(*$group).MODER;
                    (*$group).MODER ^= moder & (3 << ((reg as u8) << 1));
                    (*$group).MODER |= 2 << ((reg as u8) << 1);
                }
                Alternate(reg, self)
            }
        }

        impl Pin<Analog<$x>> for $x {
            fn enable(&'static self, reg: Reg) -> Analog<$x> {
                unsafe {
                    (*$group).MODER |= 3 << ((reg as u8) << 1);
                }
                Analog(reg, self)
            }
        }

        impl Output<$x> {
            pub fn set_high(&self) {
                unsafe {
                    (*$group).BSRR = 1 << self.0 as u8;
                }
            }

            pub fn set_low(&self) {
                unsafe {
                    (*$group).BSRR = 1 << (self.0 as u8 + 16);
                }
            }

            pub fn is_high(&self) -> bool {
                let loc = 1 << self.0 as u32;
                unsafe { (*$group).ODR & loc == loc }
            }

            pub fn is_low(&self) -> bool {
                !self.is_high()
            }

            pub fn pullup(&self) -> &Self {
                $x::pullup(&(self.0 as u8));
                self
            }

            pub fn pulldown(&self) -> &Self {
                $x::pulldown(&(self.0 as u8));
                self
            }

            pub fn float(&self) -> &Self {
                $x::float(&(self.0 as u8));
                self
            }

            pub fn push_pull(&self) -> &Self {
                $x::push_pull(&(self.0 as u8));
                self
            }

            pub fn open_drain(&self) -> &Self {
                $x::open_drain(&(self.0 as u8));
                self
            }

            pub fn speed_low(&self) -> &Self {
                $x::speed_low(&(self.0 as u8));
                self
            }

            pub fn speed_medium(&self) -> &Self {
                $x::speed_medium(&(self.0 as u8));
                self
            }

            pub fn speed_fast(&self) -> &Self {
                $x::speed_fast(&(self.0 as u8));
                self
            }

            pub fn speed_high(&self) -> &Self {
                $x::speed_high(&(self.0 as u8));
                self
            }
        }

        impl Input<$x> {
            pub fn pullup(&self) -> &Self {
                $x::pullup(&(self.0 as u8));
                self
            }

            pub fn pulldown(&self) -> &Self {
                $x::pulldown(&(self.0 as u8));
                self
            }

            pub fn float(&self) -> &Self {
                $x::float(&(self.0 as u8));
                self
            }

            pub fn is_high(&self) -> bool {
                let loc = 1 << self.0 as u32;
                unsafe { (*$group).IDR & loc == loc }
            }

            pub fn is_low(&self) -> bool {
                !self.is_high()
            }
        }

        impl Alternate<$x> {
            pub fn pullup(&self) -> &Self {
                $x::pullup(&(self.0 as u8));
                self
            }

            pub fn pulldown(&self) -> &Self {
                $x::pulldown(&(self.0 as u8));
                self
            }

            pub fn float(&self) -> &Self {
                $x::float(&(self.0 as u8));
                self
            }

            pub fn push_pull(&self) -> &Self {
                $x::push_pull(&(self.0 as u8));
                self
            }

            pub fn open_drain(&self) -> &Self {
                $x::open_drain(&(self.0 as u8));
                self
            }

            pub fn speed_low(&self) -> &Self {
                $x::speed_low(&(self.0 as u8));
                self
            }

            pub fn speed_medium(&self) -> &Self {
                $x::speed_medium(&(self.0 as u8));
                self
            }

            pub fn speed_fast(&self) -> &Self {
                $x::speed_fast(&(self.0 as u8));
                self
            }

            pub fn speed_high(&self) -> &Self {
                $x::speed_high(&(self.0 as u8));
                self
            }
        }
    };
}

impl_gpio!(A, GPIOA);
impl_gpio!(B, GPIOB);
impl_gpio!(C, GPIOC);
impl_gpio!(D, GPIOD);
impl_gpio!(E, GPIOE);
impl_gpio!(H, GPIOH);
