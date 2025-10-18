use std::path::Iter;


/// Uses a bit board along the following structure:
/// (48) (49) (50) (51) (52) (53) (54) (55)
///  40   41   42   43   44   45   46  (47)
///  32   33   34   35   36   37   38  (39)
///  24   25   26   27   28   29   30  (31)
///  16   17   18   19   20   21   22  (23)
///   8   9    10   11   12   13   14  (15)
///   0   1     2    3    4    5    6  ( 7)
///
/// The number in parantheses are sentinal guards.



/// Here wee define a couple of masks that are helpful.
fn get_bit_representation(x: u8, y: u8) -> u64 {
    1 << (x + 8 * y)
}

/// Helpful to mask out the sentinel.
fn get_full_board() -> u64 {
    let mut result: u64 = 0;

    for x in 0..7 {
        for y in 0..6 {
            result = result | get_bit_representation(x, y);
        }
    }

    result
}

/// Helpful to mask out one column.

fn get_column_mask(col: u8) -> u64 {
    let mut result: u64 = 0;
    for y in 0..6 {
        result = result | get_bit_representation(col, y);
    }

    result
}


fn get_bottom_filler_mask() -> u64 {
    let mut result: u64 = 0;
    for x in 0..7 {
        result = result | get_bit_representation(x, 0);
    }
    result
}

/// Flags out the seven different columns.
pub const  COLUMN_MASK: [u64; 7] = [0x10101010101,
                                    0x20202020202,
                                    0x40404040404,
                                    0x80808080808,
                                    0x101010101010,
                                    0x202020202020,
                                    0x404040404040];

/// Flags the full board except for the sentinel.
pub const  FULL_BOARD_MASK: u64 =  0x7f7f7f7f7f7f;

/// Flags the bottom line helpful to determine possible legal moves.
pub static  BOTTOM_FILL_MASK: u64 = 0x7f;

/// Bit shift increment:
/// 0  1   2
/// \  |  /
///   X -  3
pub static DIR_INCREMENT: [u8; 4] = [7, 8, 9, 1];

/// Initializes the static values from the but board.
pub fn print_static_values() {
        println!("Full board {:#x}", get_full_board());
        println!("Bottom filler {:#x}", get_bottom_filler_mask());

        for i  in 0_u8..7_u8 {
            println!("Column mask {} : {:#x}", i, get_column_mask(i as u8));

        }

}

/// Slow method only to be used for board drawing, gets all elements from the boards as coordinates.
pub fn get_position_iterator(board: u64) -> impl Iterator<Item = (usize, usize)> {
    (0..7)
        .flat_map(|x| (0..6).map(move |y| (x, y)))
        .filter(move |&(x, y)| board & (1 << (x + 8 * y)) != 0)
}


pub fn get_possible_move(board : u64, column : usize) -> u64 {
    // Safely upshifted board extended with a bottom row.
    ((((board << DIR_INCREMENT[1]) & FULL_BOARD_MASK) | BOTTOM_FILL_MASK) ^
        // The original board.
        board )
        // Filter out the desired column.
        & COLUMN_MASK[column]
}

/// Gets an iterator for all possible moves for the ai.
pub fn get_all_possible_moves(board : u64) -> impl Iterator<Item = u64> {
    let comb = ((((board << DIR_INCREMENT[1]) & FULL_BOARD_MASK) | BOTTOM_FILL_MASK) ^
        board );
    (0..7).into_iter().map( move |x| comb & COLUMN_MASK[x] ).filter(|&x| x != 0)
}

/// Use only for interface stuff. Returns for an indicated move, the x,y coordinate where it will wind up.
pub fn get_move_information(coded_move : u64) -> Option<(usize, usize)> {
    for x in 0..7 {
        for y in 0..6 {
            if coded_move & (1 << (x + 8 * y)) > 0 {
                return Some((x, y));
            }
        }
    }
    None
}
