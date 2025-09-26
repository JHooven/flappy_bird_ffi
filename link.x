MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 192K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    KEEP(*(.vector_table));
  } > FLASH

  .text :
  {
    *(.text .text.*);
    *(.rodata .rodata.*);
  } > FLASH

  .data : AT(LOADADDR(.text) + SIZEOF(.text))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    _ebss = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*);
  }
}