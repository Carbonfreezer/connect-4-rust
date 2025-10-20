//! The player input state administrates the players choice, checks for feasibility and it it would
//! result in an game over it also executes the falling stone animation.

use crate::board_logic::bit_board_coding::BOARD_WIDTH;
use crate::render_system::graphics::GraphicsPainter;
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};

pub struct StatePlayerInput {
    slot_picked: Option<usize>,
    animator: StoneAnimator,
    transition_to_game_over: bool,
}

impl StatePlayerInput {
    pub fn new() -> StatePlayerInput {
        StatePlayerInput {
            slot_picked: None,
            animator: StoneAnimator::new(),
            transition_to_game_over: false,
        }
    }
}

impl GameState for StatePlayerInput {
    fn enter(&mut self, _: &Blackboard) {
        self.slot_picked = None;
        self.transition_to_game_over = false;
    }

    /// We handle the stone animation and if not and the player has chosen a slot, we decide
    /// depending on whether it s game over or not to transition to the computer choice state
    /// or start the animation to follow up on game over.
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.transition_to_game_over {
            self.animator.update(delta_time);
            if !self.animator.is_animating() {
                black_board
                    .game_board
                    .apply_move_on_column(black_board.player_choice, false);
                return Some(GameStateIndex::GameOverState);
            }
        }

        if let Some(slot_choice) = self.slot_picked {
            // We have chosen a slot.
            self.slot_picked = None;

            let coded_move = black_board.game_board.get_possible_move(slot_choice);
            // Illegal move.
            if coded_move == 0 {
                return None;
            }
            black_board.player_choice = slot_choice;
            // Now we check, if this would result in a game over or not.
            black_board.game_board.apply_move(coded_move, false);
            let game_over = black_board.game_board.is_game_over();
            black_board.game_board.revoke_move(coded_move, false);
            // When it is not game over we can directly go to the computer execution.
            if !game_over {
                return Some(GameStateIndex::ComputerCalculationState);
            }
            self.transition_to_game_over = true;
            self.animator
                .start_animating(&black_board.game_board, slot_choice, false);
        }
        None
    }

    /// Picks the slot, that was chosen by the player.
    fn mouse_click(&mut self, position: [f32; 2]) {
        if self.slot_picked.is_some() {
            return;
        }
        let slot = ((position[0] + 1.0) * BOARD_WIDTH as f32 / 2.0) as usize;
        self.slot_picked = Some(slot);
    }

    /// Draws the board and eventually the falling stone.
    fn draw(&self, graphics: &GraphicsPainter, black_board: &Blackboard) {
        if self.animator.is_animating() {
            self.animator.draw(graphics);
        }
        graphics.render_board(&black_board.game_board);
    }
}
