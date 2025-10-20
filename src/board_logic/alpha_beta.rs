//! This is the entrance point to the real AI calculation.
//! It is an Alpha-Beta pruned negamax algorithm with a transposition table.
//! Alpha-Beta pruning is enhanced by heuristically presorting the movement options.
//! The transposition table is enhanced by a canonical board coding and a coding that
//! accounts for symmetry.

use crate::board_logic::bit_board::{BitBoard, SymmetryIndependentPosition};
use crate::board_logic::bit_board_coding::{check_for_free_three,  check_for_winning,  FULL_BOARD_MASK};
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


    
    /// Generates a vector of (coded Move, chosen slot, heuristic evaluation) and returns it
    /// sorted by heuristic value in descending order. This can be used to scan the options in an efficient way for
    /// Alpha-Beta.
    fn get_pre_sorted_move_list(&mut self) ->  Vec<(u64, usize, u32)> {
        let mut test_board = self.bit_board.clone();
        let combined_list = self.bit_board.get_all_possible_moves().map(|(coded, slot)|
             {

                 // TODO: Include the transposition table later on.

                 let mut val;
                 test_board.own_stones |= coded;
                 test_board.opponent_stones |= coded;
                 if check_for_winning(test_board.own_stones) {
                     // We would get a win.
                     val = 1_000_000;
                 } else if check_for_winning(test_board.opponent_stones) {
                     // We would get a loss.
                     val = 900_000;
                 } else if (test_board.own_stones | test_board.opponent_stones) == FULL_BOARD_MASK {
                     // Draft
                     val = 0;
                 } else {
                     val = check_for_free_three(test_board.own_stones) * 100;

                 }
                 test_board.opponent_stones ^= coded;
                 test_board.own_stones ^= coded;

                 let micro_score : u32 = match slot {
                     0 => 3,
                     1 => 4,
                     2 => 5,
                     3 => 7,
                     4 => 5,
                     5 => 5,
                     6 => 3,
                     _ => panic!("Invalid slot {}!", slot)
                 };
                 
                 val += micro_score;
                 
                 (coded, slot, val)
             });

        let mut result : Vec<(u64, usize, u32)>= combined_list.into_iter().collect();
        // Make an inverse sort.
        result.sort_by_key(|(_,_,val)| u32::MAX - *val);

        result
    }



    /// Evaluate the next move and returns the applied move and the value.
    fn evaluate_next_move(&mut self) -> (usize, i8) {

        let test_vec = self.get_pre_sorted_move_list();

        let (_, board_move, _) = test_vec[0];

        // HACK HACK HACK for testing.
        std::thread::sleep(Duration::from_secs(1));

        (board_move, 0)
    }

    /// Gets the best move for the AI, sets the bit board and does all the computations.
    pub fn get_best_move(&mut self, bit_board: BitBoard) -> usize {
        self.bit_board = bit_board;
        let (mov, _) = self.evaluate_next_move();
        self.hash_map.clear();
        mov
    }
}
