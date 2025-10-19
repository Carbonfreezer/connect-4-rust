//! This module contains the trait of all states and contains a blackboard,
//! over which states can exchange information.


use crate::bit_board::BitBoard;
use crate::player_start_selection::PlayerStartSelection;
use crate::render_system::graphics::GraphicsPainter;
use crate::test_state::TestState;

/// All implemented game states get an index, with which they can refer to each other.
pub enum GameStateIndex {
    Start = 0,
    StartSelection = 1,
}

/// Generates a vector with all the required game states.
pub fn generate_state_collection() -> Vec<Box<dyn GameState>> {
    let result: Vec<Box<dyn GameState>> = vec![
        Box::new(TestState::new()),
        Box::new(PlayerStartSelection::new()),
    ];
    result
}

/// A helper structure that is used by game states to exchange information.
pub struct Blackboard {
    pub game_board: BitBoard,
}

impl Blackboard {
    pub fn new() -> Blackboard {
        Blackboard {
            game_board: BitBoard::new(),
        }
    }
}

/// A general interface for a game state, to administrate the different phases we can be in.
pub trait GameState {
    /// Performs initialization when entering the game state. Data may be read out from the blackboard here.
    fn enter(&mut self, blackboard: &Blackboard);

    /// Updates the game state with the passed time and returns a new game state when required.
    /// May read and update the blackboard.
    fn update(&mut self, delta_time: f32, board: &mut Blackboard) -> Option<GameStateIndex>;

    /// Informs the game state when a mouse has been clicked with the position.
    /// The blackboard is not handed over intentionally. Mouse interaction information
    /// should be stored in struct and processed in the update method. We have done this to avoid
    /// common state confusion errors.
    fn mouse_click(&mut self, position: [f32; 2]);

    /// The rendering of the screen, requests a graphic painter to do so. It may read information
    /// from the black-board.
    fn draw(&self, graphics: &GraphicsPainter, board: &Blackboard);
}
