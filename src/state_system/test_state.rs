//! This is a dummy state for testing only during implementation.

use crate::game_state::{Blackboard, GameState, GameStateIndex};
use crate::render_system::graphics::GraphicsPainter;
use crate::render_system::stone_animator::StoneAnimator;

pub struct TestState {
    passed_time: f32,
    slot_picked: Option<usize>,
    animator : StoneAnimator,
    move_destination : u64,
    awaiting_placement: bool,
    is_computer : bool
}

impl TestState {
    pub fn new() -> TestState {
        TestState {
            passed_time: 0.0,
            slot_picked: None,
            animator : StoneAnimator::new(),
            move_destination: 0,
            awaiting_placement: false,
            is_computer : false
        }
    }
}

impl GameState for TestState {
    fn enter(&mut self, _: &Blackboard) {
        self.passed_time = 0.0;
        self.animator.stop_animating();
        self.is_computer = false;
    }

    fn update(&mut self, delta_time: f32, board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.animator.is_animating() {
            self.animator.update(delta_time);
            return None;
        }

        if self.awaiting_placement {
            self.awaiting_placement = false;
            board.game_board.apply_move(self.move_destination, self.is_computer);
            self.is_computer = !self.is_computer;
        }

        if let Some(pos) = self.slot_picked {
            self.slot_picked = None;

            let mov = board.game_board.get_possible_move(pos);
            if mov != 0 {
                self.animator.start_animating(&board.game_board, pos, self.is_computer);
                self.awaiting_placement = true;
                self.move_destination = mov;
            }
        }

        None
    }

    fn mouse_click(&mut self, position: [f32; 2]) {
        let slot = ((position[0] + 1.0) * 3.5) as usize;
        if self.slot_picked.is_none() {
            self.slot_picked = Some(slot);
        }
    }

    fn draw(&self, graphics: &GraphicsPainter, board: &Blackboard) {
        if self.animator.is_animating() {
            self.animator.draw(graphics);
        }
        graphics.render_board(&board.game_board);
    }
}
