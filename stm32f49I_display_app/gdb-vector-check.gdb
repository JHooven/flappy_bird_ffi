set pagination off
set language c
target extended-remote :3333
monitor halt
set $sp = *(unsigned int*)0x08000000
set $reset = *(unsigned int*)0x08000004
set $vtor = *(unsigned int*)0xE000ED08
printf "InitialSP=%#x ResetHandler=%#x VTOR=%#x\n", $sp, $reset, $vtor
