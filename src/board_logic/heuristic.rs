//! This is the place for all functions related to heuristically evaluations of the game situation
//! Heuristics are kept relatively simple. We account for open positions of three stones, doublets
//! whether dead or not and a board scoring that favours positions close to the central column.

use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::bit_board_coding::{
    DIR_INCREMENT, FULL_BOARD_MASK, clip_shift, clip_shift_inverse, get_bit_representation,
};

/// Returns the number of open triplets we have.
fn count_open_three(board: u64, free_spots: u64) -> u32 {
    let mut triplets = 0;

    for bit_shift in DIR_INCREMENT {
        // XXX_ Pattern
        let double_pos = clip_shift(board, bit_shift) & board;
        let dd = clip_shift(double_pos, bit_shift) & board;
        let triplets_after = clip_shift(dd, bit_shift) & free_spots;
        triplets += triplets_after.count_ones();

        // XX_X pattern
        let free_match = clip_shift(double_pos, bit_shift) & free_spots;
        let spot_right = clip_shift(free_match, bit_shift) & board;
        triplets += spot_right.count_ones();

        // X_XX pattern
        let free_left_match =
            clip_shift_inverse(clip_shift_inverse(double_pos, bit_shift), bit_shift) & free_spots;
        let spot_left = clip_shift_inverse(free_left_match, bit_shift) & board;
        triplets += spot_left.count_ones();

        // _XXX Pattern
        let triplets_before = clip_shift_inverse(
            clip_shift_inverse(clip_shift_inverse(dd, bit_shift), bit_shift),
            bit_shift,
        ) & free_spots;
        triplets += triplets_before.count_ones();
    }

    triplets
}

/// This function turns standard values from the literature into representations
/// that scale with our internal structure.
const fn make_adjusted_value() -> [f32; 12] {
    #[rustfmt::skip]
    let mut local: [f32; 12] = [
        0.0, 1.0, 3.0, 6.0, 
        0.5, 2.0, 6.0, 8.0, 
        1.5, 3.0, 8.0, 10.0,
    ];

    let mut i = 0;
    while i < 12 {
        local[i] = local[i] * local[i] * 0.001;
        i += 1;
    }
    local
}

/// This generates the bit mask to be able to read out the value table from above.
const fn make_value_bitmask() -> [u64; 12] {
    let mut mask: [u64; 12] = [0; 12];
    let mut y_scan = 0;
    while y_scan < 3 {
        let mut x_scan = 0;
        while x_scan < 4 {
            mask[((3 - x_scan) + 4 * (2 - y_scan)) as usize] =
                get_bit_representation(3 - x_scan, 2 - y_scan)
                    | get_bit_representation(3 + x_scan, 2 - y_scan)
                    | get_bit_representation(3 - x_scan, 3 + y_scan)
                    | get_bit_representation(3 + x_scan, 3 + y_scan);
            x_scan += 1;
        }
        y_scan += 1;
    }
    mask
}

/// This contains the values for the different board positions.
const BOARD_POSITION_CODING_VALUE: [f32; 12] = make_adjusted_value();

/// This is the bit masking to index the value mask.
const VALUE_POSITION_BITMASK: [u64; 12] = make_value_bitmask();

/// Evaluates the stones by their position on the board. Gives center stones a higher
/// value, because they can generate more possibilities in the future.
fn get_board_scoring(board: u64) -> f32 {
    let mut score = 0.0;

    for i in 0..12 {
        let pos_ind = (board & VALUE_POSITION_BITMASK[i]).count_ones();
        score += BOARD_POSITION_CODING_VALUE[i] * pos_ind as f32;
    }

    score
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
    let own_triplets = count_open_three(board_analyzed.own_stones, free_spots);
    score += own_triplets as f32 * 0.04;
    let opp_triplets = count_open_three(board_analyzed.opponent_stones, free_spots);
    score -= opp_triplets as f32 * 0.04;

    // 2. board control.
    score += get_board_scoring(board_analyzed.own_stones);
    score -= get_board_scoring(board_analyzed.opponent_stones);

    // We do not clamp against exactly one, so that whatever the outcome is,
    // it will always be dominated by a guaranteed win or loss.
    score.clamp(-clamp_guard, clamp_guard)
}
