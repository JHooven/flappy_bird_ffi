INCLUDE "device_STM32F429.x"
ENTRY(reset_handler)


/*Define memory regions */

MEMORY
{
  FLASH     (rx)  : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM       (rwx) : ORIGIN = 0x20000000, LENGTH = 192K
  CCMRAM    (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
  /* BATTRAM   (rw)  : ORIGIN = 0x40024000, LENGTH = 4K   */        /*Battery backed RAM */
}
     

SECTIONS 
{


_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;







.text :
{
   . = ALIGN(4);
   LONG(_start_of_stack);
   KEEP(*(.isr_vector));
   *(.text)
   *(.text*)
   . = ALIGN(4);
} > FLASH

.rodata :
{
    . = ALIGN(4);
    *(.rodata)
    *(.rodata*)
    . = ALIGN(4);
} > FLASH 

.data :
{
    _sidata = LOADADDR(.data);  /* This returns the FLASH (LMA) */
    . = ALIGN(4);
    _sdata = .;     /*start of data section in VMA(RAM) */
    *(.data)
    *(.data*)
    . = ALIGN(4);
    _edata = .;
} > RAM AT> FLASH


.bss :
{
    . = ALIGN(4);
    _sbss = .;
    *(.bss)
    *(.bss*)
    . = ALIGN(4);
    _ebss = .;
} > RAM


  .ram_usage_check :
  {
    . = ALIGN(8);
     . = . + _min_stack_size;
     . = . + _min_heap_size;
     . = ALIGN(8);
  } >RAM




}


PROVIDE(mem_manage_handler = default_handler);
PROVIDE(busfault_handler = default_handler);
PROVIDE(usagefault_handler = default_handler);
PROVIDE(SVCall_Handler = default_handler);
PROVIDE(Debug_Monitor_Handler = default_handler);
PROVIDE(PendSV_Handler = default_handler);
PROVIDE(SysTick_Handler = default_handler);
PROVIDE(WWDG_Handler = default_handler);
PROVIDE(PVD_Handler = default_handler);
PROVIDE(TAMPER_STAMP_Handler = default_handler);
PROVIDE(RTC_WKUP_Handler = default_handler);
PROVIDE(FLASH_Handler = default_handler);
PROVIDE(RCC_Handler = default_handler);
PROVIDE(EXTI0_Handler = default_handler);
PROVIDE(EXTI1_Handler = default_handler);
PROVIDE(EXTI2_TS_Handler = default_handler);
PROVIDE(EXTI3_Handler = default_handler);
PROVIDE(EXTI4_Handler = default_handler);
PROVIDE(DMA1_Stream0_Handler = default_handler);
PROVIDE(DMA1_Stream1_Handler = default_handler);
PROVIDE(DMA1_Stream2_Handler = default_handler);
PROVIDE(DMA1_Stream3_Handler = default_handler);
PROVIDE(DMA1_Stream4_Handler = default_handler);
PROVIDE(DMA1_Stream5_Handler = default_handler);
PROVIDE(DMA1_Stream6_Handler = default_handler);
PROVIDE(DMA1_Stream7_Handler = default_handler);
PROVIDE(ADC_Handler = default_handler);
PROVIDE(CAN1_TX_Handler = default_handler);
PROVIDE(CAN1_RX0_Handler = default_handler);
PROVIDE(CAN1_RX1_Handler = default_handler);
PROVIDE(CAN1_SCE_Handler = default_handler);
PROVIDE(EXTI9_5_Handler = default_handler);
PROVIDE(TIM1_BRK_TIM9_Handler = default_handler);
PROVIDE(TIM1_UP_TIM10_Handler = default_handler);
PROVIDE(TIM1_TRG_COM_TIM17_Handler = default_handler);
PROVIDE(TIM1_CC_Handler = default_handler);
PROVIDE(TIM2_Handler = default_handler);
PROVIDE(TIM3_Handler = default_handler);
PROVIDE(TIM4_Handler = default_handler);
PROVIDE(I2C1_EV_Handler = default_handler);
PROVIDE(I2C1_ER_Handler = default_handler);
PROVIDE(I2C2_EV_Handler = default_handler);
PROVIDE(I2C2_ER_Handler = default_handler);
PROVIDE(SPI1_Handler = default_handler);
PROVIDE(SPI2_Handler = default_handler);
PROVIDE(USART1_Handler = default_handler);
PROVIDE(USART2_Handler = default_handler);
PROVIDE(USART3_Handler = default_handler);
PROVIDE(EXTI15_10_Handler = default_handler);
PROVIDE(RTC_Alarm_Handler = default_handler);
PROVIDE(OTG_FS_WKUP_Handler = default_handler);
PROVIDE(TIM8_BRK_TIM12_Handler = default_handler);
PROVIDE(TIM8_UP_TIM13_Handler = default_handler);
PROVIDE(TIM8_TRG_COM_TIM14_Handler = default_handler);
PROVIDE(TIM8_CC_Handler = default_handler);
PROVIDE(FSMC_Handler = default_handler);
PROVIDE(SDIO_Handler = default_handler);
PROVIDE(TIM5_Handler = default_handler);
PROVIDE(SPI3_Handler = default_handler);
PROVIDE(UART4_Handler = default_handler);
PROVIDE(UART5_Handler = default_handler);
PROVIDE(TIM6_DAC_Handler = default_handler);
PROVIDE(TIM7_Handler = default_handler);
PROVIDE(DMA2_Stream0_Handler = default_handler);
PROVIDE(DMA2_Stream1_Handler = default_handler);
PROVIDE(DMA2_Stream2_Handler = default_handler);
PROVIDE(DMA2_Stream3_Handler = default_handler);
PROVIDE(DMA2_Stream4_Handler = default_handler);
PROVIDE(ETH_Handler = default_handler);
PROVIDE(ETH_WKUP_Handler = default_handler);
PROVIDE(CAN2_TX_Handler = default_handler);
PROVIDE(CAN2_RX0_Handler = default_handler);
PROVIDE(CAN2_RX1_Handler = default_handler);
PROVIDE(CAN2_SCE_Handler = default_handler);
PROVIDE(OTG_FS_Handler = default_handler);
PROVIDE(DMA2_Stream5_Handler = default_handler);
PROVIDE(DMA2_Stream6_Handler = default_handler);
PROVIDE(DMA2_Stream7_Handler = default_handler);
PROVIDE(USART6_Handler = default_handler);
PROVIDE(I2C3_EV_Handler = default_handler);
PROVIDE(I2C3_ER_Handler = default_handler);
PROVIDE(OTG_HS_EP1_OUT_Handler = default_handler);
PROVIDE(OTG_HS_EP1_IN_Handler = default_handler);
PROVIDE(OTG_HS_WKUP_Handler = default_handler);
PROVIDE(OTG_HS_Handler = default_handler);
PROVIDE(DCMI_Handler = default_handler);
PROVIDE(CRYP_Handler = default_handler);
PROVIDE(HASH_RNG_Handler = default_handler);
PROVIDE(FPU_Handler = default_handler);
PROVIDE(UART7_Handler = default_handler);
PROVIDE(UART8_Handler = default_handler);
PROVIDE(SPI4_Handler = default_handler);
PROVIDE(SPI5_Handler = default_handler);
PROVIDE(SPI6_Handler = default_handler);
PROVIDE(SAI1_Handler = default_handler);
PROVIDE(LCD_TFT_Handler = default_handler);
PROVIDE(LCD_TFT_1_Handler = default_handler);
PROVIDE(DMA2D_Handler = default_handler);
