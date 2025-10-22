//! Program is an adaption of the 4 Connect game. It is an attempt to brute force the game to achieve perfect strategy.


mod state_system;

mod board_logic;
mod debug_macros;
mod render_system;

use state_system::*;

use crate::game_state::{Blackboard, GameStateIndex, generate_state_collection};
use crate::render_system::graphics::GraphicsPainter;
use glume::gl;
use glume::window::{Event, MouseButton};
use std::time::Duration;

fn main() {
    let window_config = glume::window::WindowConfiguration {
        title: "4 Connect".to_string(),
        size: (800, 800),
        gl_version: (3, 3),
    };

    let window = window_config.build_window();

    unsafe {
        gl::Enable(gl::STENCIL_TEST);
    }

    let mut screen_extension = [0.0, 0.0];
    let mut adjusted_cursor_pos = [0.0, 0.0];

    let mut state_array = generate_state_collection();
    let mut current_index: usize = GameStateIndex::StartSelection as usize;
    let mut black_board: Blackboard = Blackboard::new();
    state_array[current_index].enter(&black_board);
    let graphics = GraphicsPainter::new();

    // Requested time between two frames.
    const DELTA_TIME: f32 = 0.02;

    let mut init = false;
    window.run(move |wc, event| {
        if !init {
            wc.set_tick_duration(Duration::from_secs_f32(DELTA_TIME));
            init = true;
        }
        match event {
            Event::Resized(width, height) => {
                screen_extension = [width as f32, height as f32];
                unsafe {
                    gl::Viewport(0, 0, width as i32, height as i32);
                }
            }

            Event::RedrawRequested => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                };
                state_array[current_index].draw(&graphics, &black_board);
            }

            Event::CursorMoved(x, y) => {
                adjusted_cursor_pos = [
                    2.0_f32 * x / screen_extension[0] - 1.0_f32,
                    1.0 - 2.0_f32 * y / screen_extension[1],
                ];
            }

            Event::Tick(tick) => {
                let delta_time = tick.ticks_passed as f32 * DELTA_TIME;
                let update_result = state_array[current_index].update(delta_time, &mut black_board);
                if let Some(follow_index) = update_result {
                    current_index = follow_index as usize;
                    state_array[current_index].enter(&black_board);
                }
                wc.request_redraw();
            }

            Event::MouseButtonPressed(button) => {
                wc.request_redraw();

                if button == MouseButton::Left {
                    state_array[current_index].mouse_click(adjusted_cursor_pos);
                }
            }

            _ => {}
        }
        Ok(())
    })
}
