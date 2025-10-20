//! This is the entrance point to the real AI calculation.
//! It is an Alpha-Beta pruned negamax algorithm with a transposition table.
//! Alpha-Beta pruning is enhanced by heuristically presorting the movement options.
//! The transposition table is enhanced by a canonical board coding and a coding that
//! accounts for symmetry.

use crate::board_logic::bit_board::{BitBoard, SymmetryIndependentPosition};
use crate::board_logic::bit_board_coding::{check_for_free_three,  check_for_winning,  FULL_BOARD_MASK};
use std::collections::HashMap;

/// The maximum value we use for evaluation.
const MAXIMUM_SCORE: i8 = 100;

/// The dummy move we use as an index.
const DUMMY_MOVE : usize = 100;

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
    fn get_pre_sorted_move_list(&mut self) -> Vec<(u64, usize, u32)> {
        let mut test_board = self.bit_board.clone();
        let combined_list = self.bit_board.get_all_possible_moves().map(|(coded, slot)| {
            let mut val: u32;
            test_board.own_stones |= coded;

            if check_for_winning(test_board.own_stones) {
                val = 1_000_000;
            }
            else if check_for_winning(test_board.opponent_stones | coded) {
                val = 900_000;
            }
            else if (test_board.own_stones | test_board.opponent_stones) == FULL_BOARD_MASK {
                val = 0;
            }
            else {
                let search_key = test_board.get_symmetry_independent_position();
                
                if let Some(found_value) = self.hash_map.get(&search_key) {
                    val = ((*found_value) as i32 + 200) as u32;
                } else {
                    val = check_for_free_three(test_board.opponent_stones) * 100;
                    val += match slot {
                        0 | 6 => 3,
                        1 | 5 => 4,
                        2 | 4 => 5,
                        3 => 7,
                        _ => panic!("Invalid slot {}!", slot)
                    };
                }
            }


            test_board.own_stones ^= coded;
            (coded, slot, val)
        });

        let mut result: Vec<(u64, usize, u32)> = combined_list.collect();
        result.sort_by_key(|(_, _, val)| u32::MAX - *val);
        result
    }


    /// Evaluate the next move and returns the applied move and the value.
    fn evaluate_next_move(&mut self, alpha : i8, beta : i8, depth : i8) -> (usize, i8) {

        // First we check if the opponent has scored a win.
        if check_for_winning(self.bit_board.opponent_stones) {return (DUMMY_MOVE, -MAXIMUM_SCORE + depth)};
        // Then we check for draw
        if (self.bit_board.own_stones | self.bit_board.opponent_stones) == FULL_BOARD_MASK {return (DUMMY_MOVE, 0)};

        let search_key = self.bit_board.get_symmetry_independent_position();

        // Try to look it up in the table.
        if let Some(result_value) = self.hash_map.get(&search_key) {
            return (DUMMY_MOVE, *result_value);
        }

        let mut best_value = - (MAXIMUM_SCORE + 1);
        let mut best_slot= 0;

        let mut alpha = alpha;
        // We start searching now.

        let todo_list = self.get_pre_sorted_move_list();
        for (coded_move, slot, _) in todo_list.iter() {
            // Apply move.
            self.bit_board.own_stones |= coded_move;
            self.bit_board.swap_players();
            let (_, new_result) = self.evaluate_next_move(-beta, -alpha, depth + 1);
            self.bit_board.swap_players();
            self.bit_board.own_stones ^= coded_move;

            let adjusted_result = - new_result;
            if adjusted_result > best_value {
                best_value = adjusted_result;
                best_slot = *slot;

                if adjusted_result > alpha {alpha = adjusted_result; }
            }

            // Early out here.
            if adjusted_result > beta {break;}
        }

        // Insert value into hashmap.
        self.hash_map.insert(search_key, best_value);

        (best_slot, best_value)
    }

    /// Gets the best move for the AI, sets the bit board and does all the computations.
    pub fn get_best_move(&mut self, bit_board: BitBoard) -> usize {
        self.bit_board = bit_board;
        let (mov, _) = self.evaluate_next_move(i8::MIN, i8::MAX, 0);
        self.hash_map.clear();
        mov
    }
}
