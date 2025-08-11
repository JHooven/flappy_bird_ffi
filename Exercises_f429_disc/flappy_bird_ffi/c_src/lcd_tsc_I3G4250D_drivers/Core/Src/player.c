/*
 * bird.c
 *
 *  Created on: Apr 18, 2024
 *      Author: Shreyas Acharya, BHARATI SOFTWARE
 */

#include "player.h"
#include "background.h"
#include "main.h"
/**
  * @brief  Move the player on the LCD.
  * @param  y_pos: Starting position of the player.
  * @retval None
  */
void move_player( int y_pos) {

//	if(player.wing_movment%5==0){
		ili9341_draw_image( player.x, player.w, y_pos, player.h, (uint16_t *)image_data_bird);
//	} else {
//	  ili9341_draw_image( player.x, player.w, y_pos, player.h, (uint16_t *)image_data_bird2);
//	}
  fill_player_trace(y_pos);
  player.y = y_pos;

	player.wing_movment++;
}

/**
  * @brief  Fill the player trace area with the background color.
  * @param  y_pos: New Y position of the player.
  * @retval None
  */
void fill_player_trace(int y_pos) {
#ifdef OLD
  if (obstacle.x >= (player.x + player.w) || (obstacle.x + 30) <= player.x) {
    ili9341_fill_rect(player.x, player.w, 30, 180, BACKGROUND);
  } else if (obstacle.x <= (player.x + player.w) || (obstacle.x + 30) >= player.x) {
    ili9341_fill_rect(player.x, player.w, (obstacle.y + obstacle.h), obstacle.g, BACKGROUND);
  }
#else
  if (player.y >= PLAYER_ySTART_POINT && (player.y + player.h) <= PLAYER_yEND_POINT) {
    if (y_pos > player.y) {
      // Moving downward
      ili9341_fill_rect(player.x, player.w, player.y, (y_pos - player.y), BACKGROUND);
    } else if (y_pos < player.y) {
      // Moving upward
      ili9341_fill_rect(player.x, player.w, (y_pos + player.h), (player.y + player.h) - (y_pos + player.h), BACKGROUND);
    }
  }
#endif
}
