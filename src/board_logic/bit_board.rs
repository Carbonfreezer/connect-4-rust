//! This module contains the game board represented as a bit board.

use crate::board_logic::bit_board_coding::{
    BOARD_HEIGHT, BOARD_WIDTH, DIR_INCREMENT, FULL_BOARD_MASK, check_for_winning, clip_shift,
    get_all_possible_moves, get_bit_representation, get_column_mask, get_winning_board,
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
    pub own_stones: u64,
    pub opponent_stones: u64,
    // The boards represents from the perspective of the computer in default.
    computer_first: bool,
}

/// This is the symmetry independent coding that can be used for the transposition table.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct SymmetryIndependentPosition {
    pub own: u64,
    pub opp: u64,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
            own_stones: 0,
            opponent_stones: 0,
            computer_first: false,
        }
    }

    /// Resets the board at the end of the game.
    pub fn reset(&mut self) {
        self.own_stones = 0;
        self.opponent_stones = 0;
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
    pub fn get_board_positioning(&self) -> impl Iterator<Item = (u32, u32, bool)> {
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
    pub fn get_possible_move(&self, column: u32) -> u64 {
        debug_check_board_coordinates!(col: column);
        get_possible_move(self.own_stones | self.opponent_stones, column)
    }

    /// Gets the destination height for a move. This is the slot number,
    /// where the move will wind up. The method is slow and only be intended to be used
    /// for rendering purposes. Returns none of the move is not possible.
    pub fn get_move_destination(&self, column: u32) -> Option<u32> {
        debug_check_board_coordinates!(col: column);
        let move_spot = get_possible_move(self.own_stones | self.opponent_stones, column);
        (0..BOARD_HEIGHT).find(|&y| move_spot & get_bit_representation(column, y) != 0)
    }

    /// Simplifies making a move on a column on the outside. It has to be guarantied that move is possible.
    /// This function is meant for UI only and not the AI.
    pub fn apply_move_on_column(&mut self, column: u32, is_computer: bool) {
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

    /// Checks if we have a draw situation under the assumption that we do not have a winning
    /// one.
    #[inline(always)]
    pub fn check_for_draw_if_not_winning(&self) -> bool {
        let compound = self.opponent_stones | self.own_stones;
        compound == FULL_BOARD_MASK
    }

    /// Gets an iterator of all possible moves. This method is meant for the ai.
    /// The iterator returns the move and the original move index.
    #[inline(always)]
    pub fn get_all_possible_moves(&self) -> impl Iterator<Item = (u64, u32)> {
        get_all_possible_moves(self.opponent_stones | self.own_stones)
    }

    /// Easy game over method to be used for the game state system to determine the follow-up states.
    pub fn is_game_over(&self) -> bool {
        self.check_for_draw_if_not_winning()
            || check_for_winning(self.opponent_stones)
            || check_for_winning(self.own_stones)
    }

    /// Analyzes the winning condition for the game board to be used in combination with the user interface
    /// system. It returns the situation and if one party has won, it returns the stone coordinates of the
    /// stones generating four stones. This can eventually be more than one into one direction. &
    pub fn get_winning_status_for_rendering(&self) -> (GameResult, Option<Vec<(u32, u32)>>) {
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
                Some(get_position_iterator(get_winning_board(first_board)).collect()),
            )
        } else if check_for_winning(second_board) {
            (
                GameResult::SecondPlayerWon,
                Some(get_position_iterator(get_winning_board(second_board)).collect()),
            )
        } else if self.check_for_draw_if_not_winning() {
            (GameResult::Draw, None)
        } else {
            (GameResult::Pending, None)
        }
    }

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
    /// and two lines away from the center line and multiplies it with a scoring and adds it up..
    fn get_board_scoring(board: u64) -> f32 {
        let center = (board & Self::BOARD_EVALUATION_MASK[0]).count_ones() as f32 * 0.015;
        let one_off_center = (board & Self::BOARD_EVALUATION_MASK[1]).count_ones() as f32 * 0.07;
        let two_off_center = (board & Self::BOARD_EVALUATION_MASK[2]).count_ones() as f32 * 0.03;

        center + one_off_center + two_off_center
    }

    /// Does the complete heuristic evaluation of the game board.
    pub fn compute_heuristics(&self, clamp_guard : f32) -> f32 {
        debug_assert!(
            !self.is_game_over(),
            "The game over state should have already been prechecked."
        );

        let free_spots = !(self.opponent_stones | self.own_stones) & FULL_BOARD_MASK;
        let mut score = 0.0;

        // 1. Pairing combination
        let (doublets, open_three) =
            Self::count_open_three_and_doubles(self.own_stones, free_spots);
        score += open_three as f32 * 0.04;
        score += doublets as f32 * 0.01;
        let (doublets, open_three) =
            Self::count_open_three_and_doubles(self.opponent_stones, free_spots);
        score -= open_three as f32 * 0.04;
        score -= doublets as f32 * 0.01;

        // 2. board control.
        score += Self::get_board_scoring(self.own_stones);
        score -= Self::get_board_scoring(self.opponent_stones);

        // We do not clamp against exactly one, so that whatever the outcome is,
        // it will always be dominated by a guaranteed win or loss.
        score.clamp(-clamp_guard, clamp_guard)
    }
}
