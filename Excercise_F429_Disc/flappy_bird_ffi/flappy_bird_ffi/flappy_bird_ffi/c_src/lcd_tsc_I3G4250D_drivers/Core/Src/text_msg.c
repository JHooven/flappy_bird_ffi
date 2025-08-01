/*
 * text_mssg.c
 *
 *  Created on: May 2, 2024
 *      Author: Shreyas Acharya, BHARATI SOFTWARE
 */

#include "background.h"
#include <stdio.h>
#include <string.h>
#include <text_msg.h>
#include "font.h"

#define BTN_TXT           0xFFFB
#define BUTTON            0x0B85
#define START_BTN_WIDTH   100
#define START_BTN_HEIGHT  40
/**
  * @brief Initializes the game message display.
  * @param None
  * @retval None
  */
void game_init_mssg() {
  const char* game_start_btn = "Game starts in";
  display_game_title();
ili9341_write_string(50, 120, "Hello", Font_16x26, WHITE, BLACK);
  //ili9341_write_string(50, 120, (const char*)game_start_btn, Font_16x26, RED, BACKGROUND);
}

/**
  * @brief Displays game over message on the screen.
  * @param None
  * @retval None
  */
void game_over_mssg() {
	const char* game_over_message = "GAME OVER";
	ili9341_write_string(48, 70, (const char*)game_over_message, Font_16x26, RED, BACKGROUND);
}

/**
  * @brief Renders the countdown before the game starts.
  * @param None
  * @retval None
  */
void render_gamestart_countdown() {
	static uint32_t prestart_time = 0;
	countdown_complete = false;
	prestart_time++;
	if (prestart_time < 15) {
		ili9341_write_string(112, 156, (const char*)"3", Font_16x26, BLACK, BACKGROUND);
	} else if(prestart_time > 15 && prestart_time < 30) {
		ili9341_write_string(112, 156, (const char*)"2", Font_16x26, BLACK, BACKGROUND);
	} else if(prestart_time > 30 && prestart_time < 45) {
		ili9341_write_string(112, 156, (const char*)"1", Font_16x26, BLACK, BACKGROUND);
	} else if(prestart_time > 45) {
		prestart_time = 0;
		fill_countdown_trace();
		countdown_complete = true;
	}
}

/**
  * @brief Displays the player's score on the screen.
  * @param player: Pointer to the Player struct containing player information.
  * @retval None
  */
void player_show_score() {
  char score_str[4];
  if (player.score < 1000) {
    snprintf(score_str, sizeof(score_str), "%03d", player.score);
  } else {
    strcpy(score_str, "WIN");
  }
  ili9341_write_string(130, 0, score_str, Font_16x26, BLACK, SCORE_BOARD);
}

/**
  * @brief Displays game over message on the screen.
  * @param None
  * @retval None
  */
void display_final_score() {
  char score_str[4];
  const char* game_over_message = "Your score is:";
  ili9341_write_string(50, 130, (const char*)game_over_message, Font_16x26, RED, BACKGROUND);
  if (player.score < 1000) {
    snprintf(score_str, sizeof(score_str), "%03d", player.score);
  } else {
    strcpy(score_str, "WIN");
  }
  ili9341_write_string(96, 156, score_str, Font_16x26, BLACK, BACKGROUND);
}
