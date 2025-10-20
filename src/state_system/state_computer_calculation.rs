//! In this state the real computation happens, and also the player animation is executed to
//! cover up some calculation time. The calculation happens asynchronously in a separate
//! working thread.

use crate::board_logic::alpha_beta::AlphaBeta;
use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::bit_board_coding::BOARD_WIDTH;
use crate::debug_check_board_coordinates;
use crate::render_system::graphics::GraphicsPainter;
use crate::render_system::stone_animator::StoneAnimator;
use crate::state_system::game_state::{Blackboard, GameState, GameStateIndex};
use std::sync::mpsc;
use std::thread;

pub struct StateComputerCalculation {
    animator: StoneAnimator,
    receiver: mpsc::Receiver<usize>,
    sender: mpsc::Sender<BitBoard>,
}

impl StateComputerCalculation {
    pub fn new() -> StateComputerCalculation {
        let (result_sender, result_receiver) = mpsc::channel::<usize>();
        let (task_sender, task_receiver) = mpsc::channel::<BitBoard>();

        // Kick of a worker thread, that runs in the background.
        thread::spawn(move || {
            let mut ai = AlphaBeta::new();
            loop {
                let local_board = task_receiver.recv().unwrap();
                let result = ai.get_best_move(local_board);
                result_sender.send(result).unwrap();
            }
        });

        StateComputerCalculation {
            animator: StoneAnimator::new(),
            receiver: result_receiver,
            sender: task_sender,
        }
    }
}

impl GameState for StateComputerCalculation {
    /// Here we start the animation of the stone and feed the new situation to the worker
    /// thread to perform the computations.
    fn enter(&mut self, black_board: &Blackboard) {
        let mut local_board = black_board.game_board.clone();
        // Pre make the player move.
        local_board.apply_move_on_column(black_board.player_choice, false);

        self.sender.send(local_board).unwrap();

        // Start the animation.
        self.animator
            .start_animating(&black_board.game_board, black_board.player_choice, false);
    }

    /// In the update we perform the animation and once it is finished we check with the worker
    /// thread, if the results are present and if so leave the thread for execution.
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
            debug_check_board_coordinates!(col: result);
            black_board.computer_choice = result;
            return Some(GameStateIndex::ComputerExecutionState);
        }

        None
    }

    /// We do not process mouse clicks here.
    fn mouse_click(&mut self, _: [f32; 2]) {
        // Nothing to do here.
    }

    /// Draws the board and eventually the falling stone.
    fn draw(&self, graphics: &GraphicsPainter, black_board: &Blackboard) {
        if self.animator.is_animating() {
            self.animator.draw(graphics);
        }

        graphics.render_board(&black_board.game_board);
    }
}
