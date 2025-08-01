/*
 * ili9341.h
 *
 *  Created on: Jul 6, 2025
 *      Author: Shreyas Acharya, BHARATI SOFTWARE
 */

#ifndef INC_ILI9341_H_
#define INC_ILI9341_H_

#include "main.h"
#include <stdbool.h>
#include "font.h"
#include "display.h"

// Orientation modes
#define LANDSCAPE 0
#define PORTRAIT 1

#define VIOLET                  0x801F  ///< RGB: 148, 0, 211
#define INDIGO                  0x480F  ///< RGB: 75, 0, 130
#define BLUE                    0x001F  ///< RGB: 0, 0, 255
#define GREEN                   0x07E0  ///< RGB: 0, 255, 0
#define YELLOW                  0xFFE0  ///< RGB: 255, 255, 0
#define ORANGE                  0xFC00  ///< RGB: 255, 165, 0
#define RED                     0xF800  ///< RGB: 255, 0, 0
#define WHITE                   0xFFFF  ///< RGB: 255, 255, 255
#define BLACK                   0x0000  ///< RGB: 0, 0, 0
#define NAVY                    0x000F  ///< RGB: 0, 0, 128
#define DARKGREEN               0x03E0  ///< RGB: 0, 128, 0
#define DARKCYAN                0x03EF  ///< RGB: 0, 128, 128
#define MAROON                  0x7800  ///< RGB: 128, 0, 0
#define PURPLE                  0x780F  ///< RGB: 128, 0, 128
#define OLIVE                   0x7BE0  ///< RGB: 128, 128, 0
#define LIGHTGREY               0xC618  ///< RGB: 192, 192, 192
#define DARKGREY                0x7BEF  ///< RGB: 128, 128, 128
#define CYAN                    0x07FF  ///< RGB: 0, 255, 255
#define MAGENTA                 0xF81F  ///< RGB: 255, 0, 255
#define GREENYELLOW             0xAFE5  ///< RGB: 173, 255, 47
#define PINK                    0xF81F  ///< RGB: 255, 0, 255
#define GC9A01A_COLOR565(r, g, b) (((r & 0xF8) << 8) | ((g & 0xFC) << 3) | ((b & 0xF8) >> 3))

// Function prototypes
void bsp_lcd_init(void);
void ili9341_init(void); // Add declaration
void ili9341_write_char(uint16_t x, uint16_t y, char ch, FontDef font, uint16_t color, uint16_t bgcolor); // Add declaration
void ili9341_write_string(uint16_t x, uint16_t y, const char* str, FontDef font, uint16_t color, uint16_t bgcolor);
void ili9341_fill_rect(uint16_t x, uint16_t w, uint16_t y, uint16_t h, uint16_t color);
void ili9341_set_orientation(display_orientation orientation);
void ili9341_fill_screen(uint16_t color);
void ili9341_draw_pixel(uint16_t x, uint16_t y, uint16_t color); // Ensure this is included
void ili9341_draw_image(uint16_t x, uint16_t w, uint16_t y, uint16_t h, const uint16_t* data);
void ili9341_invert_colors(bool invert);
void ili9341_lcd_fill_image(const uint16_t* data);

#endif /* INC_ILI9341_H_ */
