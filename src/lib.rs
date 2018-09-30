#![no_std]
#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]
#![feature(naked_functions)]
#![feature(used)]
#![feature(asm)]

extern crate embedded_hal;

use core::{mem, ptr};

pub mod asm;
pub mod ctypes;
pub mod hal;
pub mod peripherals;
pub mod stm32f401x;

use asm::*;
use peripherals::*;

#[naked]
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn system_init() {
    rcc::APB1::enable_pwr();

    while !rcc::hsi_ready() {
        nop();
    }

    rcc::disable_pll();

    pwr::CR::set(
        pwr::VOS::Scale2,
        pwr::ADCDC1::Unset,
        pwr::MRLVDS::Unset,
        pwr::LPLVDS::Unset,
        pwr::FPDS::Unset,
        pwr::DBP::Unset,
        pwr::PLS::Level4,
        pwr::PVDE::Unset,
        pwr::CSBF::Unset,
        pwr::CWUF::Unset,
        pwr::PDDS::Unset,
        pwr::LPDS::Unset,
    );

    rcc::config_pll(rcc::InputClock::HSI);
    rcc::enable_pll();
    rcc::use_pll();

    dbgmcu::CR::set(
        dbgmcu::TraceMode::Async,
        dbgmcu::TraceIO::Disable,
        dbgmcu::DebugStandby::Enable,
        dbgmcu::DebugStop::Enable,
        dbgmcu::DebugSleep::Enable,
    );
}

#[naked]
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn zero_bss() {
    extern "C" {
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
    }

    let mut bss: *mut _ = &mut __bss_start__;
    let ebss: *mut _ = &mut __bss_end__;

    while bss < ebss {
        ptr::write_volatile(bss, mem::zeroed());
        bss = bss.offset(1);
    }
}

#[naked]
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn init_data() {
    extern "C" {
        static mut __data_start__: u32;
        static mut __data_end__: u32;

        static mut __etext: u32;
    }

    let mut data: *mut _ = &mut __data_start__;
    let edata: *mut _ = &mut __data_end__;
    let mut sidata: *mut _ = &mut __etext;

    while data < edata {
        ptr::write(data, ptr::read(sidata));
        data = data.offset(1);
        sidata = sidata.offset(1);
    }
}

extern "Rust" {
    #[inline(never)]
    fn main() -> !;
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn reset_handler() -> ! {
    zero_bss();
    init_data();
    system_init();
    main()
}

pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn default_handler() {
    asm!("bkpt");
}

extern "C" {
    fn NMI_Handler();
    fn HardFault_Handler();
    fn MemManage_Handler();
    fn BusFault_Handler();
    fn UsageFault_Handler();
    fn SVC_Handler();
    fn DebugMon_Handler();
    fn PendSV_Handler();
    fn SysTick_Handler();
    fn WWDG_IRQHandler(); /* Window WatchDog              */
    fn PVD_IRQHandler(); /* PVD through EXTI Line detection */
    fn TAMP_STAMP_IRQHandler(); /* Tamper and TimeStamps through the EXTI line */
    fn RTC_WKUP_IRQHandler(); /* RTC Wakeup through the EXTI line */
    fn FLASH_IRQHandler(); /* FLASH                        */
    fn RCC_IRQHandler(); /* RCC                          */
    fn EXTI0_IRQHandler(); /* EXTI Line0                   */
    fn EXTI1_IRQHandler(); /* EXTI Line1                   */
    fn EXTI2_IRQHandler(); /* EXTI Line2                   */
    fn EXTI3_IRQHandler(); /* EXTI Line3                   */
    fn EXTI4_IRQHandler(); /* EXTI Line4                   */
    fn DMA1_Stream0_IRQHandler(); /* DMA1 Stream 0                */
    fn DMA1_Stream1_IRQHandler(); /* DMA1 Stream 1                */
    fn DMA1_Stream2_IRQHandler(); /* DMA1 Stream 2                */
    fn DMA1_Stream3_IRQHandler(); /* DMA1 Stream 3                */
    fn DMA1_Stream4_IRQHandler(); /* DMA1 Stream 4                */
    fn DMA1_Stream5_IRQHandler(); /* DMA1 Stream 5                */
    fn DMA1_Stream6_IRQHandler(); /* DMA1 Stream 6                */
    fn ADC_IRQHandler(); /* ADC1(); ADC2 and ADC3s   */
    fn EXTI9_5_IRQHandler(); /* External Line[9:5]s          */
    fn TIM1_BRK_TIM9_IRQHandler(); /* TIM1 Break and TIM9          */
    fn TIM1_UP_TIM10_IRQHandler(); /* TIM1 Update and TIM10        */
    fn TIM1_TRG_COM_TIM11_IRQHandler(); /* TIM1 Trigger and Commutation and TIM11 */
    fn TIM1_CC_IRQHandler(); /* TIM1 Capture Compare         */
    fn TIM2_IRQHandler(); /* TIM2                         */
    fn TIM3_IRQHandler(); /* TIM3                         */
    fn TIM4_IRQHandler(); /* TIM4                         */
    fn I2C1_EV_IRQHandler(); /* I2C1 Event                   */
    fn I2C1_ER_IRQHandler(); /* I2C1 Error                   */
    fn I2C2_EV_IRQHandler(); /* I2C2 Event                   */
    fn I2C2_ER_IRQHandler(); /* I2C2 Error                   */
    fn SPI1_IRQHandler(); /* SPI1                         */
    fn SPI2_IRQHandler(); /* SPI2                         */
    fn USART1_IRQHandler(); /* USART1                       */
    fn USART2_IRQHandler(); /* USART2                       */
    fn EXTI15_10_IRQHandler(); /* External Line[15:10]s        */
    fn RTC_Alarm_IRQHandler(); /* RTC Alarm (A and B) through EXTI Line */
    fn OTG_FS_WKUP_IRQHandler(); /* USB OTG FS Wakeup through EXTI line */
    fn DMA1_Stream7_IRQHandler(); /* DMA1 Stream7                 */
    fn SDIO_IRQHandler(); /* SDIO                         */
    fn TIM5_IRQHandler(); /* TIM5                         */
    fn SPI3_IRQHandler(); /* SPI3                         */
    fn DMA2_Stream0_IRQHandler(); /* DMA2 Stream 0                */
    fn DMA2_Stream1_IRQHandler(); /* DMA2 Stream 1                */
    fn DMA2_Stream2_IRQHandler(); /* DMA2 Stream 2                */
    fn DMA2_Stream3_IRQHandler(); /* DMA2 Stream 3                */
    fn DMA2_Stream4_IRQHandler(); /* DMA2 Stream 4                */
    fn OTG_FS_IRQHandler(); /* USB OTG FS                   */
    fn DMA2_Stream5_IRQHandler(); /* DMA2 Stream 5                */
    fn DMA2_Stream6_IRQHandler(); /* DMA2 Stream 6                */
    fn DMA2_Stream7_IRQHandler(); /* DMA2 Stream 7                */
    fn USART6_IRQHandler(); /* USART6                       */
    fn I2C3_EV_IRQHandler(); /* I2C3 event                   */
    fn I2C3_ER_IRQHandler(); /* I2C3 error                   */
    fn FPU_IRQHandler(); /* FPU                          */
    fn SPI4_IRQHandler(); /* SPI4                         */
}

pub union Vector {
    noreturn: unsafe extern "C" fn() -> !,
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[link_section = ".isr_vector"]
#[no_mangle]
#[used]
pub static isr_vector: [Vector; 100] = [
    Vector {
        noreturn: reset_handler,
    },
    Vector {
        handler: NMI_Handler,
    },
    Vector {
        handler: HardFault_Handler,
    },
    Vector {
        handler: MemManage_Handler,
    },
    Vector {
        handler: BusFault_Handler,
    },
    Vector {
        handler: UsageFault_Handler,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: SVC_Handler,
    },
    Vector {
        handler: DebugMon_Handler,
    },
    Vector { reserved: 0 },
    Vector {
        handler: PendSV_Handler,
    },
    Vector {
        handler: SysTick_Handler,
    },
    /* External Interrupts */
    Vector {
        handler: WWDG_IRQHandler,
    }, /* Window WatchDog              */
    Vector {
        handler: PVD_IRQHandler,
    }, /* PVD through EXTI Line detection */
    Vector {
        handler: TAMP_STAMP_IRQHandler,
    }, /* Tamper and TimeStamps through the EXTI line */
    Vector {
        handler: RTC_WKUP_IRQHandler,
    }, /* RTC Wakeup through the EXTI line */
    Vector {
        handler: FLASH_IRQHandler,
    }, /* FLASH                        */
    Vector {
        handler: RCC_IRQHandler,
    }, /* RCC                          */
    Vector {
        handler: EXTI0_IRQHandler,
    }, /* EXTI Line0                   */
    Vector {
        handler: EXTI1_IRQHandler,
    }, /* EXTI Line1                   */
    Vector {
        handler: EXTI2_IRQHandler,
    }, /* EXTI Line2                   */
    Vector {
        handler: EXTI3_IRQHandler,
    }, /* EXTI Line3                   */
    Vector {
        handler: EXTI4_IRQHandler,
    }, /* EXTI Line4                   */
    Vector {
        handler: DMA1_Stream0_IRQHandler,
    }, /* DMA1 Stream 0                */
    Vector {
        handler: DMA1_Stream1_IRQHandler,
    }, /* DMA1 Stream 1                */
    Vector {
        handler: DMA1_Stream2_IRQHandler,
    }, /* DMA1 Stream 2                */
    Vector {
        handler: DMA1_Stream3_IRQHandler,
    }, /* DMA1 Stream 3                */
    Vector {
        handler: DMA1_Stream4_IRQHandler,
    }, /* DMA1 Stream 4                */
    Vector {
        handler: DMA1_Stream5_IRQHandler,
    }, /* DMA1 Stream 5                */
    Vector {
        handler: DMA1_Stream6_IRQHandler,
    }, /* DMA1 Stream 6                */
    Vector {
        handler: ADC_IRQHandler,
    }, /* ADC1, ADC2 and ADC3s         */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: EXTI9_5_IRQHandler,
    }, /* External Line[9:5]s          */
    Vector {
        handler: TIM1_BRK_TIM9_IRQHandler,
    }, /* TIM1 Break and TIM9          */
    Vector {
        handler: TIM1_UP_TIM10_IRQHandler,
    }, /* TIM1 Update and TIM10        */
    Vector {
        handler: TIM1_TRG_COM_TIM11_IRQHandler,
    }, /* TIM1 Trigger and Commutation and TIM11 */
    Vector {
        handler: TIM1_CC_IRQHandler,
    }, /* TIM1 Capture Compare         */
    Vector {
        handler: TIM2_IRQHandler,
    }, /* TIM2                         */
    Vector {
        handler: TIM3_IRQHandler,
    }, /* TIM3                         */
    Vector {
        handler: TIM4_IRQHandler,
    }, /* TIM4                         */
    Vector {
        handler: I2C1_EV_IRQHandler,
    }, /* I2C1 Event                   */
    Vector {
        handler: I2C1_ER_IRQHandler,
    }, /* I2C1 Error                   */
    Vector {
        handler: I2C2_EV_IRQHandler,
    }, /* I2C2 Event                   */
    Vector {
        handler: I2C2_ER_IRQHandler,
    }, /* I2C2 Error                   */
    Vector {
        handler: SPI1_IRQHandler,
    }, /* SPI1                         */
    Vector {
        handler: SPI2_IRQHandler,
    }, /* SPI2                         */
    Vector {
        handler: USART1_IRQHandler,
    }, /* USART1                       */
    Vector {
        handler: USART2_IRQHandler,
    }, /* USART2                       */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: EXTI15_10_IRQHandler,
    }, /* External Line[15:10]s        */
    Vector {
        handler: RTC_Alarm_IRQHandler,
    }, /* RTC Alarm (A and B) through EXTI Line */
    Vector {
        handler: OTG_FS_WKUP_IRQHandler,
    }, /* USB OTG FS Wakeup through EXTI line */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: DMA1_Stream7_IRQHandler,
    }, /* DMA1 Stream7                 */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: SDIO_IRQHandler,
    }, /* SDIO                         */
    Vector {
        handler: TIM5_IRQHandler,
    }, /* TIM5                         */
    Vector {
        handler: SPI3_IRQHandler,
    }, /* SPI3                         */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: DMA2_Stream0_IRQHandler,
    }, /* DMA2 Stream 0                */
    Vector {
        handler: DMA2_Stream1_IRQHandler,
    }, /* DMA2 Stream 1                */
    Vector {
        handler: DMA2_Stream2_IRQHandler,
    }, /* DMA2 Stream 2                */
    Vector {
        handler: DMA2_Stream3_IRQHandler,
    }, /* DMA2 Stream 3                */
    Vector {
        handler: DMA2_Stream4_IRQHandler,
    }, /* DMA2 Stream 4                */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: OTG_FS_IRQHandler,
    }, /* USB OTG FS                   */
    Vector {
        handler: DMA2_Stream5_IRQHandler,
    }, /* DMA2 Stream 5                */
    Vector {
        handler: DMA2_Stream6_IRQHandler,
    }, /* DMA2 Stream 6                */
    Vector {
        handler: DMA2_Stream7_IRQHandler,
    }, /* DMA2 Stream 7                */
    Vector {
        handler: USART6_IRQHandler,
    }, /* USART6                       */
    Vector {
        handler: I2C3_EV_IRQHandler,
    }, /* I2C3 event                   */
    Vector {
        handler: I2C3_ER_IRQHandler,
    }, /* I2C3 error                   */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: FPU_IRQHandler,
    }, /* FPU                          */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector { reserved: 0 }, /* Reserved                     */
    Vector {
        handler: SPI4_IRQHandler,
    }, /* SPI4                         */
];
