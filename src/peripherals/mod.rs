use stm32f401x::*;

pub mod dbgmcu;
pub mod embedded_flash;
pub mod gpio;
pub mod pwr;
pub mod rcc;

pub const TIM1: *mut TIM_TypeDef = TIM1_BASE as *mut TIM_TypeDef;
pub const TIM2: *mut TIM_TypeDef = TIM2_BASE as *mut TIM_TypeDef;
pub const TIM3: *mut TIM_TypeDef = TIM3_BASE as *mut TIM_TypeDef;
pub const TIM4: *mut TIM_TypeDef = TIM4_BASE as *mut TIM_TypeDef;
pub const TIM5: *mut TIM_TypeDef = TIM5_BASE as *mut TIM_TypeDef;
pub const TIM9: *mut TIM_TypeDef = TIM9_BASE as *mut TIM_TypeDef;
pub const TIM10: *mut TIM_TypeDef = TIM10_BASE as *mut TIM_TypeDef;
pub const TIM11: *mut TIM_TypeDef = TIM11_BASE as *mut TIM_TypeDef;

pub const RTC: *mut RTC_TypeDef = RTC_BASE as *mut RTC_TypeDef;

pub const WWDG: *mut WWDG_TypeDef = WWDG_BASE as *mut WWDG_TypeDef;
pub const IWDG: *mut IWDG_TypeDef = IWDG_BASE as *mut IWDG_TypeDef;

pub const I2S2ext: *mut SPI_TypeDef = I2S2ext_BASE as *mut SPI_TypeDef;
pub const I2S3ext: *mut SPI_TypeDef = I2S3ext_BASE as *mut SPI_TypeDef;

pub const SPI1: *mut SPI_TypeDef = SPI1_BASE as *mut SPI_TypeDef;
pub const SPI2: *mut SPI_TypeDef = SPI2_BASE as *mut SPI_TypeDef;
pub const SPI3: *mut SPI_TypeDef = SPI3_BASE as *mut SPI_TypeDef;
pub const SPI4: *mut SPI_TypeDef = SPI4_BASE as *mut SPI_TypeDef;

pub const USART1: *mut USART_TypeDef = USART1_BASE as *mut USART_TypeDef;
pub const USART2: *mut USART_TypeDef = USART2_BASE as *mut USART_TypeDef;
pub const USART6: *mut USART_TypeDef = USART6_BASE as *mut USART_TypeDef;

pub const I2C1: *mut I2C_TypeDef = I2C1_BASE as *mut I2C_TypeDef;
pub const I2C2: *mut I2C_TypeDef = I2C2_BASE as *mut I2C_TypeDef;
pub const I2C3: *mut I2C_TypeDef = I2C3_BASE as *mut I2C_TypeDef;

pub const ADC1: *mut ADC_TypeDef = ADC1_BASE as *mut ADC_TypeDef;
pub const ADC1_COMMON: *mut ADC_Common_TypeDef = ADC1_COMMON_BASE as *mut ADC_Common_TypeDef;

pub const SDIO: *mut SDIO_TypeDef = SDIO_BASE as *mut SDIO_TypeDef;

pub const SYSCFG: *mut SYSCFG_TypeDef = SYSCFG_BASE as *mut SYSCFG_TypeDef;
pub const EXTI: *mut EXTI_TypeDef = EXTI_BASE as *mut EXTI_TypeDef;

pub const CRC: *mut CRC_TypeDef = CRC_BASE as *mut CRC_TypeDef;

pub const DMA1: *mut DMA_TypeDef = DMA1_BASE as *mut DMA_TypeDef;
pub const DMA1_Stream0: *mut DMA_Stream_TypeDef = DMA1_Stream0_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream1: *mut DMA_Stream_TypeDef = DMA1_Stream1_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream2: *mut DMA_Stream_TypeDef = DMA1_Stream2_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream3: *mut DMA_Stream_TypeDef = DMA1_Stream3_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream4: *mut DMA_Stream_TypeDef = DMA1_Stream4_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream5: *mut DMA_Stream_TypeDef = DMA1_Stream5_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream6: *mut DMA_Stream_TypeDef = DMA1_Stream6_BASE as *mut DMA_Stream_TypeDef;
pub const DMA1_Stream7: *mut DMA_Stream_TypeDef = DMA1_Stream7_BASE as *mut DMA_Stream_TypeDef;

pub const DMA2: *mut DMA_TypeDef = DMA2_BASE as *mut DMA_TypeDef;
pub const DMA2_Stream0: *mut DMA_Stream_TypeDef = DMA2_Stream0_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream1: *mut DMA_Stream_TypeDef = DMA2_Stream1_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream2: *mut DMA_Stream_TypeDef = DMA2_Stream2_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream3: *mut DMA_Stream_TypeDef = DMA2_Stream3_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream4: *mut DMA_Stream_TypeDef = DMA2_Stream4_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream5: *mut DMA_Stream_TypeDef = DMA2_Stream5_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream6: *mut DMA_Stream_TypeDef = DMA2_Stream6_BASE as *mut DMA_Stream_TypeDef;
pub const DMA2_Stream7: *mut DMA_Stream_TypeDef = DMA2_Stream7_BASE as *mut DMA_Stream_TypeDef;

