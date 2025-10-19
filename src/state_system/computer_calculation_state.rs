//! In this state the real computation happens, and also the player animation is executed to
//! cover up some calculation time. The calculation happens asynchronously in a Tokio thread.

use tokio::sync::oneshot;
use crate::render_system::stone_animator::StoneAnimator;
use crate::board_logic::move_ai::MoveAI;
use crate::render_system::graphics::GraphicsPainter;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};

pub struct ComputerCalculationState {
    animator: StoneAnimator,
    receiver: Option<oneshot::Receiver<usize>>
}

impl ComputerCalculationState {
    pub fn new() -> ComputerCalculationState {
        ComputerCalculationState {
            animator : StoneAnimator::new(),
            receiver : None
        }
    }
}

impl GameState for ComputerCalculationState {
    fn enter(&mut self, black_board: &Blackboard) {
        let mut local_board = black_board.game_board.clone();

        // Pre make the player move.
        local_board.apply_move_on_column(black_board.player_choice, false);
        let (tx, rx) = oneshot::channel();
        self.receiver = Some(rx);
        // Kick of the calculation.
        tokio::spawn(async move {
           let mut ai = MoveAI::new(local_board);
            tx.send( ai.get_best_move()).expect("Receiver already dropped.");
        });

        // Start the animation.
        self.animator.start_animating(&black_board.game_board, black_board.player_choice, false );
    }

    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.animator.is_animating() {
            self.animator.update(delta_time);
            if !self.animator.is_animating() {
                black_board.game_board.apply_move_on_column(black_board.player_choice, false);
            }
            return None;
        }


        if let Some(receiver) = self.receiver.as_mut() {
            if let Ok(result) = receiver.try_recv() {
                black_board.computer_choice = result;
                return Some(GameStateIndex::ComputerExecutionState);
            }
        }


        None

    }

    fn mouse_click(&mut self, _: [f32; 2]) {
        // Nothing to do here.
    }

    fn draw(&self, graphics: &GraphicsPainter, black_board: &Blackboard) {
        if self.animator.is_animating() {
            self.animator.draw(graphics);
        }

        graphics.render_board(&black_board.game_board);
    }
}

