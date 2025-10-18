use crate::graphics;
use crate::graphics::GraphicsPainter;
use crate::start_state::StartState;

pub enum GameStateIndex {
    Start = 0
}



pub fn generate_state_collection() -> Vec<Box<dyn GameState>> {
    let result : Vec<Box<dyn GameState>>  = vec![Box::new(StartState::new())];
    result
}

/// A general interface for a game state, to administrate the different phases we can be in.
pub trait GameState
{
    /// Performs initialization when entering the game state.
    fn initialize(&mut self);

    /// Updates the game state with the passed time and returns a new game state when required.
    fn update(&mut self, delta_time: f32) -> Option<GameStateIndex>;

    /// Informs the game state when a mouse has been clicked with the position.
    fn mouse_click(&mut self, position : [f32; 2]);

    /// The rendering of the screen.
    fn draw(&mut self, graphics : &GraphicsPainter);
}