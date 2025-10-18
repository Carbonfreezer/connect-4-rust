use crate::game_state::{Blackboard, GameState, GameStateIndex};
use crate::graphics;
use crate::graphics::{Color, GraphicsPainter};

pub struct StartState
{
    passed_time : f32,
}

impl StartState {
    pub fn new() -> StartState {
        StartState { passed_time : 0.0  }
    }
}

impl GameState for StartState {
    fn enter(&mut self, _board: &Blackboard) {
        self.passed_time = 0.0;
    }

    fn update(&mut self, delta_time: f32) -> Option<GameStateIndex> {
        self.passed_time += delta_time;
        if self.passed_time >= 1.0 {Some(GameStateIndex::Start)} else {None}
    }

    fn mouse_click(&mut self, position: [f32; 2]) {
        println!("Position {position:?}");
    }

    fn draw(&mut self, graphics : &GraphicsPainter) {

        if self.passed_time < 0.5 {
            graphics.draw_circle_normal(0.2, [0.0, 0.0], Color::Yellow);
        }
        else {

            graphics.draw_circle_into_stencil(0.05, [-0.75, -0.75]);
            graphics.draw_rectangle_conditional_stencil([-1.0, -1.0],[0.0, 0.0], Color::Brown);
        }

    }

    fn leave(&self, _blackboard: &mut Blackboard)  {

    }
}