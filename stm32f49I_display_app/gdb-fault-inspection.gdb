set pagination off
set language c
monitor halt
x/wx 0xE000ED28
x/wx 0xE000ED2C
x/wx 0xE000ED38
x/wx 0xE000ED3C
x/wx 0xE000ED08
set $cfsr = *(unsigned int*)0xE000ED28
set $hfsr = *(unsigned int*)0xE000ED2C
set $mmfar = *(unsigned int*)0xE000ED38
set $bfar = *(unsigned int*)0xE000ED3C
set $vtor = *(unsigned int*)0xE000ED08
printf "\nCFSR=%#x HFSR=%#x MMFAR=%#x BFAR=%#x VTOR=%#x\n", $cfsr, $hfsr, $mmfar, $bfar, $vtor
