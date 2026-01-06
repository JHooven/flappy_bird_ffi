

/*Define memory regions */

MEMORY
{
  FLASH     (rx)  : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM       (rwx) : ORIGIN = 0x20000000, LENGTH = 192K
  CCMRAM    (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
  /* BATTRAM   (rw)  : ORIGIN = 0x40024000, LENGTH = 4K   */        /*Battery backed RAM */
}
     

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);


