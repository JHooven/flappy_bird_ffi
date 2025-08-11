/*
 * text_mssg.h
 *
 *  Created on: May 2, 2024
 *      Author: Shreyas Acharya, BHARATI SOFTWARE
 */

#ifndef INC_TEXT_MSG_H_
#define INC_TEXT_MSG_H_

#include "main.h"
#include "ili9341.h"

typedef enum {
  NOT_PRESSED,
  PRESSED
} button_state_t;

void game_init_mssg();
void game_over_mssg();
void render_gamestart_countdown();
void player_show_score();
void display_final_score() ;
#endif /* INC_TEXT_MSG_H_ */
