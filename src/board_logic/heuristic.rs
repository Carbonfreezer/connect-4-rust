//! This is the place for all functions related to heuristically evaluations of the game situation
//! Heuristics are kept relatively simple. We account for open positions of three stones, doublets
//! whether dead or not and a board scoring that favours positions close to the central column.

use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::bit_board_coding::{
    DIR_INCREMENT, FULL_BOARD_MASK, clip_shift, get_column_mask,
};

/// Returns the number doublets and open triplets we have.
fn count_open_three_and_doubles(board: u64, free_spots: u64) -> (u32, u32) {
    let mut triplets = 0;
    let mut doublets = 0;

    for bit_shift in DIR_INCREMENT {
        // XXX_ Pattern
        let d = clip_shift(board, bit_shift) & board;
        doublets += d.count_ones();
        let dd = clip_shift(d, bit_shift) & board;
        let triplets_after = clip_shift(dd, bit_shift) & free_spots;
        triplets += triplets_after.count_ones();

        // _XXX Pattern
        let triplets_before = (dd >> (3 * bit_shift)) & free_spots;
        triplets += triplets_before.count_ones();
    }

    (doublets, triplets)
}

/// Masking central column, the two columns beside the central and one pair even one further out.
const BOARD_EVALUATION_MASK: [u64; 3] = [
    get_column_mask(3),
    get_column_mask(2) | get_column_mask(4),
    get_column_mask(1) | get_column_mask(5),
];

/// Counts the amount of stones, that are on the centerline, one line away from the center line
/// and two lines away from the center line and multiplies it with a scoring and adds it up.
fn get_board_scoring(board: u64) -> f32 {
    let center = (board & BOARD_EVALUATION_MASK[0]).count_ones() as f32 * 0.015;
    let one_off_center = (board & BOARD_EVALUATION_MASK[1]).count_ones() as f32 * 0.007;
    let two_off_center = (board & BOARD_EVALUATION_MASK[2]).count_ones() as f32 * 0.003;

    center + one_off_center + two_off_center
}

/// Does the complete heuristic evaluation of the game board.
pub fn compute_heuristics(board_analyzed: &BitBoard, clamp_guard: f32) -> f32 {
    debug_assert!(
        !board_analyzed.is_game_over(),
        "The game over state should have already been prechecked."
    );

    let free_spots =
        !(board_analyzed.opponent_stones | board_analyzed.own_stones) & FULL_BOARD_MASK;
    let mut score = 0.0;

    // 1. Pairing combination
    let (doublets, open_three) =
        count_open_three_and_doubles(board_analyzed.own_stones, free_spots);
    score += open_three as f32 * 0.04;
    score += doublets as f32 * 0.01;
    let (doublets, open_three) =
        count_open_three_and_doubles(board_analyzed.opponent_stones, free_spots);
    score -= open_three as f32 * 0.04;
    score -= doublets as f32 * 0.01;

    // 2. board control.
    score += get_board_scoring(board_analyzed.own_stones);
    score -= get_board_scoring(board_analyzed.opponent_stones);

    // We do not clamp against exactly one, so that whatever the outcome is,
    // it will always be dominated by a guaranteed win or loss.
    score.clamp(-clamp_guard, clamp_guard)
}
