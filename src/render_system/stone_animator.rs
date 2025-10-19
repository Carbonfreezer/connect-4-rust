//! This module is in charge of dropping stones into the game board.

use crate::bit_board::BitBoard;
use crate::bit_board_coding::BOARD_WIDTH;
use crate::{debug_check_board_coordinates, debug_check_draw_coordinates};
use crate::render_system::graphics::GraphicsPainter;

/// An animator that takes care on animating a stone into the drawing arena.
/// It can render itself and update itself and indicates if it is finished or not.
pub struct StoneAnimator {
    remaining_way_length: f32,
    current_position : [f32; 2],
    is_animating: bool,
    first_player: bool,
}

/// The velocity the stone falls down with.
const FALLING_VELOCITY : f32 = 2.0 / 0.5;

impl StoneAnimator {
    pub fn new() -> StoneAnimator {
        StoneAnimator {
            remaining_way_length : 0.0,
            is_animating: false,
            first_player : false,
            current_position: [0.0, 0.0],
        }
    }


    /// Starts animating a stone. Needs the board to find out where to go to in height, the column where to animate,
    /// and an indication if this is the first player to determine the color.
    pub fn start_animating(&mut self, board: &BitBoard, column : usize, is_computer: bool) {
        debug_assert_eq!(self.is_animating, false, "Cannot start animating while animating.");
        debug_check_board_coordinates!(col: column);
        self.first_player = is_computer == board.get_computer_first();
        let  height_chosen = board.get_move_destination(column).expect("The column handed over does not present a legal move.");
        self.current_position = GraphicsPainter::get_drawing_coordinates_above_column(column);
        let destination = GraphicsPainter::get_drawing_coordinates(column, height_chosen);
        debug_check_draw_coordinates!(self.current_position);
        debug_check_draw_coordinates!(destination);
        self.remaining_way_length = self.current_position[1] - destination[1];
        self.is_animating = true;
    }
    
    /// Draws the stone at the current position with the graphics painter handed over.
    pub fn draw(&self, graphics : &GraphicsPainter) {
        graphics.draw_stone_at_coordinates(self.current_position, self.first_player);
    }
    
    /// Updates the animation.
    pub fn update(&mut self, delta_time : f32)  {
        debug_assert!(self.is_animating, "Only update during animation.");
        let delta_way = - delta_time * FALLING_VELOCITY;
        self.remaining_way_length += delta_way; 
        self.is_animating = self.remaining_way_length > 0.0;
        self.current_position[1] += delta_way;
    }
    
    /// Stops the animation (typical on state leave). Is tolerant when it is already stopped
    /// anyway.
    pub fn stop_animating(&mut self) {
        self.is_animating = false;
    }
    
    /// Checks if we are still animating.
    pub fn is_animating(&self) -> bool {
        self.is_animating
    }
    
    
}