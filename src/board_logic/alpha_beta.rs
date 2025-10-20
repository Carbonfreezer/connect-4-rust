//! This is the entrance point to the real AI calculation.
//! It is an Alpha-Beta pruned negamax algorithm with a transposition table.
//! Alpha-Beta pruning is enhanced by heuristically presorting the movement options.
//! The transposition table is enhanced by a canonical board coding and a coding that
//! accounts for symmetry.

use crate::board_logic::bit_board::{BitBoard, SymmetryIndependentPosition};
use crate::board_logic::bit_board_coding::BOARD_WIDTH;
use std::collections::HashMap;
use std::time::Duration;

/// Contains a bit-board and a hashmap.
pub struct AlphaBeta {
    bit_board: BitBoard,
    hash_map: HashMap<SymmetryIndependentPosition, i8>,
}

impl AlphaBeta {
    /// The bit board is handed over intentionally with a move situations.
    pub fn new() -> AlphaBeta {
        AlphaBeta {
            bit_board: BitBoard::new(),
            hash_map: HashMap::new(),
        }
    }

    /// Evaluate the next move and returns the applied move and the value.
    fn evaluate_next_move(&mut self) -> (usize, i8) {
        let possibilities = (0..BOARD_WIDTH)
            .into_iter()
            .find(|&x| self.bit_board.get_possible_move(x) != 0)
            .unwrap();
        // HACK HACK HACK for testing.
        std::thread::sleep(Duration::from_secs(1));

        (possibilities, 0)
    }
    /// Gets the best move for the AI, sets the bit board and does all the computations.
    pub fn get_best_move(&mut self, bit_board: BitBoard) -> usize {
        self.bit_board = bit_board;
        let (mov, _) = self.evaluate_next_move();
        self.hash_map.clear();
        mov
    }
}
