//! In this state we are awaiting the computation result, that has been kicked off in the
//! player input state and perform the dropping stone animation. 

use crate::render_system::graphics::render_board;
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};
use macroquad::math::Vec2;

pub struct StateComputerExecution {
    animator: StoneAnimator,
    slot_picked: u32,
    computation_executed: bool,
}

impl StateComputerExecution {
    pub fn new() -> StateComputerExecution {
        StateComputerExecution {
            animator: StoneAnimator::new(),
            slot_picked: 0,
            computation_executed: false,
        }
    }
}

impl GameState for StateComputerExecution {
    /// Here we start the animation of the stone and feed the new situation to the worker
    /// thread to perform the computations.
    fn enter(&mut self, _: &Blackboard) {
        self.computation_executed = false;
    }

    /// In the update we perform the animation and once it is finished we check with the worker
    /// thread, if the results are present and if so leave the thread for execution.
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {

        // Do this one frame delayed to get smooth animations.
        if self.computation_executed && (!self.animator.is_animating()) {
            self.animator
                .start_animating(&black_board.game_board, self.slot_picked, true);
            
            return None;
        }


        if !self.computation_executed {
            let slot_choice = black_board.alpha_beta.get_best_move(black_board.game_board.clone());
            self.slot_picked = slot_choice;

            self.computation_executed = true;

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
