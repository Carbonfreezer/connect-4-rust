//! This module contains everything to drawing boards, stones and simple ui elements.

use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::bit_board_coding::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::debug_check_board_coordinates;
use macroquad::prelude::*;

/// The window dimension that will be used for rendering.
pub const WINDOW_DIMENSION: f32 = 700.0;

/// The radius with which we want to draw the stones in the below function.
pub const CIRCLE_RADIUS: f32 = WINDOW_DIMENSION / BOARD_WIDTH as f32 * 0.8 * 0.5;

/// Represents color types we can draw elements with.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SymbolColor {
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
}


/// Static array with colors that can be queried.
const COLOR_ARRAY: [Color; 4] = [
    Color::new(0.75, 0.55, 0.06, 1.0),
    Color::new(1.0, 0.91, 0.0, 1.0),
    Color::new(0.0, 0.28, 0.67, 1.0),
    Color::new(0.0, 0.58, 1.0, 1.0),
];

/// Generates an RGB value for any of the Colors indicated.
pub fn get_color(color: SymbolColor) -> &'static Color {
    match color {
        SymbolColor::Yellow => &COLOR_ARRAY[0],
        SymbolColor::LightYellow => &COLOR_ARRAY[1],
        SymbolColor::Blue => &COLOR_ARRAY[2],
        SymbolColor::LightBlue => &COLOR_ARRAY[3],
    }
}

/// Returns the drawing coordinates for an indicated stone position.
pub const fn get_drawing_coordinates(x_stone: u32, y_stone: u32) -> Vec2 {
    Vec2 {
        x: (x_stone as f32 + 0.5) * WINDOW_DIMENSION / BOARD_WIDTH as f32,
        y: (y_stone as f32 + 0.5) * WINDOW_DIMENSION / BOARD_WIDTH as f32,
    }
}

/// Gets a painting position above the column.
pub const fn get_drawing_coordinates_above_column(column: u32) -> Vec2 {
    // One column above the maximum.
    get_drawing_coordinates(column, 7)
}

/// Renders the board as is with all the stones in there.
pub fn render_board(board: &BitBoard, board_texture: &Texture2D) {
    draw_texture(board_texture, 0.0, 0.0, WHITE);

    for (x, y, first) in board.get_board_positioning() {
        debug_check_board_coordinates!(x, y);
        let color = if first {
            get_color(SymbolColor::Yellow)
        } else {
            get_color(SymbolColor::Blue)
        };
        let draw_pos = get_drawing_coordinates(x, y);
        draw_circle(draw_pos.x, draw_pos.y, CIRCLE_RADIUS, *color);
    }
}

/// Renders the indicated stones into the stone array with highlighted color. Indicates
/// if this is the first player who is winning to pick the right color.
pub fn render_winning_stones(is_first_player_winning: bool, list_of_positions: &Vec<(u32, u32)>) {
    let color = get_color(if is_first_player_winning {
        SymbolColor::LightYellow
    } else {
        SymbolColor::LightBlue
    });

    for (column, row) in list_of_positions {
        let draw_pos = get_drawing_coordinates(*column, *row);
        draw_circle(draw_pos.x, draw_pos.y, CIRCLE_RADIUS, *color);
    }
}

/// Draws the stone at the indicated coordinates, this is meant for drawing an animated stone.
pub fn draw_stone_at_coordinates(position: Vec2, is_first_player: bool) {
    let color = get_color(if is_first_player {
        SymbolColor::Yellow
    } else {
        SymbolColor::Blue
    });

    draw_circle(position.x, position.y, CIRCLE_RADIUS, *color);
}


/// A standardized way on how to write text in the game.
pub fn print_text(text: &str, position: Vec2) {
    draw_text_ex(
        text,
        position.x,
        position.y,
        TextParams {
            font: None,
            font_size: 50,
            font_scale: -1.0,
            font_scale_aspect: -1.0,
            rotation: 0.0,
            color: WHITE,
        },
    );
}
