/*
 * STM32F401XE
 *
 * Linker script to configure memory regions. 
 * Need modifying for a specific board. 
 *   FLASH.ORIGIN: starting address of flash
 *   FLASH.LENGTH: length of flash
 *   RAM.ORIGIN: starting address of RAM bank 0
 *   RAM.LENGTH: length of RAM bank 0
 */

MEMORY {
  FLASH             (rx) : ORIGIN = 0x08000000, LENGTH = 512K
  RAM              (rwx) : ORIGIN = 0x20000000, LENGTH = 96K
}

/* Linker script to place sections and symbol values. Should be used together
 * with other linker script that defines memory regions FLASH and RAM.
 * It references following symbols, which must be defined in code:
 *   Reset_Handler : Entry of reset handler
 * 
 * It defines following symbols, which code can use without definition:
 *   __exidx_start
 *   __exidx_end
 *   __copy_table_start__
 *   __copy_table_end__
 *   __zero_table_start__
 *   __zero_table_end__
 *   __etext
 *   __data_start__
 *   __preinit_array_start
 *   __preinit_array_end
 *   __init_array_start
 *   __init_array_end
 *   __fini_array_start
 *   __fini_array_end
 *   __data_end__
 *   __bss_start__
 *   __bss_end__
 *   __end__
 *   end
 *   __HeapLimit
 *   __StackLimit
 *   __StackTop
 *   __stack
 */
ENTRY(reset_handler);
EXTERN(__RESET_VECTOR);

EXTERN(default_handler);

EXTERN(NMI_Handler);
EXTERN(HardFault_Handler);
EXTERN(MemManage_Handler);
EXTERN(BusFault_Handler);
EXTERN(UsageFault_Handler);
EXTERN(SVC_Handler);
EXTERN(DebugMon_Handler);
EXTERN(PendSV_Handler);
EXTERN(SysTick_Handler);
EXTERN(WWDG_IRQHandler);                    /* Window WatchDog              */
EXTERN(PVD_IRQHandler);                     /* PVD through EXTI Line detection */
EXTERN(TAMP_STAMP_IRQHandler);              /* Tamper and TimeStamps through the EXTI line */
EXTERN(RTC_WKUP_IRQHandler);                /* RTC Wakeup through the EXTI line */
EXTERN(FLASH_IRQHandler);                   /* FLASH                        */
EXTERN(RCC_IRQHandler);                     /* RCC                          */
EXTERN(EXTI0_IRQHandler);                   /* EXTI Line0                   */
EXTERN(EXTI1_IRQHandler);                   /* EXTI Line1                   */
EXTERN(EXTI2_IRQHandler);                   /* EXTI Line2                   */
EXTERN(EXTI3_IRQHandler);                   /* EXTI Line3                   */
EXTERN(EXTI4_IRQHandler);                   /* EXTI Line4                   */
EXTERN(DMA1_Stream0_IRQHandler);            /* DMA1 Stream 0                */
EXTERN(DMA1_Stream1_IRQHandler);            /* DMA1 Stream 1                */
EXTERN(DMA1_Stream2_IRQHandler);            /* DMA1 Stream 2                */
EXTERN(DMA1_Stream3_IRQHandler);            /* DMA1 Stream 3                */
EXTERN(DMA1_Stream4_IRQHandler);            /* DMA1 Stream 4                */
EXTERN(DMA1_Stream5_IRQHandler);            /* DMA1 Stream 5                */
EXTERN(DMA1_Stream6_IRQHandler);            /* DMA1 Stream 6                */
EXTERN(ADC_IRQHandler);                     /* ADC1); ADC2 and ADC3s   */
EXTERN(EXTI9_5_IRQHandler);                 /* External Line[9:5]s          */
EXTERN(TIM1_BRK_TIM9_IRQHandler);           /* TIM1 Break and TIM9          */
EXTERN(TIM1_UP_TIM10_IRQHandler);           /* TIM1 Update and TIM10        */
EXTERN(TIM1_TRG_COM_TIM11_IRQHandler);      /* TIM1 Trigger and Commutation and TIM11 */
EXTERN(TIM1_CC_IRQHandler);                 /* TIM1 Capture Compare         */
EXTERN(TIM2_IRQHandler);                    /* TIM2                         */
EXTERN(TIM3_IRQHandler);                    /* TIM3                         */
EXTERN(TIM4_IRQHandler);                    /* TIM4                         */
EXTERN(I2C1_EV_IRQHandler);                 /* I2C1 Event                   */
EXTERN(I2C1_ER_IRQHandler);                 /* I2C1 Error                   */
EXTERN(I2C2_EV_IRQHandler);                 /* I2C2 Event                   */
EXTERN(I2C2_ER_IRQHandler);                 /* I2C2 Error                   */
EXTERN(SPI1_IRQHandler);                    /* SPI1                         */
EXTERN(SPI2_IRQHandler);                    /* SPI2                         */
EXTERN(USART1_IRQHandler);                  /* USART1                       */
EXTERN(USART2_IRQHandler);                  /* USART2                       */
EXTERN(EXTI15_10_IRQHandler);               /* External Line[15:10]s        */
EXTERN(RTC_Alarm_IRQHandler);               /* RTC Alarm (A and B) through EXTI Line */
EXTERN(OTG_FS_WKUP_IRQHandler);             /* USB OTG FS Wakeup through EXTI line */
EXTERN(DMA1_Stream7_IRQHandler);            /* DMA1 Stream7                 */
EXTERN(SDIO_IRQHandler);                    /* SDIO                         */
EXTERN(TIM5_IRQHandler);                    /* TIM5                         */
EXTERN(SPI3_IRQHandler);                    /* SPI3                         */
EXTERN(DMA2_Stream0_IRQHandler);            /* DMA2 Stream 0                */
EXTERN(DMA2_Stream1_IRQHandler);            /* DMA2 Stream 1                */
EXTERN(DMA2_Stream2_IRQHandler);            /* DMA2 Stream 2                */
EXTERN(DMA2_Stream3_IRQHandler);            /* DMA2 Stream 3                */
EXTERN(DMA2_Stream4_IRQHandler);            /* DMA2 Stream 4                */
EXTERN(OTG_FS_IRQHandler);                  /* USB OTG FS                   */
EXTERN(DMA2_Stream5_IRQHandler);            /* DMA2 Stream 5                */
EXTERN(DMA2_Stream6_IRQHandler);            /* DMA2 Stream 6                */
EXTERN(DMA2_Stream7_IRQHandler);            /* DMA2 Stream 7                */
EXTERN(USART6_IRQHandler);                  /* USART6                       */
EXTERN(I2C3_EV_IRQHandler);                 /* I2C3 event                   */
EXTERN(I2C3_ER_IRQHandler);                 /* I2C3 error                   */
EXTERN(FPU_IRQHandler);                     /* FPU                          */
EXTERN(SPI4_IRQHandler);                    /* SPI4                         */


PROVIDE(NMI_Handler = default_handler);
PROVIDE(HardFault_Handler = default_handler);
PROVIDE(MemManage_Handler = default_handler);
PROVIDE(BusFault_Handler = default_handler);
PROVIDE(UsageFault_Handler = default_handler);
PROVIDE(SVC_Handler = default_handler);
PROVIDE(DebugMon_Handler = default_handler);
PROVIDE(PendSV_Handler = default_handler);
PROVIDE(SysTick_Handler = default_handler);
PROVIDE(WWDG_IRQHandler = default_handler);                    /* Window WatchDog              */
PROVIDE(PVD_IRQHandler = default_handler);                     /* PVD through EXTI Line detection */
PROVIDE(TAMP_STAMP_IRQHandler = default_handler);              /* Tamper and TimeStamps through the EXTI line */
PROVIDE(RTC_WKUP_IRQHandler = default_handler);                /* RTC Wakeup through the EXTI line */
PROVIDE(FLASH_IRQHandler = default_handler);                   /* FLASH                        */
PROVIDE(RCC_IRQHandler = default_handler);                     /* RCC                          */
PROVIDE(EXTI0_IRQHandler = default_handler);                   /* EXTI Line0                   */
PROVIDE(EXTI1_IRQHandler = default_handler);                   /* EXTI Line1                   */
PROVIDE(EXTI2_IRQHandler = default_handler);                   /* EXTI Line2                   */
PROVIDE(EXTI3_IRQHandler = default_handler);                   /* EXTI Line3                   */
PROVIDE(EXTI4_IRQHandler = default_handler);                   /* EXTI Line4                   */
PROVIDE(DMA1_Stream0_IRQHandler = default_handler);            /* DMA1 Stream 0                */
PROVIDE(DMA1_Stream1_IRQHandler = default_handler);            /* DMA1 Stream 1                */
PROVIDE(DMA1_Stream2_IRQHandler = default_handler);            /* DMA1 Stream 2                */
PROVIDE(DMA1_Stream3_IRQHandler = default_handler);            /* DMA1 Stream 3                */
PROVIDE(DMA1_Stream4_IRQHandler = default_handler);            /* DMA1 Stream 4                */
PROVIDE(DMA1_Stream5_IRQHandler = default_handler);            /* DMA1 Stream 5                */
PROVIDE(DMA1_Stream6_IRQHandler = default_handler);            /* DMA1 Stream 6                */
PROVIDE(ADC_IRQHandler = default_handler);                     /* ADC1 = default_handler); ADC2 and ADC3s   */
PROVIDE(EXTI9_5_IRQHandler = default_handler);                 /* External Line[9:5]s          */
PROVIDE(TIM1_BRK_TIM9_IRQHandler = default_handler);           /* TIM1 Break and TIM9          */
PROVIDE(TIM1_UP_TIM10_IRQHandler = default_handler);           /* TIM1 Update and TIM10        */
PROVIDE(TIM1_TRG_COM_TIM11_IRQHandler = default_handler);      /* TIM1 Trigger and Commutation and TIM11 */
PROVIDE(TIM1_CC_IRQHandler = default_handler);                 /* TIM1 Capture Compare         */
PROVIDE(TIM2_IRQHandler = default_handler);                    /* TIM2                         */
PROVIDE(TIM3_IRQHandler = default_handler);                    /* TIM3                         */
PROVIDE(TIM4_IRQHandler = default_handler);                    /* TIM4                         */
PROVIDE(I2C1_EV_IRQHandler = default_handler);                 /* I2C1 Event                   */
PROVIDE(I2C1_ER_IRQHandler = default_handler);                 /* I2C1 Error                   */
PROVIDE(I2C2_EV_IRQHandler = default_handler);                 /* I2C2 Event                   */
PROVIDE(I2C2_ER_IRQHandler = default_handler);                 /* I2C2 Error                   */
PROVIDE(SPI1_IRQHandler = default_handler);                    /* SPI1                         */
PROVIDE(SPI2_IRQHandler = default_handler);                    /* SPI2                         */
PROVIDE(USART1_IRQHandler = default_handler);                  /* USART1                       */
PROVIDE(USART2_IRQHandler = default_handler);                  /* USART2                       */
PROVIDE(EXTI15_10_IRQHandler = default_handler);               /* External Line[15:10]s        */
PROVIDE(RTC_Alarm_IRQHandler = default_handler);               /* RTC Alarm (A and B) through EXTI Line */
PROVIDE(OTG_FS_WKUP_IRQHandler = default_handler);             /* USB OTG FS Wakeup through EXTI line */
PROVIDE(DMA1_Stream7_IRQHandler = default_handler);            /* DMA1 Stream7                 */
PROVIDE(SDIO_IRQHandler = default_handler);                    /* SDIO                         */
PROVIDE(TIM5_IRQHandler = default_handler);                    /* TIM5                         */
PROVIDE(SPI3_IRQHandler = default_handler);                    /* SPI3                         */
PROVIDE(DMA2_Stream0_IRQHandler = default_handler);            /* DMA2 Stream 0                */
PROVIDE(DMA2_Stream1_IRQHandler = default_handler);            /* DMA2 Stream 1                */
PROVIDE(DMA2_Stream2_IRQHandler = default_handler);            /* DMA2 Stream 2                */
PROVIDE(DMA2_Stream3_IRQHandler = default_handler);            /* DMA2 Stream 3                */
PROVIDE(DMA2_Stream4_IRQHandler = default_handler);            /* DMA2 Stream 4                */
PROVIDE(OTG_FS_IRQHandler = default_handler);                  /* USB OTG FS                   */
PROVIDE(DMA2_Stream5_IRQHandler = default_handler);            /* DMA2 Stream 5                */
PROVIDE(DMA2_Stream6_IRQHandler = default_handler);            /* DMA2 Stream 6                */
PROVIDE(DMA2_Stream7_IRQHandler = default_handler);            /* DMA2 Stream 7                */
PROVIDE(USART6_IRQHandler = default_handler);                  /* USART6                       */
PROVIDE(I2C3_EV_IRQHandler = default_handler);                 /* I2C3 event                   */
PROVIDE(I2C3_ER_IRQHandler = default_handler);                 /* I2C3 error                   */
PROVIDE(FPU_IRQHandler = default_handler);                     /* FPU                          */
PROVIDE(SPI4_IRQHandler = default_handler);                    /* SPI4                         */



SECTIONS
{
    .text :
    {
        LONG(__stack);
        KEEP(*(.isr_vector));
        *(.text*);
    } > FLASH

    .rodata : ALIGN(4)
    {
        *(.rodata*);
        . = ALIGN(4);
    }

    __etext = .;
        
    .data : AT (__etext)
    {
        __data_start__ = .;
        *(.data*)
        . = ALIGN(4);
        /* All data end */
        __data_end__ = .;

    } > RAM

    .bss : ALIGN(4)
    {
        __bss_start__ = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end__ = .;
    } > RAM
    
    .heap (COPY):
    {
        __end__ = .;
        PROVIDE(end = .);
        *(.heap*)
        __HeapLimit = .;
    } > RAM

    /* .stack_dummy section doesn't contains any symbols. It is only
     * used for linker to calculate size of stack sections, and assign
     * values to stack symbols later */
    .stack_dummy (COPY):
    {
        *(.stack*)
    } > RAM

    /* ## Discarded sections */
    /DISCARD/ :
    {
        /* Unused exception related info that only wastes space */
        *(.ARM.exidx.*);
    }

    /* Set stack top to end of RAM, and stack limit move down by
     * size of stack_dummy section */
    __StackTop = ORIGIN(RAM) + LENGTH(RAM);
    __StackLimit = __StackTop - SIZEOF(.stack_dummy);
    PROVIDE(__stack = __StackTop);

    /* Check if data + heap + stack exceeds RAM limit */
    ASSERT(__StackLimit >= __HeapLimit, "region RAM overflowed with stack")
}
