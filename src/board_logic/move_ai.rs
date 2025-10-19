//! This is the entrance point to the real AI calculation.

use std::collections::HashMap;
use std::time::Duration;
use crate::board_logic::bit_board::{BitBoard, SymmetryIndependentPosition};
use crate::board_logic::bit_board_coding::BOARD_WIDTH;

/// Contains a bit-board and a hashmap.
pub struct MoveAI {
    bit_board: BitBoard,
    hash_map : HashMap<SymmetryIndependentPosition, i8>
}

impl MoveAI {
    /// The bit board is handed over intentionally with a move situations.
    pub fn new(bit_board: BitBoard) -> MoveAI {
        MoveAI {
            bit_board,
            hash_map: HashMap::new()
        }
    }


    /// Gets the best move for the AI.
    pub fn get_best_move(&mut self) -> usize
    {
        let possibilities = (0..BOARD_WIDTH).into_iter().find(|&x| self.bit_board.get_possible_move(x) != 0).unwrap();
        // HACK HACK HACK for testing.
        std::thread::sleep(Duration::from_secs(1));
        possibilities
    }
}
