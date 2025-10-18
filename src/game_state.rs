use crate::graphics::GraphicsPainter;
use crate::start_state::StartState;
use crate::player_start_selection::PlayerStartSelection;

/// All implemented game states get an index, with which they can refer to each other.
pub enum GameStateIndex {
    Start = 0,
    StartSelection = 1,
}


/// Generates a vector with all the required game states.
pub fn generate_state_collection() -> Vec<Box<dyn GameState>> {
    let result : Vec<Box<dyn GameState>>  = vec![Box::new(StartState::new()), Box::new(PlayerStartSelection::new())];
    result
}

/// A helper structure that is used by game states to communicate.
#[derive(Default)]
pub struct Blackboard {
    pub is_computer_start_gamer : bool,
}

/// A general interface for a game state, to administrate the different phases we can be in.
pub trait GameState
{
    /// Performs initialization when entering the game state. Data may be read out from the blackboard here.
    fn enter(&mut self, blackboard: &Blackboard);

    /// Updates the game state with the passed time and returns a new game state when required.
    fn update(&mut self, delta_time: f32) -> Option<GameStateIndex>;

    /// Informs the game state when a mouse has been clicked with the position.
    fn mouse_click(&mut self, position : [f32; 2]);

    /// The rendering of the screen, requests a graphic painter to do so.
    fn draw(&mut self, graphics : &GraphicsPainter);
    
    /// Function that is called in the end where states can write their data into the blackboard.
    fn leave(&self, blackboard: &mut Blackboard);
}