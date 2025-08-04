/*
 * background.c
 *
 *  Created on: Apr 25, 2024
 *      Author: Shreyas Acharya, BHARATI SOFTWARE
 */

#include "background.h"
#include "obstacle.h"
#include "main.h"

/**
  * @brief  Set the background color on the LCD.
  * @retval None
  */
void bsp_lcd_set_background() {
  ili9341_fill_rect(0, 0, 320, 240, BACKGROUND);
  //ili9341_fill_rect(0,(BSP_LCD_BACKGROUND_WIDTH),0,(BSP_LCD_BACKGROUND_HEIGHT), BACKGROUND);
}

/**
  * @brief  Display bush images on the LCD.
  * @retval None
  */
// void display_bush() {
//   ili9341_draw_image(0, 60, 210, 30, (uint16_t *)bush);
//   ili9341_draw_image(60,60, 210, 30, (uint16_t *)bush);
//   ili9341_draw_image(120, 60, 210, 30, (uint16_t *)bush);
//   ili9341_draw_image(180, 60, 210, 30, (uint16_t *)bush);
//   ili9341_draw_image(240, 60, 210, 30, (uint16_t *)bush);
//   ili9341_draw_image(260, 60, 210, 30, (uint16_t *)bush);

// }

void display_bush() {
    ili9341_draw_image(0,    210, 30, 30, (uint16_t *)bush);   // X = 0
    ili9341_draw_image(30,   210, 30, 30, (uint16_t *)bush);   // X = 30
    ili9341_draw_image(60,   210, 30, 30, (uint16_t *)bush);   // X = 60
    ili9341_draw_image(90,   210, 30, 30, (uint16_t *)bush);   // X = 90
    ili9341_draw_image(120,  210, 30, 30, (uint16_t *)bush);   // X = 120
    ili9341_draw_image(150,  210, 30, 30, (uint16_t *)bush);   // X = 150
    ili9341_draw_image(180,  210, 30, 30, (uint16_t *)bush);   // X = 180
    ili9341_draw_image(210,  210, 30, 30, (uint16_t *)bush);   // X = 210
    ili9341_draw_image(240, 210, 30, 30, (uint16_t *)bush);
    ili9341_draw_image(270, 210, 30, 30, (uint16_t *)bush);
    ili9341_draw_image(300, 210, 30, 30, (uint16_t *)bush);
}



/**
  * @brief  Set the background and display bush images for the Flappy Bird game.
  * @retval None
  */
void flappy_game_set_back_ground() {
	bsp_lcd_set_background();
	score_card_background();
  display_bush();
}

/**
  * @brief  Fill the countdown trace area with the background color.
  * @retval None
  */
void fill_countdown_trace() {
  ili9341_fill_rect(0,107,240,26, BACKGROUND);
  ili9341_fill_rect(116,140,16,26, BACKGROUND);
}

/**
  * @brief  Fill the game over trace area with the background color.
  * @retval None
  */
void set_gameover_background() {
  ili9341_fill_rect(0, 0, 320, 240, BACKGROUND);
}

/**
  * @brief  Display the game title on the LCD.
  * @retval None
  */
void display_game_title() {
  ili9341_draw_image(120, 100, 80, 40, (uint16_t *)Flappy_font);
}

/**
  * @brief  Display the game title on the LCD.
  * @retval None
  */
void display_game_over() {
  score_card_background();
  ili9341_draw_image(80, 90, 160,60, (uint16_t *)game_over);
}
