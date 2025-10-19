//! This module contains the game board represented as a bit board.

use crate::board_logic::bit_board_coding::{
    BOARD_HEIGHT, BOARD_WIDTH, FULL_BOARD_MASK, check_for_winning, get_bit_representation,
    get_winning_board,
};
use crate::board_logic::bit_board_coding::{flip_board, get_position_iterator, get_possible_move};
use crate::debug_check_board_coordinates;
use std::hash::Hash;
use std::iter::Iterator;
use std::mem;

/// Encodes the game result needed for the drawing and state system.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameResult {
    Pending,
    Draw,
    FirstPlayerWon,
    SecondPlayerWon,
}

/// The bitboard has two representations for own and opponent stones.
#[derive(Clone)]
pub struct BitBoard {
    own_stones: u64,
    opponent_stones: u64,
    // The boards represents from the perspective of the computer in default.
    computer_first: bool,
}

impl BitBoard {
    /// Resets the board at the end of the game.
    pub fn reset(&mut self) {
        self.own_stones = 0;
        self.opponent_stones = 0;
    }
}

/// This is the symmetry independent coding that can be used for the transposition table.
#[derive(Hash, PartialEq, Eq, Clone)]
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

    /// Checks if the computer makes the first move.
    pub fn get_computer_first(&self) -> bool {
        self.computer_first
    }

    /// Swaps the players needed for the NEGAMAX algorithm.
    pub fn swap_players(&mut self) {
        mem::swap(&mut self.own_stones, &mut self.opponent_stones);
    }

    /// Returns a list of stones of positions and indications, if they are first player stones.
    /// This method is slow and to be used for rendering the board.
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
        debug_check_board_coordinates!(col: column);
        get_possible_move(self.own_stones | self.opponent_stones, column)
    }

    /// Gets the destination height for a move. This is the slot number,
    /// where the move will wind up. The method is slow and only be intended to be used
    /// for rendering purposes. Returns none of the move is not possible.
    pub fn get_move_destination(&self, column: usize) -> Option<usize> {
        debug_check_board_coordinates!(col: column);
        let move_spot = get_possible_move(self.own_stones | self.opponent_stones, column);
        for y in 0..BOARD_HEIGHT {
            if move_spot & get_bit_representation(column, y) != 0 {
                return Some(y);
            }
        }
        None
    }

    /// Simplifies making a move on a column on the outside. It has to be guarantied that move is possible.
    pub fn apply_move_on_column(&mut self, column: usize, is_computer: bool) {
        let coded_move = self.get_possible_move(column);
        debug_assert!(coded_move != 0, "The indicated move is not possible.");
        self.apply_move(coded_move, is_computer);
    }

    /// Applies an encoded move has handed out by the function *get_possible_move*.
    /// This function is meant to be used for UI interaction and not the AI.
    pub fn apply_move(&mut self, coded_move: u64, is_computer: bool) {
        if is_computer {
            self.own_stones |= coded_move;
        } else {
            self.opponent_stones |= coded_move;
        }
    }

    /// Revokes an encoded move has handed out by the function *get_possible_move*.
    /// This function is meant to be used for UI interaction and not the AI.
    /// It is used to precheck, if a move would wind result in an ending of the game.
    pub fn revoke_move(&mut self, coded_move: u64, is_computer: bool) {
        if is_computer {
            self.own_stones ^= coded_move;
        } else {
            self.opponent_stones ^= coded_move;
        }
    }

    /// The apply move as intended to be used in the ai, as this will always refer to the own stone.
    fn apply_move_own_stone(&mut self, coded_move: u64) {
        self.own_stones |= coded_move;
    }

    /// Revokes the move as used in the ai.
    fn revoke_move_own_stone(&mut self, coded_move: u64) {
        self.own_stones ^= coded_move;
    }

    /// Checks if we have a winning constellation for the opponent stone.
    /// This method is intended to be used for the AI, because after going into the recursion
    /// after means that the other player has possibly finished the game.
    pub fn check_winning_opponent(&self) -> bool {
        check_for_winning(self.opponent_stones)
    }

    /// Checks if we have a draw situation under the assumption that we do not have a winning
    /// one.
    pub fn check_for_draw_if_not_winning(&self) -> bool {
        let compound = self.opponent_stones | self.own_stones;
        compound == FULL_BOARD_MASK
    }

    /// Easy game over method to be used for the game state system to determine the follow-up states.
    pub fn is_game_over(&self) -> bool {
        self.check_for_draw_if_not_winning()
            || check_for_winning(self.opponent_stones)
            || check_for_winning(self.own_stones)
    }

    /// Analyzes the winning condition for the game board to be used in combination with the user interface
    /// system. It returns the situation and if one party has won, it returns the stone coordinates of the
    /// stones generating four stones. This can eventually be more than one into one direction.
    pub fn get_winning_status_for_rendering(&self) -> (GameResult, Option<Vec<(usize, usize)>>) {
        let first_board;
        let second_board;

        if self.computer_first {
            first_board = self.own_stones;
            second_board = self.opponent_stones;
        } else {
            first_board = self.opponent_stones;
            second_board = self.own_stones;
        }

        if check_for_winning(first_board) {
            (
                GameResult::FirstPlayerWon,
                Some(get_position_iterator(get_winning_board(first_board).unwrap()).collect()),
            )
        } else if check_for_winning(second_board) {
            (
                GameResult::SecondPlayerWon,
                Some(get_position_iterator(get_winning_board(second_board).unwrap()).collect()),
            )
        } else if self.check_for_draw_if_not_winning() {
            (GameResult::Draw, None)
        } else {
            (GameResult::Pending, None)
        }
    }
}
