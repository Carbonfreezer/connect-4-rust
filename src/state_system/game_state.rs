//! This module contains the trait of all states and contains a blackboard,
//! over which states can exchange information.

use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use crate::board_logic::bit_board::BitBoard;
use crate::state_computer_move_execution::StateComputerMoveExecution;
use crate::state_game_over::StateGameOver;
use crate::state_player_start_selection::StatePlayerStartSelection;
use crate::state_system::state_computer_calculation::StateComputerCalculation;
use crate::state_system::state_player_input::StatePlayerInput;

/// All implemented game states get an index, with which they can refer to each other.
pub enum GameStateIndex {
    StartSelection = 0,
    ComputerExecutionState = 1,
    PlayerInputState = 2,
    ComputerCalculationState = 3,
    GameOverState = 4,
}

/// Generates a vector with all the required game states.
pub fn generate_state_collection() -> Vec<Box<dyn GameState>> {
    let result: Vec<Box<dyn GameState>> = vec![
        Box::new(StatePlayerStartSelection::new()),
        Box::new(StateComputerMoveExecution::new()),
        Box::new(StatePlayerInput::new()),
        Box::new(StateComputerCalculation::new()),
        Box::new(StateGameOver::new()),
    ];
    result
}

/// A helper structure that is used by game states to exchange information.
pub struct Blackboard {
    /// The general board, that show the current game.
    pub game_board: BitBoard,
    /// When the information of a computer choice has to be carried over, it is done here.
    pub computer_choice: u32,
    /// Here comes the choice of the player.
    pub player_choice: u32,
    /// The board texture we use.
    pub board_texture: Texture2D,
}

impl Blackboard {
    pub fn new(texture : Texture2D) -> Blackboard {
        Blackboard {
            game_board: BitBoard::new(),
            computer_choice: 0,
            player_choice: 0,
            board_texture: texture,
        }
    }
}

/// A general interface for a game state, to administrate the different phases we can be in.
/// A general interface for a game state, to administrate the different phases we can be in.
pub trait GameState {
    /// Performs initialization when entering the game state. Data may be read out from the blackboard here.
    fn enter(&mut self, black_board: &Blackboard);

    /// Updates the game state with the passed time and returns a new game state when required.
    /// May read and update the blackboard.
    fn update(&mut self, delta_time: f32, black_board: &mut Blackboard) -> Option<GameStateIndex>;

    /// Informs the game state when a mouse has been clicked with the position.
    /// The blackboard is not handed over intentionally. Mouse interaction information
    /// should be stored in struct and processed in the update method. We have done this to avoid
    /// common state confusion errors.
    fn mouse_click(&mut self, position: Vec2);

    /// The rendering of the screen, it may read information
    /// from the black-board.
    fn draw(&self, black_board: &Blackboard);
}
