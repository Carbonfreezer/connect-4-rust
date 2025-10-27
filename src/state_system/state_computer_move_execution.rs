//! This state simply executes a prestored decision and shows the animation.

use macroquad::math::Vec2;
use crate::render_system::graphics::{render_board};
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};

/// We remember the chosen slot to make the move permanent at the end of the animation
/// and have an animator to show it.
pub struct StateComputerMoveExecution {
    slot_picked: u32,
    animator: StoneAnimator,
}

impl StateComputerMoveExecution {
    pub(crate) fn new() -> StateComputerMoveExecution {
        StateComputerMoveExecution {
            slot_picked: 0,
            animator: StoneAnimator::new(),
        }
    }
}

impl GameState for StateComputerMoveExecution {
    /// We read out the move we want to make and initialize the animation.
    fn enter(&mut self, black_board: &Blackboard) {
        self.slot_picked = black_board.computer_choice;
        self.animator
            .start_animating(&black_board.game_board, self.slot_picked, true);
    }

    /// We wait for the end of the animation, See if game is over and transfer to the next states accordingly
    /// (end of game or players choice).
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.animator.is_animating() {
            self.animator.update(delta_time);
            return None;
        }

        black_board
            .game_board
            .apply_move_on_column(self.slot_picked, true);
        if black_board.game_board.is_game_over() {
            Some(GameStateIndex::GameOverState)
        } else {
            Some(GameStateIndex::PlayerInputState)
        }
    }

    /// No mouse input is processed in this state.
    fn mouse_click(&mut self, _: Vec2) {
        // No input required.
    }

    /// Draw the board with the falling stone.
    fn draw(&self,  black_board: &Blackboard) {
        self.animator.draw();
        render_board(&black_board.game_board, &black_board.board_texture);
    }
}
