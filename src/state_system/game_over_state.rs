//! This module shows the game over part with the winning situation and a button to return
//! to the player selection screen.

use crate::board_logic::bit_board::GameResult;
use crate::render_system::graphics::{Color, GraphicsPainter};
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};

pub struct GameOverState {
    end_result: GameResult,
    highlighted_stones: Vec<(usize, usize)>,
    exit_pressed: bool,
    central_position: [f32; 2],
    lower_position: [f32; 2],
    upper_position: [f32; 2],
}

impl GameOverState {
    pub fn new() -> GameOverState {
        let central_position = GraphicsPainter::get_drawing_coordinates_above_column(3);
        let lower_position = [
            central_position[0] - GraphicsPainter::CIRCLE_RADIUS,
            central_position[1] - GraphicsPainter::CIRCLE_RADIUS,
        ];
        let upper_position = [
            central_position[0] + GraphicsPainter::CIRCLE_RADIUS,
            central_position[1] + GraphicsPainter::CIRCLE_RADIUS,
        ];
        
        GameOverState {
            end_result: GameResult::Pending,
            highlighted_stones: Vec::new(),
            exit_pressed: false,
            central_position,
            lower_position,
            upper_position,
        }
    }
}

impl GameState for GameOverState {
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

    /// When the exit button got pressed we leave and clear the board.
    fn update(&mut self, _: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.exit_pressed {
            black_board.game_board.reset();
            Some(GameStateIndex::StartSelection)
        } else {
            None
        }
    }

    /// Checks if the exit button got pressed.
    fn mouse_click(&mut self, position: [f32; 2]) {
        let diff_x = (position[0] - self.central_position[0]).abs();
        let diff_y = (position[1] - self.central_position[1]).abs();

        self.exit_pressed =
            (diff_x < GraphicsPainter::CIRCLE_RADIUS) && (diff_y < GraphicsPainter::CIRCLE_RADIUS);
    }

    /// Renders the board, eventually highlighted winning stones and the exit button.
    fn draw(&self, graphics: &GraphicsPainter, black_board: &Blackboard) {
        graphics.render_board(&black_board.game_board);

        // The button.
        graphics.draw_rectangle_normal(self.lower_position, self.upper_position, Color::Grey);
        // Eventually the highlighted stones and button inset.
        if self.end_result == GameResult::FirstPlayerWon {
            graphics.render_winning_stones(true, &self.highlighted_stones);
            graphics.draw_circle_normal(
                GraphicsPainter::CIRCLE_RADIUS * 0.6,
                self.central_position,
                Color::LightYellow,
            );
        } else if self.end_result == GameResult::SecondPlayerWon {
            graphics.render_winning_stones(false, &self.highlighted_stones);
            graphics.draw_circle_normal(
                GraphicsPainter::CIRCLE_RADIUS * 0.6,
                self.central_position,
                Color::LightBlue,
            );
        }
    }
}
