#![allow(missing_abi)]

use core::ptr;
//define the vector table for the mcu
unsafe extern "C" {
   fn mem_manage_handler();
    fn busfault_handler();
    fn usagefault_handler();
    fn SVCall_Handler();
    fn Debug_Monitor_Handler();
    fn PendSV_Handler();
    fn SysTick_Handler();
    fn WWDG_Handler();
    fn PVD_Handler();
    fn TAMPER_STAMP_Handler();
    fn RTC_WKUP_Handler();
    fn FLASH_Handler();
    fn RCC_Handler();
    fn EXTI0_Handler();
    fn EXTI1_Handler();
    fn EXTI2_TS_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn DMA1_Stream0_Handler();
    fn DMA1_Stream1_Handler();
    fn DMA1_Stream2_Handler();
    fn DMA1_Stream3_Handler();
    fn DMA1_Stream4_Handler();
    fn DMA1_Stream5_Handler();
    fn DMA1_Stream6_Handler();
    fn DMA1_Stream7_Handler();
    fn ADC_Handler();
    fn CAN1_TX_Handler();
    fn CAN1_RX0_Handler();
    fn CAN1_RX1_Handler();
    fn CAN1_SCE_Handler();
    fn EXTI9_5_Handler();
    fn TIM1_BRK_TIM9_Handler();
    fn TIM1_UP_TIM10_Handler();
    fn TIM1_TRG_COM_TIM17_Handler();
    fn TIM1_CC_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn I2C1_EV_Handler();
    fn I2C1_ER_Handler();
    fn I2C2_EV_Handler();
    fn I2C2_ER_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn USART1_Handler();
    fn USART2_Handler();
    fn USART3_Handler();
    fn EXTI15_10_Handler();
    fn RTC_Alarm_Handler();
    fn OTG_FS_WKUP_Handler();
    fn TIM8_BRK_TIM12_Handler();
    fn TIM8_UP_TIM13_Handler();
    fn TIM8_TRG_COM_TIM14_Handler();
    fn TIM8_CC_Handler();
    fn FSMC_Handler();
    fn SDIO_Handler();
    fn TIM5_Handler();
    fn SPI3_Handler();
    fn UART4_Handler();
    fn UART5_Handler();
    fn TIM6_DAC_Handler();
    fn TIM7_Handler();
    fn DMA2_Stream0_Handler();
    fn DMA2_Stream1_Handler();
    fn DMA2_Stream2_Handler();
    fn DMA2_Stream3_Handler();
    fn DMA2_Stream4_Handler();
    fn ETH_Handler();
    fn ETH_WKUP_Handler();
    fn CAN2_TX_Handler();
    fn CAN2_RX0_Handler();
    fn CAN2_RX1_Handler();
    fn CAN2_SCE_Handler();
    fn OTG_FS_Handler();
    fn DMA2_Stream5_Handler();
    fn DMA2_Stream6_Handler();
    fn DMA2_Stream7_Handler();
    fn USART6_Handler();
    fn I2C3_EV_Handler();
    fn I2C3_ER_Handler();
    fn OTG_HS_EP1_OUT_Handler();
    fn OTG_HS_EP1_IN_Handler();
    fn OTG_HS_WKUP_Handler();
    fn OTG_HS_Handler();
    fn DCMI_Handler();
    fn CRYP_Handler();
    fn HASH_RNG_Handler();
    fn FPU_Handler();
    fn UART7_Handler();
    fn UART8_Handler();
    fn SPI4_Handler();
    fn SPI5_Handler();
    fn SPI6_Handler();
    fn SAI1_Handler();
    fn LCD_TFT_Handler();
    fn LCD_TFT_1_Handler();
    fn DMA2D_Handler();

}

#[unsafe(link_section = ".isr_vector")]
#[used]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 106] = [
  Some(reset_handler),
    Some(nmi_handler),
    Some(hardfault_handler),
    Some(mem_manage_handler),
    Some(busfault_handler),
    Some(usagefault_handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    Some(Debug_Monitor_Handler),
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMPER_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_TS_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Stream0_Handler),
    Some(DMA1_Stream1_Handler),
    Some(DMA1_Stream2_Handler),
    Some(DMA1_Stream3_Handler),
    Some(DMA1_Stream4_Handler),
    Some(DMA1_Stream5_Handler),
    Some(DMA1_Stream6_Handler),
    Some(ADC_Handler),
    Some(CAN1_TX_Handler),
    Some(CAN1_RX0_Handler),
    Some(CAN1_RX1_Handler),
    Some(CAN1_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM9_Handler),
    Some(TIM1_UP_TIM10_Handler),
    Some(TIM1_TRG_COM_TIM17_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTC_Alarm_Handler),
    Some(OTG_FS_WKUP_Handler),
    Some(TIM8_BRK_TIM12_Handler),
    Some(TIM8_UP_TIM13_Handler),
    Some(TIM8_TRG_COM_TIM14_Handler),
    Some(TIM8_CC_Handler),
    Some(DMA1_Stream7_Handler),
    Some(FSMC_Handler),
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_DAC_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Stream0_Handler),
    Some(DMA2_Stream1_Handler),
    Some(DMA2_Stream2_Handler),
    Some(DMA2_Stream3_Handler),
    Some(DMA2_Stream4_Handler),
    Some(ETH_Handler),
    Some(ETH_WKUP_Handler),
    Some(CAN2_TX_Handler),
    Some(CAN2_RX0_Handler),
    Some(CAN2_RX1_Handler),
    Some(CAN2_SCE_Handler),
    Some(OTG_FS_Handler),
    Some(DMA2_Stream5_Handler),
    Some(DMA2_Stream6_Handler),
    Some(DMA2_Stream7_Handler),
    Some(USART6_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(OTG_HS_EP1_OUT_Handler),
    Some(OTG_HS_EP1_IN_Handler),
    Some(OTG_HS_WKUP_Handler),
    Some(OTG_HS_Handler),
    Some(DCMI_Handler),
    Some(CRYP_Handler),
    Some(HASH_RNG_Handler),
    Some(FPU_Handler),
    Some(UART7_Handler),
    Some(UART8_Handler),
    Some(SPI4_Handler),
    Some(SPI5_Handler),
    Some(SPI6_Handler),
    Some(SAI1_Handler),
    Some(LCD_TFT_Handler),
    Some(LCD_TFT_1_Handler),
    Some(DMA2D_Handler),
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
extern "C" fn reset_handler() {
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
