//! Program is an adaption of the Four Connect game. It features an alpha-beta pruned negamax algorithm
//! with transposition tables and a thread based asynchronous user interface.
//!
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128.png")]
#![doc(html_favicon_url = "https://www.rust-lang.org/favicon.ico")]

mod state_system;

mod board_logic;
mod debug_macros;
mod render_system;

use macroquad::miniquad::window::set_window_size;
use state_system::*;

use crate::game_state::{Blackboard, GameStateIndex, generate_state_collection};
use crate::render_system::graphics::{WINDOW_DIMENSION, create_board_texture};
use macroquad::prelude::*;

#[macroquad::main("Connect four")]
async fn main() {
    set_window_size(WINDOW_DIMENSION as u32, WINDOW_DIMENSION as u32);

    let board_texture = create_board_texture();
    // Origin is in the lower left corner
    let camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, WINDOW_DIMENSION, WINDOW_DIMENSION));
    set_camera(&camera);

    let mut state_array = generate_state_collection();
    let mut current_index: usize = GameStateIndex::StartSelection as usize;
    let mut black_board: Blackboard = Blackboard::new(board_texture);

    loop {
        // First do the mouse clicks:
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let drawing_pos = camera.screen_to_world(Vec2::from(mouse_pos));
            state_array[current_index].mouse_click(drawing_pos);
        }

        // Update logic-
        let update_result = state_array[current_index].update(get_frame_time(), &mut black_board);
        if let Some(follow_index) = update_result {
            current_index = follow_index as usize;
            state_array[current_index].enter(&black_board);
        }

        // First we do the logic.
        clear_background(BLACK);
        // Render stuff.
        state_array[current_index].draw(&black_board);

        next_frame().await
    }
}
