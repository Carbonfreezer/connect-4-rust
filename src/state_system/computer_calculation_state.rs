//! In this state the real computation happens, and also the player animation is executed to
//! cover up some calculation time. The calculation happens asynchronously in a Tokio thread.

use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::move_ai::MoveAI;
use crate::render_system::graphics::GraphicsPainter;
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};
use std::sync::mpsc;
use std::thread;

pub struct ComputerCalculationState {
    animator: StoneAnimator,
    receiver: mpsc::Receiver<usize>,
    sender: mpsc::Sender<BitBoard>,
}

impl ComputerCalculationState {
    pub fn new() -> ComputerCalculationState {
        let (result_sender, result_receiver): (mpsc::Sender<usize>, mpsc::Receiver<usize>) =
            mpsc::channel();
        let (task_sender, task_receiver): (mpsc::Sender<BitBoard>, mpsc::Receiver<BitBoard>) =
            mpsc::channel();

        // Kick of a worker thread, that runs in the background.
        thread::spawn(move || {
            loop {
                let local_board = task_receiver.recv().unwrap();
                let mut ai = MoveAI::new(local_board);
                let result = ai.get_best_move();
                result_sender
                    .send(result)
                    .unwrap();
            }
        });

        ComputerCalculationState {
            animator: StoneAnimator::new(),
            receiver: result_receiver,
            sender: task_sender,
        }
    }
}

impl GameState for ComputerCalculationState {
    fn enter(&mut self, black_board: &Blackboard) {
        let mut local_board = black_board.game_board.clone();
        // Pre make the player move.
        local_board.apply_move_on_column(black_board.player_choice, false);

        self.sender
            .send(local_board)
            .unwrap();

        // Start the animation.
        self.animator
            .start_animating(&black_board.game_board, black_board.player_choice, false);
    }

    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex> {
        if self.animator.is_animating() {
            self.animator.update(delta_time);
            if !self.animator.is_animating() {
                black_board
                    .game_board
                    .apply_move_on_column(black_board.player_choice, false);
            }
            return None;
        }

        if let Ok(result) = self.receiver.try_recv() {
            black_board.computer_choice = result;
            return Some(GameStateIndex::ComputerExecutionState);
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
