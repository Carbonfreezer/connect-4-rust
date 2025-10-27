//! Contains the state to administrate the start screen, where the player selects, who will start
//! the game. When the computer starts the first calculation is kicked off.

use crate::game_state::{Blackboard, GameState, GameStateIndex};
use crate::render_system::graphics::{SymbolColor, get_color, print_text};
use macroquad::prelude::*;

pub struct StatePlayerStartSelection {
    position_selected: u8,
    time_passed_after_selection: f32,
    selection_happened: bool,
}

impl StatePlayerStartSelection {
    pub fn new() -> StatePlayerStartSelection {
        StatePlayerStartSelection {
            position_selected: 0,
            time_passed_after_selection: 0.0,
            selection_happened: false,
        }
    }
}

/// The position where the left element should be drawn-
const LEFT_CENTER: Vec2 = Vec2 { x: 175.0, y: 350.0 };
/// The position where the right element should be drawn.
const RIGHT_CENTER: Vec2 = Vec2 { x: 525.0, y: 350.0 };
/// The radius of the button.
const RADIUS: f32 = 100.0;
/// The highlight time for the button.
const HIGHLIGHT_TIME: f32 = 0.25;

impl GameState for StatePlayerStartSelection {
    fn enter(&mut self, _: &Blackboard) {
        self.selection_happened = false;
        self.time_passed_after_selection = 0.0;
    }

    /// The update waits for the input signal, updates the information on the game board and
    /// waits a short time for the highlighted button.
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.selection_happened {
            self.time_passed_after_selection += delta_time;
        }

        if self.time_passed_after_selection >= HIGHLIGHT_TIME {
            black_board
                .game_board
                .set_computer_first(self.position_selected == 1);
            if self.position_selected == 1 {
                black_board
                    .ai_system
                    .send_analysis_request(black_board.game_board.clone());
                return Some(GameStateIndex::ComputerExecutionState);
            } else {
                return Some(GameStateIndex::PlayerInputState);
            }
        }

        None
    }

    /// Mouse click detects the potential onto one of the buttons and eventually sets
    /// the information in the state.
    fn mouse_click(&mut self, position: Vec2) {
        if self.selection_happened {
            return;
        }

        if LEFT_CENTER.distance(position) < RADIUS {
            self.selection_happened = true;
            self.position_selected = 0;
        }

        if RIGHT_CENTER.distance(position) < RADIUS {
            self.selection_happened = true;
            self.position_selected = 1;
        }
    }

    /// Simply renders the two buttons, eventually highlighted when just selected.
    fn draw(&self, _: &Blackboard) {
        print_text("Welcome to Connect Four", Vec2::new(100.0, 575.0));
        if self.selection_happened && (self.position_selected == 0) {
            draw_poly(
                LEFT_CENTER.x,
                LEFT_CENTER.y,
                200,
                RADIUS,
                0.0,
                *get_color(SymbolColor::LightYellow),
            );
        } else {
            draw_poly(
                LEFT_CENTER.x,
                LEFT_CENTER.y,
                200,
                RADIUS,
                0.0,
                *get_color(SymbolColor::Yellow),
            );
        }

        print_text(
            "I start",
            LEFT_CENTER
                - Vec2 {
                    x: RADIUS,
                    y: 1.6 * RADIUS,
                },
        );

        if self.selection_happened && (self.position_selected == 1) {
            draw_poly(
                RIGHT_CENTER.x,
                RIGHT_CENTER.y,
                200,
                RADIUS,
                0.0,
                *get_color(SymbolColor::LightBlue),
            );
        } else {
            draw_poly(
                RIGHT_CENTER.x,
                RIGHT_CENTER.y,
                200,
                RADIUS,
                0.0,
                *get_color(SymbolColor::Blue),
            );
        }

        print_text(
            "You start",
            RIGHT_CENTER
                - Vec2 {
                    x: RADIUS,
                    y: 1.6 * RADIUS,
                },
        );
    }
}
