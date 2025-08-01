/* display.h */
#ifndef __DISPLAY_H__
#define __DISPLAY_H__

#include <stdint.h>
#include <stddef.h>
#include "font.h"

// Enum for screen orientation
typedef enum {
    ORIENTATION_PORTRAIT = 0, // Renamed to avoid macro conflict
    ORIENTATION_LANDSCAPE = 1
} display_orientation;

// Abstract display driver structure
typedef struct {
    void (*init)(void);
    void (*write_char)(uint16_t x, uint16_t y, char ch, FontDef font, uint16_t color, uint16_t bgcolor);
    void (*write_string)(uint16_t x, uint16_t y, const char* str, FontDef font, uint16_t color, uint16_t bgcolor);
    void (*fill_screen)(uint16_t color);
    void (*draw_pixel)(uint16_t x, uint16_t y, uint16_t color);
    void (*draw_image)(uint16_t x, uint16_t w, uint16_t y, uint16_t h, const uint16_t* img_data);
    void (*set_orientation)(display_orientation orientation);
    void (*fill_rect)(uint16_t x, uint16_t w, uint16_t y, uint16_t h, uint16_t color);
} display_driver_t;

#include "ili9341.h"


// Moved after typedef
extern const display_driver_t ili9341_display_driver;

// Function prototypes
void display_register_driver(const display_driver_t *driver);
void display_init(void);
void display_write_char(uint16_t x, uint16_t y, char ch, FontDef font, uint16_t color, uint16_t bgcolor);
void display_write_string(uint16_t x, uint16_t y, const char *str, FontDef font, uint16_t color, uint16_t bgcolor);
void display_fill_screen(uint16_t color);
void display_draw_pixel(uint16_t x, uint16_t y, uint16_t color);
void display_draw_image(uint16_t x, uint16_t w, uint16_t y, uint16_t h, const uint16_t* img_data);
void display_set_orientation(display_orientation orientation);
void display_fill_rect(uint16_t x, uint16_t w, uint16_t y, uint16_t h, uint16_t color);

//Function to register the hardware-specific driver
void display_register_driver(const display_driver_t *driver);
void display_init(void);
void display_write_char(uint16_t x, uint16_t y, char ch, FontDef font, uint16_t color, uint16_t bgcolor);
void display_write_string(uint16_t x, uint16_t y, const char *str, FontDef font, uint16_t color, uint16_t bgcolor);
void display_fill_screen(uint16_t color);
void display_draw_pixel(uint16_t x, uint16_t y, uint16_t color);
void display_draw_image(uint16_t x, uint16_t w, uint16_t y, uint16_t h, const uint16_t* img_data);
void display_set_orientation(display_orientation orientation);
void display_fill_rect(uint16_t x, uint16_t w, uint16_t y, uint16_t h, uint16_t color);

//Function to register the hardware-specific driver
void display_register_driver(const display_driver_t *driver);



#endif // __DISPLAY_H__
