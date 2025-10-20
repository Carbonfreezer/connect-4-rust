//! This module shows the game over part with the winning situation and a button to return
//! to the player selection screen.

use crate::board_logic::bit_board::GameResult;
use crate::render_system::graphics::{Color, GraphicsPainter};
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};

pub struct StateGameOver {
    end_result: GameResult,
    highlighted_stones: Vec<(usize, usize)>,
    exit_pressed: bool,
}

const CENTRAL_POSITION: [f32; 2] = GraphicsPainter::get_drawing_coordinates_above_column(3);
const LOWER_POSITION_OUTER: [f32; 2] = [
    CENTRAL_POSITION[0] - GraphicsPainter::CIRCLE_RADIUS,
    CENTRAL_POSITION[1] - GraphicsPainter::CIRCLE_RADIUS,
];

const UPPER_POSITION_OUTER: [f32; 2] = [
    CENTRAL_POSITION[0] + GraphicsPainter::CIRCLE_RADIUS,
    CENTRAL_POSITION[1] + GraphicsPainter::CIRCLE_RADIUS,
];

const LOWER_POSITION_INNER: [f32; 2] = [
    CENTRAL_POSITION[0] - GraphicsPainter::CIRCLE_RADIUS * 0.8,
    CENTRAL_POSITION[1] - GraphicsPainter::CIRCLE_RADIUS * 0.8,
];

const UPPER_POSITION_INNER: [f32; 2] = [
    CENTRAL_POSITION[0] + GraphicsPainter::CIRCLE_RADIUS * 0.8,
    CENTRAL_POSITION[1] + GraphicsPainter::CIRCLE_RADIUS * 0.8,
];

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
    /// On enter er extract the information of why the game is over and eventually highlighted stones.
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

    /// When the exit got triggered we leave and clear the board.
    fn update(&mut self, _: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.exit_pressed {
            black_board.game_board.reset();
            Some(GameStateIndex::StartSelection)
        } else {
            None
        }
    }

    /// Checks if mouse button got pressed and flags that we want to leave.
    fn mouse_click(&mut self, _: [f32; 2]) {
        self.exit_pressed = true;
    }

    /// Renders the board, eventually highlighted winning stones and the game end
    /// status icon.
    fn draw(&self, graphics: &GraphicsPainter, black_board: &Blackboard) {
        graphics.render_board(&black_board.game_board);

        // The button.
        graphics.draw_rectangle_normal(LOWER_POSITION_OUTER, UPPER_POSITION_OUTER, Color::DarkGrey);
        graphics.draw_rectangle_normal(LOWER_POSITION_INNER, UPPER_POSITION_INNER, Color::Grey);
        // Eventually the highlighted stones and button inset.
        if self.end_result == GameResult::FirstPlayerWon {
            graphics.render_winning_stones(true, &self.highlighted_stones);
            graphics.draw_circle_normal(
                GraphicsPainter::CIRCLE_RADIUS * 0.6,
                CENTRAL_POSITION,
                Color::LightYellow,
            );
        } else if self.end_result == GameResult::SecondPlayerWon {
            graphics.render_winning_stones(false, &self.highlighted_stones);
            graphics.draw_circle_normal(
                GraphicsPainter::CIRCLE_RADIUS * 0.6,
                CENTRAL_POSITION,
                Color::LightBlue,
            );
        }
    }
}
