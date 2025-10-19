
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


/// The width of the board.
pub const BOARD_WIDTH: usize = 7;

/// The height of the board.
pub const BOARD_HEIGHT : usize = 6;



/// Here wee define a couple of masks that are helpful.
const fn get_bit_representation(x: usize, y: usize) -> u64 {
    1 << (x + 8 * y)
}

const fn get_column_mask(col: usize) -> u64 {
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
pub fn  flip_board(input : u64) -> u64
{
    let mut result : u64;
    result =  (input&COLUMN_MASK[6]) >> 6;
    result |= (input&COLUMN_MASK[5]) >> 4;
    result |= (input&COLUMN_MASK[4]) >> 2;
    result |= input&COLUMN_MASK[3];
    result |= (input&COLUMN_MASK[2]) << 2;
    result |= (input&COLUMN_MASK[1]) << 4;
    result |= (input&COLUMN_MASK[0]) << 6;

    return result;
}
/// Slow method only to be used for board drawing, gets all elements from the boards as coordinates.
pub fn get_position_iterator(board: u64) -> impl Iterator<Item = (usize, usize)> {
    (0..BOARD_WIDTH)
        .flat_map(|x| (0..BOARD_HEIGHT).map(move |y| (x, y)))
        .filter(move |&(x, y)| board & (1 << (x + 8 * y)) != 0)
}

/// Gets a  representation, where the bit for the specific column is set where a move would wind up.
/// If it is not possible to make move in that column, a 0 is returned.
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
    let comb = (((board << DIR_INCREMENT[1]) & FULL_BOARD_MASK) | BOTTOM_FILL_MASK) ^
        board ;
    (0..BOARD_WIDTH).into_iter().map( move |x| comb & COLUMN_MASK[x] ).filter(|&x| x != 0)
}

/// Use only for interface stuff. Returns for an indicated move, the x,y coordinate where it will wind up.
pub fn get_move_information(coded_move : u64) -> Option<(usize, usize)> {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            if coded_move & (1 << (x + 8 * y)) > 0 {
                return Some((x, y));
            }
        }
    }
    None
}
