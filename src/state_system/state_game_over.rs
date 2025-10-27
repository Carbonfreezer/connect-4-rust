//! This module shows the game over part with the winning situation and an additional text.
//! On mouse interaction we transfer to the player selection screen.

use crate::board_logic::bit_board::GameResult;
use crate::render_system::graphics::{print_text, render_board, render_winning_stones};
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};
use macroquad::math::Vec2;

pub struct StateGameOver {
    end_result: GameResult,
    highlighted_stones: Vec<(u32, u32)>,
    exit_pressed: bool,
}

const TEXT_POSITION: Vec2 = Vec2 { x: 200.0, y: 640.0 };

impl StateGameOver {
    pub fn new() -> StateGameOver {
        StateGameOver {
            end_result: GameResult::Pending,
            highlighted_stones: Vec::new(),
            exit_pressed: false,
        }
    }
}

impl GameState for StateGameOver {
    /// On enter we extract the information of why the game is over and eventually highlighted stones.
    fn enter(&mut self, black_board: &Blackboard) {
        let (state, list) = black_board.game_board.get_winning_status_for_rendering();
        assert_ne!(
            state,
            GameResult::Pending,
            "The game should have been ended now"
        );
        self.end_result = state;
        self.highlighted_stones = list.unwrap_or(Vec::new());
        self.exit_pressed = false;
    }

    /// When the exit got triggered we leave and clear the board and go for start selection.
    fn update(&mut self, _: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.exit_pressed {
            black_board.game_board.reset();
            Some(GameStateIndex::StartSelection)
        } else {
            None
        }
    }

    /// Checks if mouse button got pressed and flags that we want to leave.
    fn mouse_click(&mut self, _: Vec2) {
        self.exit_pressed = true;
    }

    /// Renders the board, eventually highlighted winning stones and the game end
    /// status icon.
    fn draw(&self, black_board: &Blackboard) {
        render_board(&black_board.game_board, &black_board.board_texture);

        // The indicator.
        match self.end_result {
            GameResult::Pending => {
                panic!("Should not be the case")
            }
            GameResult::FirstPlayerWon => {
                print_text("Yellow has won", TEXT_POSITION);
                render_winning_stones(true, &self.highlighted_stones);
            }
            GameResult::SecondPlayerWon => {
                print_text("Blue has won", TEXT_POSITION);
                render_winning_stones(false, &self.highlighted_stones);
            }
            GameResult::Draw => print_text("Draw", TEXT_POSITION),
        }
    }
}
