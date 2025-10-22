//! This module contains bit board coding helper functions and a lot of
//! constants. Those are calculated at compile time. As for loops are not allowed
//! in const functions they have been reformulated to while loops.

//! Uses a bit board along the following structure:  
//! (48) (49) (50) (51) (52) (53) (54) (55)  
//!  40   41   42   43   44   45   46  (47)  
//!  32   33   34   35   36   37   38  (39)  
//!  24   25   26   27   28   29   30  (31)  
//!  16   17   18   19   20   21   22  (23)  
//!   8   9    10   11   12   13   14  (15)  
//!   0   1     2    3    4    5    6  ( 7)  
//!  
//! The number in parentheses are sentinel guards.

use crate::debug_check_board_coordinates;

/// The width of the board.
pub const BOARD_WIDTH: u32 = 7;

/// The height of the board.
pub const BOARD_HEIGHT: u32 = 6;

/// Gets a mask, where the bit at the indicated position is set.
#[inline(always)]
pub const fn get_bit_representation(x: u32, y: u32) -> u64 {
    1 << (x + 8 * y)
}

/// Generates a mask for one specific column.
pub const fn get_column_mask(col: u32) -> u64 {
    let mut result: u64 = 0;
    let mut y = 0;
    while y < BOARD_HEIGHT {
        result |= get_bit_representation(col, y);
        y += 1;
    }
    result
}

/// Helpful to mask out the sentinel.
const fn get_full_board_mask() -> u64 {
    let mut result: u64 = 0;
    let mut x = 0;
    while x < BOARD_WIDTH {
        let mut y = 0;
        while y < BOARD_HEIGHT {
            result |= get_bit_representation(x, y);
            y += 1;
        }
        x += 1;
    }
    result
}

/// Generates a mask for the lowest line.
const fn get_bottom_filler_mask() -> u64 {
    let mut result: u64 = 0;
    let mut x = 0;
    while x < BOARD_WIDTH {
        result |= get_bit_representation(x, 0);
        x += 1;
    }
    result
}

/// Flags out the seven different columns.
pub const COLUMN_MASK: [u64; 7] = [
    get_column_mask(0),
    get_column_mask(1),
    get_column_mask(2),
    get_column_mask(3),
    get_column_mask(4),
    get_column_mask(5),
    get_column_mask(6),
];

/// Flags the full board except for the sentinel.
pub const FULL_BOARD_MASK: u64 = get_full_board_mask();
/// Flags the bottom line helpful to determine possible legal moves.
pub const BOTTOM_FILL_MASK: u64 = get_bottom_filler_mask();

/// Bit shift increment:  
/// 0  1   2  
/// \  |  /  
///    X -  3  
pub const DIR_INCREMENT: [u8; 4] = [7, 8, 9, 1];

/// Method to mirror a board along the y-axis.
pub fn flip_board(input: u64) -> u64 {
    let mut result: u64;
    result = (input & COLUMN_MASK[6]) >> 6;
    result |= (input & COLUMN_MASK[5]) >> 4;
    result |= (input & COLUMN_MASK[4]) >> 2;
    result |= input & COLUMN_MASK[3];
    result |= (input & COLUMN_MASK[2]) << 2;
    result |= (input & COLUMN_MASK[1]) << 4;
    result |= (input & COLUMN_MASK[0]) << 6;

    result
}
/// Slow method only to be used for board drawing, gets all elements from the boards as coordinates.
pub fn get_position_iterator(board: u64) -> impl Iterator<Item = (u32, u32)> {
    (0..BOARD_WIDTH)
        .flat_map(|x| (0..BOARD_HEIGHT).map(move |y| (x, y)))
        .filter(move |&(x, y)| board & (1 << (x + 8 * y)) != 0)
}

/// Applies the indicated shift for movement by the shift value and clips
/// the value against the sentinel.
#[inline(always)]
pub fn clip_shift(input: u64, amount: u8) -> u64 {
    (input << amount) & FULL_BOARD_MASK
}

/// Does the inverse clip shift.
#[inline(always)]
pub fn clip_shift_inverse(input: u64, amount: u8) -> u64 {
    (input >> amount) & FULL_BOARD_MASK
}


/// Gets a  representation, where the bit for the specific column is set where a move would wind up.
/// If it is not possible to make move in that column, a 0 is returned.
pub fn get_possible_move(board: u64, column: u32) -> u64 {
    debug_check_board_coordinates!(col: column);
    // Safely upshifted board extended with a bottom row.
    ((clip_shift(board, DIR_INCREMENT[1]) | BOTTOM_FILL_MASK) ^
        // The original board.
        board )
        // Filter out the desired column.
        & COLUMN_MASK[column as usize]
}

/// Checks if the game board contains a winning constellation.
/// Here the bit board representation really shines. Returns true
/// if the board has one sequence of rows.
///
/// The idea is:
/// board:
/// 001111000
/// d:
/// 000111000
/// dd:
/// 000001110
/// dd & d
/// 000001000
#[inline(always)]
pub fn check_for_winning(board: u64) -> bool {
    for bit_shift in DIR_INCREMENT {
        let d = clip_shift(board, bit_shift) & board;
        let dd = clip_shift(clip_shift(d, bit_shift), bit_shift);
        if (dd & d) != 0 {
            return true;
        }
    }

    false
}

/// Generates a board representation, where bits are set that belong to a winning combination.
/// Makes use of the fact, that *check_for_winning* effectively collapsed a winning combination
/// into one bit that is the furthest out in shift direction. So we invert the shift three times
/// and ore it together.
pub fn get_winning_board(board: u64) -> u64 {
    let mut result = 0;
    for bit_shift in DIR_INCREMENT {
        let d = clip_shift(board, bit_shift) & board;
        let dd = clip_shift(clip_shift(d, bit_shift), bit_shift);
        let mut flag = dd & d;

        // Now the last bit of every winning constellation is set.
        result |= flag;
        // We can safely shift back, because we came from there.
        for _ in 0..3 {
            flag >>= bit_shift;
            result |= flag;
        }
    }

    result
}

/// Gets an iterator for all possible moves for the AI. The iterator returns the move and the original
/// move index.
#[inline(always)]
pub fn get_all_possible_moves(board: u64) -> impl Iterator<Item = (u64, u32)> {
    let comb = (clip_shift(board, DIR_INCREMENT[1]) | BOTTOM_FILL_MASK) ^ board;
    (0..7)
        .map(move |x| (comb & COLUMN_MASK[x], x as u32))
        .filter(|&x| x.0 != 0)
}


/// Helper function to debug log a board on std out.
pub fn debug_log_board(board: u64) {
    for height in (0..BOARD_HEIGHT).rev() {
        for width in 0..BOARD_WIDTH {
            let pit_pos :u64 = 1 << (width + 8 * height);
            if (board & pit_pos) != 0 { print!("X") } else { print!("-") }
        }
        println!();
    }
}