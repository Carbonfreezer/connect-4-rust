//! Contains all components related to board logic and ai.
//! This is the Negamax algorithm with alpha-beta pruning and
//! transposition table lookup. The game board representation is shown
//! as an efficient bitboard.

pub mod alpha_beta;
pub mod bit_board;
pub mod bit_board_coding;
mod heuristic;
