
ENTRY(_reset);

/*Define memory regions */

MEMORY
{
/*  FLASH     (rx)  : ORIGIN = 0x08000000, LENGTH = 2048K*/
  FLASH     (rx)  : ORIGIN = 0x08000000, LENGTH = 2M
  RAM       (rwx) : ORIGIN = 0x20000000, LENGTH = 112K
  CCMRAM    (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
  /* BATTRAM   (rw)  : ORIGIN = 0x40024000, LENGTH = 4K   */        /*Battery backed RAM */
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
    .vector_table : {
        KEEP(*(.vector_table));
    } > FLASH

    .text : {
        *(.text);
        *(.text.*);
        *(.rodata);
        *(.rodata.*);
        . = ALIGN(4);
    } > FLASH

    .data : AT(LOADADDR(.text) + SIZEOF(.text)) {
        _sdata = .;
        *(.data);
        *(.data.*);
        . = ALIGN(4);
        _edata = .;
    } > RAM

    .bss : {
        _sbss = .;
        *(.bss);
        *(.bss.*);
        *(COMMON);
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .stack : {
        . = ALIGN(8);
        _estack = .;
    } > RAM
}
