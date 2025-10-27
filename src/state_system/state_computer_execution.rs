//! In this state we are awaiting the computation result, that has been kicked off in the
//! player input state and perform the dropping stone animation. 

use crate::render_system::graphics::render_board;
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};
use macroquad::math::Vec2;

pub struct StateComputerExecution {
    animator: StoneAnimator,
    slot_picked: u32,
    result_received: bool,
}

impl StateComputerExecution {
    pub fn new() -> StateComputerExecution {
        StateComputerExecution {
            animator: StoneAnimator::new(),
            slot_picked: 0,
            result_received: false,
        }
    }
}

impl GameState for StateComputerExecution {
    /// Here we start the animation of the stone and feed the new situation to the worker
    /// thread to perform the computations.
    fn enter(&mut self, _: &Blackboard) {
        self.result_received = false;
    }

    /// In the update we perform the animation and once it is finished we check with the worker
    /// thread, if the results are present and if so leave the thread for execution.
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if !self.result_received {
            if let Some(slot_choice) = black_board.ai_system.try_get_computation_result() {
                self.slot_picked = slot_choice;
                self.animator
                    .start_animating(&black_board.game_board, slot_choice, true);
                self.result_received = true;
            }

            return None;
        }

        if self.animator.is_animating() {
            self.animator.update(delta_time);
            if !self.animator.is_animating() {
                black_board
                    .game_board
                    .apply_move_on_column(self.slot_picked, true);

                if black_board.game_board.is_game_over() {
                    return Some(GameStateIndex::GameOverState);
                } else {
                    return Some(GameStateIndex::PlayerInputState);
                }
            }
            return None;
        }

        None
    }

    /// We do not process mouse clicks here.
    fn mouse_click(&mut self, _: Vec2) {
        // Nothing to do here.
    }

    /// Draws the board and eventually the falling stone.
    fn draw(&self, black_board: &Blackboard) {
        if self.animator.is_animating() {
            self.animator.draw();
        }

        render_board(&black_board.game_board, &black_board.board_texture);
    }
}
