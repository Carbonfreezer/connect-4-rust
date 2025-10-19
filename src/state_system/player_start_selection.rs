//! Contains the state to administrate the start screen, where the player selects, who will start
//! the game.

use crate::game_state::{Blackboard, GameState, GameStateIndex};
use crate::render_system::graphics::{Color, GraphicsPainter};

pub struct PlayerStartSelection {
    position_selected: u8,
    time_passed_after_selection: f32,
    selection_happened: bool,
}

impl PlayerStartSelection {
    pub fn new() -> PlayerStartSelection {
        PlayerStartSelection {
            position_selected: 0,
            time_passed_after_selection: 0.0,
            selection_happened: false,
        }
    }
}

/// The position where the left element should be drawn-
const LEFT_CENTER: [f32; 2] = [-0.5, 0.0];
/// The position where the right element should be drawn.
const RIGHT_CENTER: [f32; 2] = [0.5, 0.0];
/// The radius of the button.
const RADIUS: f32 = 0.3;
/// The highlight time for the button.
const HIGHLIGHT_TIME: f32 = 0.3;

/// Computes the euclidean distance between points.
fn get_distance_between(first_point: [f32; 2], second_point: [f32; 2]) -> f32 {
    let x = second_point[0] - first_point[0];
    let y = second_point[1] - first_point[1];

    (x * x + y * y).sqrt()
}
impl GameState for PlayerStartSelection {
    fn enter(&mut self, _: &Blackboard) {
        self.selection_happened = false;
        self.time_passed_after_selection = 0.0;
    }

    fn update(&mut self, delta_time: f32, board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.selection_happened {
            self.time_passed_after_selection += delta_time;
        }

        if self.time_passed_after_selection >= HIGHLIGHT_TIME {
            board
                .game_board
                .set_computer_first(self.position_selected == 1);
            return Some(GameStateIndex::Start);
        }

        None
    }

    fn mouse_click(&mut self, position: [f32; 2]) {
        if self.selection_happened {
            return;
        }

        if get_distance_between(LEFT_CENTER, position) < RADIUS {
            self.selection_happened = true;
            self.position_selected = 0;
        }

        if get_distance_between(RIGHT_CENTER, position) < RADIUS {
            self.selection_happened = true;
            self.position_selected = 1;
        }
    }

    fn draw(&self, graphics: &GraphicsPainter, _: &Blackboard) {
        if self.selection_happened && (self.position_selected == 0) {
            graphics.draw_circle_normal(RADIUS, LEFT_CENTER, Color::LightYellow);
        } else {
            graphics.draw_circle_normal(RADIUS, LEFT_CENTER, Color::Yellow);
        }

        if self.selection_happened && (self.position_selected == 1) {
            graphics.draw_circle_normal(RADIUS, RIGHT_CENTER, Color::LightBlue);
        } else {
            graphics.draw_circle_normal(RADIUS, RIGHT_CENTER, Color::Blue);
        }
    }
}
