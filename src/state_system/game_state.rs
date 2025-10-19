//! This module contains the trait of all states and contains a blackboard,
//! over which states can exchange information.

use crate::board_logic::bit_board::BitBoard;
use crate::computer_move_execution_state::ComputerMoveExecutionState;
use crate::game_over_state::GameOverState;
use crate::player_start_selection::PlayerStartSelection;
use crate::render_system::graphics::GraphicsPainter;
use crate::state_system::computer_calculation_state::ComputerCalculationState;
use crate::state_system::player_input_state::PlayerInputState;
use crate::test_state::TestState;

/// All implemented game states get an index, with which they can refer to each other.
pub enum GameStateIndex {
    TestState = 0,
    StartSelection = 1,
    ComputerExecutionState = 2,
    PlayerInputState = 3,
    ComputerCalculationState = 4,
    GameOverState = 5,
}

/// Generates a vector with all the required game states.
pub fn generate_state_collection() -> Vec<Box<dyn GameState>> {
    let result: Vec<Box<dyn GameState>> = vec![
        Box::new(TestState::new()),
        Box::new(PlayerStartSelection::new()),
        Box::new(ComputerMoveExecutionState::new()),
        Box::new(PlayerInputState::new()),
        Box::new(ComputerCalculationState::new()),
        Box::new(GameOverState::new()),
    ];
    result
}

/// A helper structure that is used by game states to exchange information.
pub struct Blackboard {
    /// The general board, that show the current game.
    pub game_board: BitBoard,
    /// When the information of a computer choice has to be carried over, it is done here.
    pub computer_choice: usize,
    /// Here comes the choice of the player.
    pub player_choice: usize,
}

impl Blackboard {
    pub fn new() -> Blackboard {
        Blackboard {
            game_board: BitBoard::new(),
            computer_choice: 0,
            player_choice: 0,
        }
    }
}

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
    fn mouse_click(&mut self, position: [f32; 2]);

    /// The rendering of the screen, requests a graphic painter to do so. It may read information
    /// from the black-board.
    fn draw(&self, graphics: &GraphicsPainter, black_board: &Blackboard);
}
