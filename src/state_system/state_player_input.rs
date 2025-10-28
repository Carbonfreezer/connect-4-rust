//! The player input state administrates the players choice, checks for feasibility and it it would
//! result in an game over it also executes the falling stone animation. If this is not the end of
//! the game, the computer move calculation is kicked off, before the animations starts.

use crate::board_logic::bit_board_coding::BOARD_WIDTH;
use crate::render_system::graphics::{WINDOW_DIMENSION, render_board};
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};
use macroquad::math::Vec2;

pub struct StatePlayerInput {
    /// The choice coming from the user interface.
    slot_picked: Option<u32>,
    /// The stone animator we use.
    animator: StoneAnimator,
    /// A flag whether we want to transition to game over in the end,
    transition_to_game_over: bool,
    ///  The buffered move we need to execute.
    buffered_move: u64,
    /// Indicates, that we are waiting for player input.
    waiting_for_player: bool,
}

impl StatePlayerInput {
    pub fn new() -> StatePlayerInput {
        StatePlayerInput {
            slot_picked: None,
            animator: StoneAnimator::new(),
            transition_to_game_over: false,
            buffered_move: 0,
            waiting_for_player: false,
        }
    }
}

impl GameState for StatePlayerInput {
    fn enter(&mut self, _: &Blackboard) {
        self.slot_picked = None;
        self.transition_to_game_over = false;
        self.waiting_for_player = true;
    }

    /// We handle the stone animation and if not and the player has chosen a slot, we decide
    /// depending on whether it s game over or not to transition to the computer choice state
    /// or start the animation to follow up on game over.
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.waiting_for_player {
            let slot_choice = self.slot_picked?;

            // We have chosen a slot.
            self.slot_picked = None;

            let coded_move = black_board.game_board.get_possible_move(slot_choice);
            // Illegal move.
            if coded_move == 0 {
                return None;
            }

            self.waiting_for_player = false;
            let mut clon = black_board.game_board.clone();
            clon.apply_move(coded_move, false);
            // See if we transition to game over in the end.
            self.transition_to_game_over = clon.is_game_over();
            self.buffered_move = coded_move;
            self.animator
                .start_animating(&black_board.game_board, slot_choice, false);

            return None;
        }

        // In this case the stone is falling.
        // In this case we have some animation going.
        self.animator.update(delta_time);
        if self.animator.is_animating() {
            return None;
        }

        // Animation is over at that point.
        black_board.game_board.apply_move(self.buffered_move, false);

        if self.transition_to_game_over {
            Some(GameStateIndex::GameOverState)
        } else {
            Some(GameStateIndex::ComputerExecutionState)
        }
    }

    /// Picks the slot, that was chosen by the player.
    fn mouse_click(&mut self, position: Vec2) {
        if self.slot_picked.is_some() {
            return;
        }
        let slot = (position.x / WINDOW_DIMENSION * BOARD_WIDTH as f32) as u32;
        self.slot_picked = Some(slot);
    }

    /// Draws the board and eventually the falling stone.
    fn draw(&self, black_board: &Blackboard) {
        if self.animator.is_animating() {
            self.animator.draw();
        }

        render_board(&black_board.game_board, &black_board.board_texture);
    }
}
