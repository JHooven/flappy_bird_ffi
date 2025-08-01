/*
 * ili9341.c
 *
 *  Created on: Jul 6, 2025
 *      Author: Shreyas Acharya, BHARATI SOFTWARE
 */


#include "main.h"
#include "ili9341.h"
#include <string.h>
#include "display.h"
#include "font.h"

const display_driver_t ili9341_display_driver = {
    .init = ili9341_init,
    .write_char = ili9341_write_char,
    .write_string = ili9341_write_string,
    .draw_image = ili9341_draw_image,
    .fill_screen = ili9341_fill_screen,
    .draw_pixel = ili9341_draw_pixel,
    .fill_rect = ili9341_fill_rect, // Fixed from fill_rectangle
    .set_orientation = ili9341_set_orientation,
};

// SPI handle (assumed to be initialized in main.c or via STM32CubeMX)
extern SPI_HandleTypeDef hspi5;

// Control pin macros (based on STM32F429 Discovery or custom setup)
#define ILI9341_CS_LOW    HAL_GPIO_WritePin(GPIOC, GPIO_PIN_2, GPIO_PIN_RESET)
#define ILI9341_CS_HIGH   HAL_GPIO_WritePin(GPIOC, GPIO_PIN_2, GPIO_PIN_SET)
#define ILI9341_DC_CMD    HAL_GPIO_WritePin(GPIOD, GPIO_PIN_13, GPIO_PIN_RESET)
#define ILI9341_DC_DATA   HAL_GPIO_WritePin(GPIOD, GPIO_PIN_13, GPIO_PIN_SET)
#define ILI9341_RST_LOW   HAL_GPIO_WritePin(GPIOG, GPIO_PIN_14, GPIO_PIN_RESET)
#define ILI9341_RST_HIGH  HAL_GPIO_WritePin(GPIOG, GPIO_PIN_14, GPIO_PIN_SET)
#define ILI9341_BL_ON     HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET)
#define ILI9341_BL_OFF    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET)

// ILI9341 commands (subset relevant to initialization and drawing)
#define ILI9341_SWRESET   0x01 // Software Reset
#define ILI9341_SLPOUT    0x11 // Sleep Out
#define ILI9341_DISPON    0x29 // Display On
#define ILI9341_CASET     0x2A // Column Address Set
#define ILI9341_RASET     0x2B // Row Address Set
#define ILI9341_RAMWR     0x2C // Memory Write
#define ILI9341_MADCTL    0x36 // Memory Access Control
#define ILI9341_COLMOD    0x3A // Pixel Format Set
#define ILI9341_INVON     0x21 // Display Inversion On
#define ILI9341_INVOFF    0x20 // Display Inversion Off
#define ILI9341_TEON      0x35 // Tearing Effect Line On

// Display dimensions
#define ILI9341_WIDTH     320
#define ILI9341_HEIGHT    240

// Memory Access Control parameters
#define MADCTL_MY  0x80 // Row Address Order
#define MADCTL_MX  0x40 // Column Address Order
#define MADCTL_MV  0x20 // Row/Column Exchange
#define MADCTL_ML  0x10 // Vertical Refresh Order
#define MADCTL_BGR 0x08 // RGB-BGR Order
#define MADCTL_MH  0x04 // Horizontal Refresh Order

/**
  * @brief  Sends a command to the ILI9341 LCD display.
  * @param  cmd: The command byte to be sent to the display.
  * @retval None
  */
static void ili9341_write_cmd(uint8_t cmd) {
    ILI9341_CS_LOW;
    ILI9341_DC_CMD;
    HAL_SPI_Transmit(&hspi5, &cmd, 1, HAL_MAX_DELAY);
    ILI9341_CS_HIGH;
}

/**
  * @brief  Sends a buffer of data to the ILI9341 LCD display.
  * @param  buff: Pointer to the buffer containing the data to be sent.
  * @param  buff_size: Size of the buffer.
  * @retval None
  */
static void ili9341_write_data(uint8_t* buff, uint32_t buff_size) {
    ILI9341_CS_LOW;
    ILI9341_DC_DATA;
    HAL_SPI_Transmit(&hspi5, buff, buff_size, HAL_MAX_DELAY);
    ILI9341_CS_HIGH;
}

/**
  * @brief  Hardware reset for ILI9341 LCD
  * @param  None
  * @retval None
  */
static void ili9341_hw_reset(void) {
    ILI9341_RST_HIGH;
    HAL_Delay(10);
    ILI9341_RST_LOW;
    HAL_Delay(10);
    ILI9341_RST_HIGH;
    HAL_Delay(120); // ILI9341 requires longer reset recovery time
}

/**
  * @brief  Set an area for drawing on the display with start row, end row, start col, end col.
  * @param  x0: Start column address.
  * @param  x1: End column address.
  * @param  y0: Start row address.
  * @param  y1: End row address.
  * @retval None
  */
void ili9341_set_address_window(uint16_t x0, uint16_t x1, uint16_t y0, uint16_t y1) {
    uint8_t data[4];

    // Set column address
    ili9341_write_cmd(ILI9341_CASET);
    data[0] = x0 >> 8;
    data[1] = x0 & 0xFF;
    data[2] = x1 >> 8;
    data[3] = x1 & 0xFF;
    ili9341_write_data(data, 4);

    // Set row address
    ili9341_write_cmd(ILI9341_RASET);
    data[0] = y0 >> 8;
    data[1] = y0 & 0xFF;
    data[2] = y1 >> 8;
    data[3] = y1 & 0xFF;
    ili9341_write_data(data, 4);

    // Memory write
    ili9341_write_cmd(ILI9341_RAMWR);
}

/**
  * @brief  Initialize the ILI9341 LCD
  * @param  None
  * @retval None
  */
void ili9341_init(void) {
    uint8_t params[15];

    // Software reset
    ili9341_write_cmd(ILI9341_SWRESET);
    HAL_Delay(150);

    // Exit sleep mode
    ili9341_write_cmd(ILI9341_SLPOUT);
    HAL_Delay(150);

    // Power control B (0xCF)
    ili9341_write_cmd(0xCF);
    params[0] = 0x00;
    params[1] = 0x81;
    params[2] = 0x30;
    ili9341_write_data(params, 3);

    // Power on sequence control (0xED)
    ili9341_write_cmd(0xED);
    params[0] = 0x64;
    params[1] = 0x03;
    params[2] = 0x12;
    params[3] = 0x81;
    ili9341_write_data(params, 4);

    // Driver timing control A (0xE8)
    ili9341_write_cmd(0xE8);
    params[0] = 0x85;
    params[1] = 0x10;
    params[2] = 0x78;
    ili9341_write_data(params, 3);

    // Power control A (0xCB)
    ili9341_write_cmd(0xCB);
    params[0] = 0x39;
    params[1] = 0x2C;
    params[2] = 0x00;
    params[3] = 0x34;
    params[4] = 0x02;
    ili9341_write_data(params, 5);

    // Pump ratio control (0xF7)
    ili9341_write_cmd(0xF7);
    params[0] = 0x20;
    ili9341_write_data(params, 1);

    // Driver timing control B (0xEA)
    ili9341_write_cmd(0xEA);
    params[0] = 0x00;
    params[1] = 0x00;
    ili9341_write_data(params, 2);

    // Power control 1 (0xC0)
    ili9341_write_cmd(0xC0);
    params[0] = 0x21;
    ili9341_write_data(params, 1);

    // Power control 2 (0xC1)
    ili9341_write_cmd(0xC1);
    params[0] = 0x11;
    ili9341_write_data(params, 1);

    // VCOM control 1 (0xC5)
    ili9341_write_cmd(0xC5);
    params[0] = 0x31;
    params[1] = 0x3C;
    ili9341_write_data(params, 2);

    // VCOM control 2 (0xC7)
    ili9341_write_cmd(0xC7);
    params[0] = 0xC0;
    ili9341_write_data(params, 1);

    // Pixel format set (RGB565)
    ili9341_write_cmd(ILI9341_COLMOD);
    params[0] = 0x55; // 16-bit/pixel (RGB565)
    ili9341_write_data(params, 1);

    // Frame rate control (0xB1)
    ili9341_write_cmd(0xB1);
    params[0] = 0x00;
    params[1] = 0x1B;
    ili9341_write_data(params, 2);

    // Gamma settings (example values, may need tuning)
    ili9341_write_cmd(0xF2);
    params[0] = 0x48;
    ili9341_write_data(params, 1);

    ili9341_write_cmd(0x26);
    params[0] = 0x01;
    ili9341_write_data(params, 1);

    // Positive gamma correction (0xE0)
    ili9341_write_cmd(0xE0);
    params[0] = 0x0F;
    params[1] = 0x31;
    params[2] = 0x2B;
    params[3] = 0x0C;
    params[4] = 0x0E;
    params[5] = 0x08;
    params[6] = 0x4E;
    params[7] = 0xF1;
    params[8] = 0x37;
    params[9] = 0x07;
    params[10] = 0x10;
    params[11] = 0x03;
    params[12] = 0x0E;
    params[13] = 0x09;
    params[14] = 0x00;
    ili9341_write_data(params, 15);

    // Negative gamma correction (0xE1)
    ili9341_write_cmd(0xE1);
    params[0] = 0x00;
    params[1] = 0x0E;
    params[2] = 0x14;
    params[3] = 0x03;
    params[4] = 0x11;
    params[5] = 0x07;
    params[6] = 0x31;
    params[7] = 0xC1;
    params[8] = 0x48;
    params[9] = 0x08;
    params[10] = 0x0F;
    params[11] = 0x0C;
    params[12] = 0x31;
    params[13] = 0x36;
    params[14] = 0x0F;
    ili9341_write_data(params, 15);


//    // Memory Access Control
    ili9341_write_cmd(0x36); // ILI9341_MADCTL
    params[0] = 0x28; // Example: MX=0, MY=1, MV=0, ML=0, BGR=1, MH=0
    ili9341_write_data(params, 1);


    // Tearing effect line on
    ili9341_write_cmd(ILI9341_TEON);
    params[0] = 0x00;
    ili9341_write_data(params, 1);

    // Display on
    ili9341_write_cmd(ILI9341_DISPON);
    HAL_Delay(150);
}

/**
  * @brief  Initialize the ILI9341 LCD
  * @param  None
  * @retval None
  */
void bsp_lcd_init(void) {
    ILI9341_CS_HIGH;
    ILI9341_BL_ON;
    ILI9341_RST_HIGH;

    ili9341_hw_reset();
    ili9341_init();
    ili9341_set_orientation(LANDSCAPE);
}

/**
  * @brief  Draw a single pixel on the ILI9341 LCD
  * @param  x: X-coordinate of the pixel
  * @param  y: Y-coordinate of the pixel
  * @param  color: Color of the pixel in RGB565 format
  * @retval None
  */
void ili9341_draw_pixel(uint16_t x, uint16_t y, uint16_t color) {
    if ((x >= ILI9341_WIDTH) || (y >= ILI9341_HEIGHT)) return;

    ili9341_set_address_window(x, x, y, y);
    uint8_t data[] = { color >> 8, color & 0xFF };
    ili9341_write_data(data, sizeof(data));
}

/**
  * @brief  Write a character on the ILI9341 LCD
  * @param  x: Start column address
  * @param  y: Start row address
  * @param  ch: Character to be displayed
  * @param  font: Font definition structure
  * @param  color: Color of the character (RGB565 format)
  * @param  bgcolor: Background color of the character (RGB565 format)
  * @retval None
  */
void ili9341_write_char(uint16_t x, uint16_t y, char ch, FontDef font, uint16_t color, uint16_t bgcolor) {
    uint32_t i, b, j;

    ili9341_set_address_window(x, x + font.width - 1, y, y + font.height - 1);

    for (i = 0; i < font.height; i++) {
        b = font.data[(ch - 32) * font.height + i];
        for (j = 0; j < font.width; j++) {
            uint8_t data[] = { (b << j) & 0x8000 ? (color >> 8) : (bgcolor >> 8),
                               (b << j) & 0x8000 ? (color & 0xFF) : (bgcolor & 0xFF) };
            ili9341_write_data(data, sizeof(data));
        }
    }
}

/**
  * @brief  Write a string on the ILI9341 LCD
  * @param  x: Start column address
  * @param  y: Start row address
  * @param  str: Pointer to the string to be displayed
  * @param  font: Font definition structure
  * @param  color: Text color (RGB565 format)
  * @param  bgcolor: Background color (RGB565 format)
  * @retval None
  */
void ili9341_write_string(uint16_t x, uint16_t y, const char* str, FontDef font, uint16_t color, uint16_t bgcolor) {
    while (*str) {
        if (x + font.width >= ILI9341_WIDTH) {
            x = 0;
            y += font.height;
            if (y + font.height >= ILI9341_HEIGHT) {
                break;
            }
            if (*str == ' ') {
                str++;
                continue;
            }
        }
        ili9341_write_char(x, y, *str, font, color, bgcolor);
        x += font.width;
        str++;
    }
}

/**
  * @brief  Fills a rectangle on the ILI9341 LCD display with a specified color.
  * @param  x: X-coordinate of the top-left corner of the rectangle.
  * @param  w: Width of the rectangle.
  * @param  y: Y-coordinate of the top-left corner of the rectangle.
  * @param  h: Height of the rectangle.
  * @param  color: Color to fill the rectangle with.
  * @retval None
  */
void ili9341_fill_rect(uint16_t x, uint16_t w, uint16_t y, uint16_t h, uint16_t color) {
    if ((x >= ILI9341_WIDTH) || (y >= ILI9341_HEIGHT)) return;
    if ((x + w - 1) >= ILI9341_WIDTH) w = ILI9341_WIDTH - x;
    if ((y + h - 1) >= ILI9341_HEIGHT) h = ILI9341_HEIGHT - y;

    ili9341_set_address_window(x, x + w - 1, y, y + h - 1);
    uint8_t data[] = { color >> 8, color & 0xFF };
    ILI9341_CS_LOW;
    ILI9341_DC_DATA;
    for (uint32_t i = 0; i < w * h; i++) {
        HAL_SPI_Transmit(&hspi5, data, sizeof(data), HAL_MAX_DELAY);
    }
    ILI9341_CS_HIGH;
}

/**
  * @brief  Set the orientation of the ILI9341 LCD display
  * @param  orientation: Orientation mode (LANDSCAPE or PORTRAIT)
  * @retval None
  */
void ili9341_set_orientation(display_orientation orientation) {
    uint8_t params[1];

    if (orientation == ORIENTATION_LANDSCAPE) {
        params[0] = MADCTL_MV | MADCTL_MY | MADCTL_BGR; // 0xA8
    } else { // PORTRAIT
        params[0] = MADCTL_MX | MADCTL_BGR;
    }

    ili9341_write_cmd(ILI9341_MADCTL);
    ili9341_write_data(params, 1);
}

/**
  * @brief  Fill the entire background of the ILI9341 LCD with a specified color
  * @param  color: Color to fill the background (RGB565 format)
  * @retval None
  */
void ili9341_fill_screen(uint16_t color) {
    ili9341_fill_rect(0, ILI9341_WIDTH, 0, ILI9341_HEIGHT, color);
}

/**
  * @brief  Draw an image on the ILI9341 LCD
  * @param  x: Start column address
  * @param  w: Width of the image
  * @param  y: Start row address
  * @param  h: Height of the image
  * @param  data: Pointer to the image data (RGB565 format)
  * @retval None
  */
void ili9341_draw_image(uint16_t x, uint16_t w, uint16_t y, uint16_t h, const uint16_t* data) {
    if ((x >= ILI9341_WIDTH) || (y >= ILI9341_HEIGHT)) return;
    if ((x + w - 1) >= ILI9341_WIDTH) w = ILI9341_WIDTH - x;
    if ((y + h - 1) >= ILI9341_HEIGHT) h = ILI9341_HEIGHT - y;

    ili9341_set_address_window(x, x + w - 1, y, y + h - 1);
    ILI9341_CS_LOW;
    ILI9341_DC_DATA;
    for (uint32_t i = 0; i < w * h; i++) {
        uint8_t color[2] = { data[i] >> 8, data[i] & 0xFF };
        HAL_SPI_Transmit(&hspi5, color, 2, HAL_MAX_DELAY);
    }
    ILI9341_CS_HIGH;
}

/**
  * @brief  Inverts the colors on the ILI9341 LCD display.
  * @param  invert: Boolean flag to determine whether to invert colors.
  * @retval None
  */
void ili9341_invert_colors(bool invert) {
    ili9341_write_cmd(invert ? ILI9341_INVON : ILI9341_INVOFF);
}

/**
  * @brief  Draw a full-scale image on the ILI9341 LCD (320x240)
  * @param  data: Pointer to the image data (RGB565 format)
  * @retval None
  */
void ili9341_lcd_fill_image(const uint16_t* data) {
    ili9341_draw_image(0, ILI9341_WIDTH, 0, ILI9341_HEIGHT, data);
}
