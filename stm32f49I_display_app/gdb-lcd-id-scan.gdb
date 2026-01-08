# Connect to existing OpenOCD GDB server and probe ILI9341 ID over FMC
set pagination off
set confirm off

target extended-remote :3333

# Continue to let firmware configure FMC and hit its breakpoint
continue

define scan_one
  # args: cmd_addr, data_addr
  set $cmd = $arg0
  set $dat = $arg1
  # Software reset
  set {unsigned short}$cmd = 0x0001
  monitor sleep 10
  # Sleep out
  set {unsigned short}$cmd = 0x0011
  monitor sleep 10
  # Read ID (0xD3)
  set {unsigned short}$cmd = 0x00D3
  x/4bx $dat
end

echo Scanning NE1/NE4 with RS=A16/A17/A18...\n
scan_one 0x60000000 0x60020000
scan_one 0x60000000 0x60040000
scan_one 0x60000000 0x60080000
scan_one 0x6C000000 0x6C020000
scan_one 0x6C000000 0x6C040000
scan_one 0x6C000000 0x6C080000

echo Done.\n