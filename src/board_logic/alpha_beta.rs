//! This is the entrance point to the real AI calculation.
//! It is an Alpha-Beta pruned negamax algorithm with a transposition table.
//! Alpha-Beta pruning is enhanced by heuristically presorting the movement options.
//! The transposition table is enhanced by a canonical board coding and a coding that
//! accounts for symmetry.

use crate::board_logic::bit_board::{BitBoard, SymmetryIndependentPosition};
use crate::board_logic::bit_board_coding::BOARD_WIDTH;
use crate::board_logic::bit_board_coding::{FULL_BOARD_MASK, check_for_winning};
use crate::board_logic::heuristic::compute_heuristics;
use crate::debug_check_board_coordinates;
use std::collections::HashMap;

/// The search depth we want to apply.
const SEARCH_DEPTH: u32 = 15;

/// We clamp values to the region of 1: guaranteed winn to -1: guaranteed loss.
const MAX_SCORE: f32 = 1.0;

/// This score is lower than any of the others, we use it as an initialization to check to build the maximum.
const SCORE_GUARD: f32 = -1.1;

/// The discount factor to favour fast wins and late losses.
/// This should be extremely close to 1 because it interferes negatively with the
/// transposition table.
const DISCOUNT_FACTOR: f32 = 0.99999;

/// The region we want to clamp the heuristics against, that it
/// can never dominate even overdiscounted win / loss.
const CLAMP_GUARD_HEURISTIC: f32 = 0.97;

/// Contains a bit-board and two hashmaps. One for the current move and one recycled
/// from the previous one.
pub struct AlphaBeta {
    /// The bit board we play with.
    bit_board: BitBoard,
    /// The hash map of the current generation.
    hash_map: HashMap<SymmetryIndependentPosition, f32>,
    /// The hash map of the previous move / generation. It may not be used any more for position
    /// look up but for heuristical evaluation in move ordering.
    hash_map_old: HashMap<SymmetryIndependentPosition, f32>,
}

/// The working list are the elements of what we need to do.
struct WorkingListEntry {
    /// The coded move is the bit set at where the stone needs to go.
    coded_move: u64,
    /// The is the slot index that has been chosen.
    slot: u32,
    /// The initial estimate move the move value, to make the choice on where to go.
    evaluation: f32,
}

/// A result we get for the presorting. The presort result is used for
/// move ordering to help the alpha beta clip. Eventually found end games and
/// some of the TT look ups are already filtered out at this stage.
struct PresortResult {
    /// The maximum score we have reached on precached moves and winnings.
    pub max_score: f32,
    /// The move that belongs to the best score.
    pub best_move: Option<u32>,
    /// The list with the remainder we still have to process. Contains the coded board,
    /// the move index and the evaluation. The position is to avoid recalculating the has structs.
    pub working_list: Vec<WorkingListEntry>,
}

impl AlphaBeta {
    /// The bit board is handed over intentionally with a move situations.
    pub fn new() -> AlphaBeta {
        AlphaBeta {
            bit_board: BitBoard::new(),
            hash_map: HashMap::new(),
            hash_map_old: HashMap::new(),
        }
    }

    /// Generates a vector of (coded Move, chosen slot, heuristic evaluation) and returns it
    /// sorted by heuristic value in descending order. This can be used to scan the options in an efficient way for
    /// Alpha-Beta.
    fn get_pre_sorted_move_list(&mut self) -> PresortResult {
        let mut local_max = SCORE_GUARD;
        let mut local_move = None;
        let mut test_board = self.bit_board.clone();
        let mut local_sorter = Vec::<WorkingListEntry>::new();

        for (coded_move, slot) in self.bit_board.get_all_possible_moves() {
            // Test execute the move.
            test_board.own_stones |= coded_move;
            // First we try the immediate situations, because it is a win a loss or a draw.
            if check_for_winning(test_board.own_stones) {
                local_max = MAX_SCORE;
                local_move = Some(slot);
            } else if ((test_board.own_stones | test_board.opponent_stones) == FULL_BOARD_MASK)
                && (local_max < 0.0)
            {
                local_max = 0.0;
                local_move = Some(slot);
            }
            // Then we look in the transposition tables.
            else {
                // As Swap the player to get the values. because we encoded the player from the follow up move.
                test_board.swap_players();
                let search_key = test_board.get_symmetry_independent_position();
                test_board.swap_players();

                // See if it is in the current transposition table.
                // If we found it here, we can insert the result and do not need to analyze the node any further.
                if let Some(evaluation) = self.hash_map.get(&search_key) {
                    let score = -*evaluation;
                    if score > local_max {
                        local_max = score;
                        local_move = Some(slot);
                    }
                } else {
                    // Hopefully it is still in the transposition table from last move.
                    // In this case we take this as a heuristic evaluation.
                    if let Some(evaluation) = self.hash_map_old.get(&search_key) {
                        local_sorter.push(WorkingListEntry {
                            coded_move,
                            slot,
                            evaluation: -*evaluation,
                        });
                    }
                    // Heere we have to apply our heuristics.
                    else {
                        local_sorter.push(WorkingListEntry {
                            coded_move,
                            slot,
                            evaluation: compute_heuristics(&test_board, CLAMP_GUARD_HEURISTIC),
                        });
                    }
                }
            }
            // Retake move.
            test_board.own_stones ^= coded_move;
        }

        // Do the inverse sort (descending order.).
        local_sorter.sort_by(|first, second| second.evaluation.total_cmp(&first.evaluation));

        PresortResult {
            working_list: local_sorter,
            max_score: local_max,
            best_move: local_move,
        }
    }

    /// Evaluate the next move and returns the applied move and the value. This is the implementation
    /// of the Negamax algorithm.
    ///
    /// # Parameters:
    /// * **alpha**: The alpha value of the alpha beta algorithm. This is the value that gets increased.
    /// * **beta**: The beta value of the alpha beta algorithm. This is the oner that generates the early exit.
    /// * **heuristics**: The heuristical evaluation of the current situation.
    /// * **depth**: The current search depth.
    ///
    /// # Returns
    /// A pair of the node evaluation and eventually a chosen move. In the case of a TT hit or max search_depth we do not
    /// generate this (None).
    fn evaluate_next_move(
        &mut self,
        alpha: f32,
        beta: f32,
        heuristics: f32,
        depth: u32,
    ) -> (f32, Option<u32>) {
        // We should never wind up in a situation where the current position is a draw or winning,
        // because that has already been checked in get_pre_sorted_move_list from previous call. We insert it as
        // debug assert here.
        debug_assert!(
            !check_for_winning(self.bit_board.opponent_stones),
            "This should already have been prechecked."
        );
        // Same for draw.
        debug_assert!(
            (self.bit_board.own_stones | self.bit_board.opponent_stones) != FULL_BOARD_MASK,
            "The case that we have have a draw should have also already been prechecked."
        );

        let search_key = self.bit_board.get_symmetry_independent_position();
        if let Some(&cached_value) = self.hash_map.get(&search_key) {
            // Transposition hit!
            return (cached_value, None);
        }

        // If we have reached max depth we simply return the heuristics value.
        if depth == SEARCH_DEPTH {
            return (heuristics, None);
        }

        let mut best_value = SCORE_GUARD;
        let mut best_slot = 0;

        let presort_result = self.get_pre_sorted_move_list();
        let mut alpha = alpha;
        // The presort result has already filtered out sone moves, that either run into an ending or are already completely analyzed.
        if presort_result.best_move.is_some() {
            best_slot = presort_result.best_move.unwrap();
            best_value = presort_result.max_score;
        }

        // We may need to do an alpha beta check here and can eventually return.
        if best_value > alpha {
            alpha = best_value;
            if best_value >= beta {
                self.hash_map.insert(search_key, best_value);
                return (best_value, Some(best_slot));
            }
        }

        // We start searching now.
        for list_entry in presort_result.working_list.iter() {
            // Apply move.
            self.bit_board.own_stones |= list_entry.coded_move;
            self.bit_board.swap_players();
            let (new_result, _) =
                self.evaluate_next_move(-beta, -alpha, -list_entry.evaluation, depth + 1);
            self.bit_board.swap_players();
            self.bit_board.own_stones ^= list_entry.coded_move;

            let adjusted_result = -new_result * DISCOUNT_FACTOR;
            if adjusted_result > best_value {
                best_value = adjusted_result;
                best_slot = list_entry.slot;
                if adjusted_result > alpha {
                    alpha = adjusted_result;
                }
            }

            // Early out here.
            if adjusted_result > beta {
                break;
            }
        }

        // Insert value into hashmap.
        self.hash_map.insert(search_key, best_value);

        (best_value, Some(best_slot))
    }

    /// Gets the best move for the AI, sets the bit board and does all the computations.
    pub fn get_best_move(&mut self, bit_board: BitBoard) -> u32 {
        self.bit_board = bit_board;

        let (_, mov) = self.evaluate_next_move(-MAX_SCORE, MAX_SCORE, 0.0, 0);

        // Demote hash map.
        self.hash_map_old = self.hash_map.clone();
        self.hash_map.clear();
        debug_assert!(mov.is_some(), "We wound up with an empty move here");
        let mov = mov.unwrap();
        debug_check_board_coordinates!(col: mov);
        mov
    }
}
