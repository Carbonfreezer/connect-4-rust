use crate::bit_board_coding::BOARD_WIDTH;
use crate::bit_board_coding::{flip_board, get_position_iterator, get_possible_move};
use crate::debug_check_coordinates;
use std::hash::Hash;
use std::iter::Iterator;
use std::mem;

#[derive(Clone)]
pub struct BitBoard {
    own_stones: u64,
    opponent_stones: u64,
    // The boards represents from the perspective of the computer in default.
    computer_first: bool,
}

/// This is the symmetry independent coding that can be used for the transposition table.
#[derive(Hash, PartialEq, Eq)]
pub struct SymmetryIndependentPosition {
    own: u64,
    opp: u64,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
            own_stones: 0,
            opponent_stones: 0,
            computer_first: false,
        }
    }

    /// Generates a structure that looks the same with its symmetrically identical board.
    /// This is meant to be used for the transposition table.  
    pub fn get_symmetry_independent_position(&self) -> SymmetryIndependentPosition {
        let flipped_own = flip_board(self.own_stones);
        let flipped_opp = flip_board(self.opponent_stones);

        // Sort lexicographically.
        if self.own_stones < flipped_own
            || (self.own_stones == flipped_own && self.opponent_stones < flipped_opp)
        {
            SymmetryIndependentPosition {
                own: self.own_stones,
                opp: self.opponent_stones,
            }
        } else {
            SymmetryIndependentPosition {
                own: flipped_own,
                opp: flipped_opp,
            }
        }
    }

    /// Gets adjusted from the outside to get the coloring right.
    pub fn set_computer_first(&mut self, is_first: bool) {
        self.computer_first = is_first;
    }
    pub fn swap_players(&mut self) {
        mem::swap(&mut self.own_stones, &mut self.opponent_stones);
    }

    /// Returns a list of stones of positions and indications, if they are first player stones.
    pub fn get_board_positioning(&self) -> impl Iterator<Item = (usize, usize, bool)> {
        let first_stones;
        let second_stones;
        if self.computer_first {
            first_stones = get_position_iterator(self.own_stones);
            second_stones = get_position_iterator(self.opponent_stones);
        } else {
            first_stones = get_position_iterator(self.opponent_stones);
            second_stones = get_position_iterator(self.own_stones);
        }

        first_stones
            .into_iter()
            .map(|(x, y)| (x, y, true))
            .chain(second_stones.into_iter().map(|(x, y)| (x, y, false)))
    }

    /// Gets in general a possible move for the board, Returns eiter 0 if column is full or returns
    /// the correctly set bit.
    pub fn get_possible_move(&self, column: usize) -> u64 {
        debug_check_coordinates!(col: column);
        get_possible_move(self.own_stones | self.opponent_stones, column)
    }

    /// Applies an encoded move has handed out by the function ['get_possible_move']
    pub fn apply_move(&mut self, coded_move: u64, is_computer: bool) {
        if is_computer {
            self.own_stones |= coded_move;
        } else {
            self.opponent_stones |= coded_move;
        }
    }

    /// The apply move as intended to be used in the ai, as this will always refer to the own stone.
    fn apply_move_active(&mut self, coded_move: u64) {
        self.own_stones |= coded_move;
    }

    /// Revokes the move as used in the ai.
    fn revoke_move_active(&mut self, coded_move: u64) {
        self.own_stones ^= coded_move;
    }
}
