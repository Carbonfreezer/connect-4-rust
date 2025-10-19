use crate::game_state::{Blackboard, GameState, GameStateIndex};
use crate::graphics::{ GraphicsPainter};

pub struct StartState
{
    passed_time : f32,
    slot_picked : Option<usize>
}

impl StartState {
    pub fn new() -> StartState {
        StartState { passed_time : 0.0, slot_picked : None  }
    }
}

impl GameState for StartState {
    fn enter(&mut self,_ : &Blackboard) {
        self.passed_time = 0.0;
    }

    fn update(&mut self, delta_time: f32, board : &mut Blackboard) -> Option<GameStateIndex> {
        if let Some(pos) = self.slot_picked {
            self.slot_picked = None;

            let mov = board.game_board.get_possible_move(pos);
            if mov != 0 {
                board.game_board.apply_move(mov, false);
            }
        }

        self.passed_time += delta_time;
        if self.passed_time >= 1.0 {Some(GameStateIndex::Start)} else {None}
    }

    fn mouse_click(&mut self, position: [f32; 2]) {
        let slot = ((position[0] + 1.0) * 3.5) as usize;
        self.slot_picked = Some(slot);
    }



    fn draw(&mut self, graphics : &GraphicsPainter, board : &Blackboard) {
        graphics.render_board(&board.game_board);

    }

}