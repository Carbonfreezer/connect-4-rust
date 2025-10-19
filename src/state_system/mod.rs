//! At the highest level this game is a state machine. These states get reflected in this module.
//! Every new implemented state has to implement the trait *game_state::GameState*. It has to be added
//! into the function *game_state::generate_state_collection* and needs to get
//! a corresponding index in *game_state::GameStateIndex*, that it cen be referred to from other states.
//!
//! We have 5 states:
//! 1. The player select state, where the player can choose when to start.
//! 2. The computer execution state, where a determined move gets executed.
//! 3. The player input state. Input is processed here and also the animation is shown, when this would end ending the game.
//! 4. The player stone animation and computer move calculation state. This is where the AI computation happens.
//! The calculation gets spawned in a Tokio tak as soon as the animation starts.
//! 5. The game end state, that shows the game situation and asks for a confirmation button to start over.
//!
//! Transitions are
//! * 1->2 : If player chooses to be second, the move into the central slot gets executed on behalf of the computer.
//! * 1->3 : When the player chooses to start, we wind up here.
//! * 2->3: When the computer move is executed (animation) and the game end is not reached we go to player input.
//! * 3->4: When the player has made the input and the input does not result in ending the game, we go over to 4.
//! * 3->5: When the player input would result in ending the game, the animation is still played and then the transfer happens.
//! * 5->1: When the player has acknowledged the result, we go to selection again.

pub mod computer_move_execution_state;
pub mod game_over_state;
pub mod game_state;
pub mod player_input_state;
pub mod player_start_selection;
pub mod test_state;
