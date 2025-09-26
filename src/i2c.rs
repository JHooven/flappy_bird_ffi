use crate::mcu::*;
use crate::reg::*;
use crate::gpio::*;

// I2C1 Register offsets
const I2C_CR1_OFFSET: u32 = 0x00;
const I2C_CR2_OFFSET: u32 = 0x04;
const I2C_OAR1_OFFSET: u32 = 0x08;
const I2C_DR_OFFSET: u32 = 0x10;
const I2C_SR1_OFFSET: u32 = 0x14;
const I2C_SR2_OFFSET: u32 = 0x18;
const I2C_CCR_OFFSET: u32 = 0x1C;
const I2C_TRISE_OFFSET: u32 = 0x20;

// I2C1 Base address
pub const I2C1_BASE: u32 = 0x4000_5400;

// I2C Control Register 1 (CR1) bits
const I2C_CR1_PE: u32 = 0;      // Peripheral Enable
const I2C_CR1_START: u32 = 8;   // Start Generation
const I2C_CR1_STOP: u32 = 9;    // Stop Generation
const I2C_CR1_ACK: u32 = 10;    // Acknowledge Enable

// I2C Status Register 1 (SR1) bits
const I2C_SR1_SB: u32 = 0;      // Start Bit
const I2C_SR1_ADDR: u32 = 1;    // Address sent
const I2C_SR1_TXE: u32 = 7;     // Data register empty
const I2C_SR1_RXNE: u32 = 6;    // Data register not empty
const I2C_SR1_BTF: u32 = 2;     // Byte transfer finished

#[derive(Debug)]
pub enum I2CError {
    Timeout,
    AddressNack,
    DataNack,
}

pub fn i2c_init() {
    // Enable I2C1 and GPIOB clocks
    enable_i2c1_clock();
    enable_gpio_clock(GPIOB_BASE);
    
    // Configure PB8 (SCL) and PB9 (SDA) as alternate function
    configure_i2c_pins();
    
    // Reset I2C1
    reset_i2c1();
    
    // Configure I2C1
    configure_i2c1();
}

fn enable_i2c1_clock() {
    let rcc_apb1enr_addr = (RCC_BASE + 0x40) as *mut u32;
    reg_set_bit(rcc_apb1enr_addr, 21, true); // I2C1EN bit
}

fn configure_i2c_pins() {
    // Configure PB8 and PB9 as alternate function, open-drain, pull-up
    
    // Set mode to alternate function (10)
    set_gpio_mode_alternate_function(GPIOB_BASE, 8);
    set_gpio_mode_alternate_function(GPIOB_BASE, 9);
    
    // Set alternate function to AF4 (I2C1)
    set_gpio_alternate_function(GPIOB_BASE, 8, 4);
    set_gpio_alternate_function(GPIOB_BASE, 9, 4);
    
    // Set output type to open-drain
    set_gpio_output_type_open_drain(GPIOB_BASE, 8);
    set_gpio_output_type_open_drain(GPIOB_BASE, 9);
    
    // Set pull-up
    set_gpio_pull_up(GPIOB_BASE, 8);
    set_gpio_pull_up(GPIOB_BASE, 9);
    
    // Set speed to high
    set_gpio_speed_high(GPIOB_BASE, 8);
    set_gpio_speed_high(GPIOB_BASE, 9);
}

fn reset_i2c1() {
    let rcc_apb1rstr_addr = (RCC_BASE + 0x20) as *mut u32;
    reg_set_bit(rcc_apb1rstr_addr, 21, true);  // Reset I2C1
    reg_set_bit(rcc_apb1rstr_addr, 21, false); // Release reset
}

fn configure_i2c1() {
    let i2c_cr1_addr = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
    let i2c_cr2_addr = (I2C1_BASE + I2C_CR2_OFFSET) as *mut u32;
    let i2c_ccr_addr = (I2C1_BASE + I2C_CCR_OFFSET) as *mut u32;
    let i2c_trise_addr = (I2C1_BASE + I2C_TRISE_OFFSET) as *mut u32;
    let i2c_oar1_addr = (I2C1_BASE + I2C_OAR1_OFFSET) as *mut u32;
    
    // Disable I2C1
    reg_set_bit(i2c_cr1_addr, I2C_CR1_PE, false);
    
    // Configure CR2: Set FREQ field (APB1 frequency in MHz, assuming 42MHz)
    reg_set_bits(i2c_cr2_addr, 42, 0, 6);
    
    // Configure CCR for standard mode (100kHz)
    // CCR = PCLK1 / (2 * I2C_FREQ) = 42MHz / (2 * 100kHz) = 210
    reg_set_val(i2c_ccr_addr, 210);
    
    // Configure TRISE (maximum rise time)
    // TRISE = (maximum rise time / TPCLK1) + 1 = (1000ns / 23.8ns) + 1 = 43
    reg_set_val(i2c_trise_addr, 43);
    
    // Set own address (not used as master, but required)
    reg_set_val(i2c_oar1_addr, 0x00);
    
    // Enable I2C1
    reg_set_bit(i2c_cr1_addr, I2C_CR1_PE, true);
}

pub fn i2c_start() -> Result<(), I2CError> {
    let i2c_cr1_addr = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
    let i2c_sr1_addr = (I2C1_BASE + I2C_SR1_OFFSET) as *mut u32;
    
    // Generate start condition
    reg_set_bit(i2c_cr1_addr, I2C_CR1_START, true);
    
    // Wait for start condition to be generated
    let mut timeout = 100000;
    while !reg_read_bit(i2c_sr1_addr, I2C_SR1_SB) {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2CError::Timeout);
        }
    }
    
    Ok(())
}

pub fn i2c_stop() {
    let i2c_cr1_addr = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
    reg_set_bit(i2c_cr1_addr, I2C_CR1_STOP, true);
}

pub fn i2c_send_address(address: u8, read: bool) -> Result<(), I2CError> {
    let i2c_dr_addr = (I2C1_BASE + I2C_DR_OFFSET) as *mut u32;
    let i2c_sr1_addr = (I2C1_BASE + I2C_SR1_OFFSET) as *mut u32;
    let i2c_sr2_addr = (I2C1_BASE + I2C_SR2_OFFSET) as *mut u32;
    
    // Send address with read/write bit
    let addr_with_rw = (address << 1) | if read { 1 } else { 0 };
    reg_set_val(i2c_dr_addr, addr_with_rw as u32);
    
    // Wait for address to be acknowledged
    let mut timeout = 100000;
    while !reg_read_bit(i2c_sr1_addr, I2C_SR1_ADDR) {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2CError::AddressNack);
        }
    }
    
    // Clear ADDR flag by reading SR1 and SR2
    let _sr1 = reg_get_val(i2c_sr1_addr);
    let _sr2 = reg_get_val(i2c_sr2_addr);
    
    Ok(())
}

pub fn i2c_send_data(data: u8) -> Result<(), I2CError> {
    let i2c_dr_addr = (I2C1_BASE + I2C_DR_OFFSET) as *mut u32;
    let i2c_sr1_addr = (I2C1_BASE + I2C_SR1_OFFSET) as *mut u32;
    
    // Wait for data register to be empty
    let mut timeout = 100000;
    while !reg_read_bit(i2c_sr1_addr, I2C_SR1_TXE) {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2CError::Timeout);
        }
    }
    
    // Send data
    reg_set_val(i2c_dr_addr, data as u32);
    
    // Wait for byte transfer to finish
    timeout = 100000;
    while !reg_read_bit(i2c_sr1_addr, I2C_SR1_BTF) {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2CError::DataNack);
        }
    }
    
    Ok(())
}

pub fn i2c_receive_data(ack: bool) -> Result<u8, I2CError> {
    let i2c_dr_addr = (I2C1_BASE + I2C_DR_OFFSET) as *mut u32;
    let i2c_sr1_addr = (I2C1_BASE + I2C_SR1_OFFSET) as *mut u32;
    let i2c_cr1_addr = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
    
    // Set/clear ACK bit for next byte
    reg_set_bit(i2c_cr1_addr, I2C_CR1_ACK, ack);
    
    // Wait for data to be received
    let mut timeout = 100000;
    while !reg_read_bit(i2c_sr1_addr, I2C_SR1_RXNE) {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2CError::Timeout);
        }
    }
    
    // Read and return data
    Ok(reg_get_val(i2c_dr_addr) as u8)
}

pub fn i2c_write_register(device_addr: u8, reg_addr: u8, data: u8) -> Result<(), I2CError> {
    i2c_start()?;
    i2c_send_address(device_addr, false)?;
    i2c_send_data(reg_addr)?;
    i2c_send_data(data)?;
    i2c_stop();
    Ok(())
}

pub fn i2c_read_register(device_addr: u8, reg_addr: u8) -> Result<u8, I2CError> {
    // Write phase
    i2c_start()?;
    i2c_send_address(device_addr, false)?;
    i2c_send_data(reg_addr)?;
    
    // Read phase
    i2c_start()?; // Repeated start
    i2c_send_address(device_addr, true)?;
    let data = i2c_receive_data(false)?; // NACK for single byte read
    i2c_stop();
    
    Ok(data)
}