#![allow(missing_abi)]

use core::ptr;

// External interrupt handler implemented in main.rs
unsafe extern "C" {
    fn EXTI0_Handler();
}

// All interrupt handler implementations - these will be linked by the vector table
#[unsafe(no_mangle)]
unsafe extern "C" fn mem_manage_handler() { loop {} }
#[unsafe(no_mangle)]
unsafe extern "C" fn busfault_handler() { loop {} }
#[unsafe(no_mangle)]
unsafe extern "C" fn usagefault_handler() { loop {} }
#[unsafe(no_mangle)]
unsafe extern "C" fn SVCall_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn Debug_Monitor_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn PendSV_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SysTick_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn WWDG_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn PVD_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TAMPER_STAMP_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn RTC_WKUP_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn FLASH_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn RCC_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn EXTI1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn EXTI2_TS_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn EXTI3_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn EXTI4_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream0_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream2_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream3_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream4_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream5_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream6_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA1_Stream7_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn ADC_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN1_TX_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN1_RX0_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN1_RX1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN1_SCE_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn EXTI9_5_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM1_BRK_TIM9_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM1_UP_TIM10_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM1_TRG_COM_TIM17_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM1_CC_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM2_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM3_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM4_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn I2C1_EV_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn I2C1_ER_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn I2C2_EV_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn I2C2_ER_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SPI1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SPI2_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn USART1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn USART2_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn USART3_Handler() {}

// EXTI15_10_Handler is defined in main.rs
unsafe extern "C" {
    fn EXTI15_10_Handler();
}
#[unsafe(no_mangle)]
unsafe extern "C" fn RTC_Alarm_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn OTG_FS_WKUP_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM8_BRK_TIM12_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM8_UP_TIM13_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM8_TRG_COM_TIM14_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM8_CC_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn FSMC_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDIO_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM5_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SPI3_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn UART4_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn UART5_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM6_DAC_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn TIM7_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream0_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream2_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream3_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream4_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn ETH_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn ETH_WKUP_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN2_TX_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN2_RX0_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN2_RX1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CAN2_SCE_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn OTG_FS_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream5_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream6_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2_Stream7_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn USART6_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn I2C3_EV_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn I2C3_ER_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn OTG_HS_EP1_OUT_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn OTG_HS_EP1_IN_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn OTG_HS_WKUP_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn OTG_HS_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DCMI_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn CRYP_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn HASH_RNG_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn FPU_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn UART7_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn UART8_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SPI4_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SPI5_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SPI6_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SAI1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn LCD_TFT_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn LCD_TFT_1_Handler() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn DMA2D_Handler() {}

#[repr(C)]
pub union VectorEntry {
    handler: unsafe extern "C" fn(),
    reserved: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table")]
pub static __VECTOR_TABLE: [VectorEntry; 107] = [
    VectorEntry { reserved: 0x20030000 },                     // 0: Initial stack pointer (192KB RAM end)
    VectorEntry { handler: CustomReset },                     // 1: Reset handler
    VectorEntry { handler: nmi_handler },                     // 2: NMI handler
    VectorEntry { handler: hardfault_handler },               // 3: Hard fault handler
    VectorEntry { handler: mem_manage_handler },              // 4: Memory management fault handler
    VectorEntry { handler: busfault_handler },                // 5: Bus fault handler
    VectorEntry { handler: usagefault_handler },              // 6: Usage fault handler
    VectorEntry { reserved: 0 },                              // 7: Reserved
    VectorEntry { reserved: 0 },                              // 8: Reserved
    VectorEntry { reserved: 0 },                              // 9: Reserved
    VectorEntry { reserved: 0 },                              // 10: Reserved
    VectorEntry { handler: SVCall_Handler },                  // 11: SVC handler
    VectorEntry { handler: Debug_Monitor_Handler },           // 12: Debug monitor handler
    VectorEntry { reserved: 0 },                              // 13: Reserved
    VectorEntry { handler: PendSV_Handler },                  // 14: PendSV handler
    VectorEntry { handler: SysTick_Handler },                 // 15: SysTick handler
    VectorEntry { handler: WWDG_Handler },                    // 16: WWDG handler
    VectorEntry { handler: PVD_Handler },                     // 17: PVD handler
    VectorEntry { handler: TAMPER_STAMP_Handler },            // 18: TAMPER_STAMP handler
    VectorEntry { handler: RTC_WKUP_Handler },                // 19: RTC_WKUP handler
    VectorEntry { handler: FLASH_Handler },                   // 20: FLASH handler
    VectorEntry { handler: RCC_Handler },                     // 21: RCC handler
    VectorEntry { handler: EXTI0_Handler },                   // 22: EXTI0 handler
    VectorEntry { handler: EXTI1_Handler },                   // 23: EXTI1 handler
    VectorEntry { handler: EXTI2_TS_Handler },                // 24: EXTI2_TS handler
    VectorEntry { handler: EXTI3_Handler },                   // 25: EXTI3 handler
    VectorEntry { handler: EXTI4_Handler },                   // 26: EXTI4 handler
    VectorEntry { handler: DMA1_Stream0_Handler },            // 27: DMA1_Stream0 handler
    VectorEntry { handler: DMA1_Stream1_Handler },            // 28: DMA1_Stream1 handler
    VectorEntry { handler: DMA1_Stream2_Handler },            // 29: DMA1_Stream2 handler
    VectorEntry { handler: DMA1_Stream3_Handler },            // 30: DMA1_Stream3 handler
    VectorEntry { handler: DMA1_Stream4_Handler },            // 31: DMA1_Stream4 handler
    VectorEntry { handler: DMA1_Stream5_Handler },            // 32: DMA1_Stream5 handler
    VectorEntry { handler: DMA1_Stream6_Handler },            // 33: DMA1_Stream6 handler
    VectorEntry { handler: ADC_Handler },                     // 34: ADC handler
    VectorEntry { handler: CAN1_TX_Handler },                 // 35: CAN1_TX handler
    VectorEntry { handler: CAN1_RX0_Handler },                // 36: CAN1_RX0 handler
    VectorEntry { handler: CAN1_RX1_Handler },                // 37: CAN1_RX1 handler
    VectorEntry { handler: CAN1_SCE_Handler },                // 38: CAN1_SCE handler
    VectorEntry { handler: EXTI9_5_Handler },                 // 39: EXTI9_5 handler
    VectorEntry { handler: TIM1_BRK_TIM9_Handler },           // 40: TIM1_BRK_TIM9 handler
    VectorEntry { handler: TIM1_UP_TIM10_Handler },           // 41: TIM1_UP_TIM10 handler
    VectorEntry { handler: TIM1_TRG_COM_TIM17_Handler },      // 42: TIM1_TRG_COM_TIM17 handler
    VectorEntry { handler: TIM1_CC_Handler },                 // 43: TIM1_CC handler
    VectorEntry { handler: TIM2_Handler },                    // 44: TIM2 handler
    VectorEntry { handler: TIM3_Handler },                    // 45: TIM3 handler
    VectorEntry { handler: TIM4_Handler },                    // 46: TIM4 handler
    VectorEntry { handler: I2C1_EV_Handler },                 // 47: I2C1_EV handler
    VectorEntry { handler: I2C1_ER_Handler },                 // 48: I2C1_ER handler
    VectorEntry { handler: I2C2_EV_Handler },                 // 49: I2C2_EV handler
    VectorEntry { handler: I2C2_ER_Handler },                 // 50: I2C2_ER handler
    VectorEntry { handler: SPI1_Handler },                    // 51: SPI1 handler
    VectorEntry { handler: SPI2_Handler },                    // 52: SPI2 handler
    VectorEntry { handler: USART1_Handler },                  // 53: USART1 handler
    VectorEntry { handler: USART2_Handler },                  // 54: USART2 handler
    VectorEntry { handler: USART3_Handler },                  // 55: USART3 handler
    VectorEntry { handler: EXTI15_10_Handler },               // 56: EXTI15_10 handler
    VectorEntry { handler: RTC_Alarm_Handler },               // 57: RTC_Alarm handler
    VectorEntry { handler: OTG_FS_WKUP_Handler },             // 58: OTG_FS_WKUP handler
    VectorEntry { handler: TIM8_BRK_TIM12_Handler },          // 59: TIM8_BRK_TIM12 handler
    VectorEntry { handler: TIM8_UP_TIM13_Handler },           // 60: TIM8_UP_TIM13 handler
    VectorEntry { handler: TIM8_TRG_COM_TIM14_Handler },      // 61: TIM8_TRG_COM_TIM14 handler
    VectorEntry { handler: TIM8_CC_Handler },                 // 62: TIM8_CC handler
    VectorEntry { handler: DMA1_Stream7_Handler },            // 63: DMA1_Stream7 handler
    VectorEntry { handler: FSMC_Handler },                    // 64: FSMC handler
    VectorEntry { handler: SDIO_Handler },                    // 65: SDIO handler
    VectorEntry { handler: TIM5_Handler },                    // 66: TIM5 handler
    VectorEntry { handler: SPI3_Handler },                    // 67: SPI3 handler
    VectorEntry { handler: UART4_Handler },                   // 68: UART4 handler
    VectorEntry { handler: UART5_Handler },                   // 69: UART5 handler
    VectorEntry { handler: TIM6_DAC_Handler },                // 70: TIM6_DAC handler
    VectorEntry { handler: TIM7_Handler },                    // 71: TIM7 handler
    VectorEntry { handler: DMA2_Stream0_Handler },            // 72: DMA2_Stream0 handler
    VectorEntry { handler: DMA2_Stream1_Handler },            // 73: DMA2_Stream1 handler
    VectorEntry { handler: DMA2_Stream2_Handler },            // 74: DMA2_Stream2 handler
    VectorEntry { handler: DMA2_Stream3_Handler },            // 75: DMA2_Stream3 handler
    VectorEntry { handler: DMA2_Stream4_Handler },            // 76: DMA2_Stream4 handler
    VectorEntry { handler: ETH_Handler },                     // 77: ETH handler
    VectorEntry { handler: ETH_WKUP_Handler },                // 78: ETH_WKUP handler
    VectorEntry { handler: CAN2_TX_Handler },                 // 79: CAN2_TX handler
    VectorEntry { handler: CAN2_RX0_Handler },                // 80: CAN2_RX0 handler
    VectorEntry { handler: CAN2_RX1_Handler },                // 81: CAN2_RX1 handler
    VectorEntry { handler: CAN2_SCE_Handler },                // 82: CAN2_SCE handler
    VectorEntry { handler: OTG_FS_Handler },                  // 83: OTG_FS handler
    VectorEntry { handler: DMA2_Stream5_Handler },            // 84: DMA2_Stream5 handler
    VectorEntry { handler: DMA2_Stream6_Handler },            // 85: DMA2_Stream6 handler
    VectorEntry { handler: DMA2_Stream7_Handler },            // 86: DMA2_Stream7 handler
    VectorEntry { handler: USART6_Handler },                  // 87: USART6 handler
    VectorEntry { handler: I2C3_EV_Handler },                 // 88: I2C3_EV handler
    VectorEntry { handler: I2C3_ER_Handler },                 // 89: I2C3_ER handler
    VectorEntry { handler: OTG_HS_EP1_OUT_Handler },          // 90: OTG_HS_EP1_OUT handler
    VectorEntry { handler: OTG_HS_EP1_IN_Handler },           // 91: OTG_HS_EP1_IN handler
    VectorEntry { handler: OTG_HS_WKUP_Handler },             // 92: OTG_HS_WKUP handler
    VectorEntry { handler: OTG_HS_Handler },                  // 93: OTG_HS handler
    VectorEntry { handler: DCMI_Handler },                    // 94: DCMI handler
    VectorEntry { handler: CRYP_Handler },                    // 95: CRYP handler
    VectorEntry { handler: HASH_RNG_Handler },                // 96: HASH_RNG handler
    VectorEntry { handler: FPU_Handler },                     // 97: FPU handler
    VectorEntry { handler: UART7_Handler },                   // 98: UART7 handler
    VectorEntry { handler: UART8_Handler },                   // 99: UART8 handler
    VectorEntry { handler: SPI4_Handler },                    // 100: SPI4 handler
    VectorEntry { handler: SPI5_Handler },                    // 101: SPI5 handler
    VectorEntry { handler: SPI6_Handler },                    // 102: SPI6 handler
    VectorEntry { handler: SAI1_Handler },                    // 103: SAI1 handler
    VectorEntry { handler: LCD_TFT_Handler },                 // 104: LCD_TFT handler
    VectorEntry { handler: LCD_TFT_1_Handler },               // 105: LCD_TFT_1 handler
    VectorEntry { handler: DMA2D_Handler },                   // 106: DMA2D handler
];

#[unsafe(no_mangle)]
extern "C" fn nmi_handler() {
    loop { }
}

#[unsafe(no_mangle)]
extern "C" fn hardfault_handler() {
    loop { }
}

#[unsafe(no_mangle)]
extern "C" fn default_handler() {
    loop { }
}



unsafe extern {
    static _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

#[unsafe(no_mangle)]
pub extern "C" fn CustomReset() {
    //reference of static variable to C like raw pointer.
   unsafe {

       let mut src_is_flash: *const u32 = ptr::addr_of!(_sidata);
       let mut  dest_is_ram: *mut u32 = ptr::addr_of_mut!(_sdata); 
       let data_end_in_ram: *mut  u32 = ptr::addr_of_mut!(_edata);
      
       while dest_is_ram < data_end_in_ram {
            *dest_is_ram = *src_is_flash; 
            dest_is_ram = dest_is_ram.add(1);
            src_is_flash = src_is_flash.add(1);
       }

       let mut bss: *mut u32 = ptr::addr_of_mut!(_sbss);
       let bss_end: *mut u32 = ptr::addr_of_mut!(_ebss);
       while bss < bss_end {
           *bss = 0;
           bss = bss.add(1);
       }
       
     }
   

    //3. call main() 
    crate::main();
    
}
