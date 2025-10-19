//! At the highest level this game is a state machine. These states get reflected in this module.
//! Every new implemented state has to implement the trait *game_state::GameState*. It has to be added 
//! into the function *game_state::generate_state_collection* and needs to get
//! a corresponding index in *game_state::GameStateIndex*, that it cen be referred to from other states,

pub mod game_state;
pub mod player_start_selection;
pub mod test_state;
