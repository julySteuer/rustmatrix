use core::panic;

use super::delays;

pub trait SimState {
    fn delay(&mut self);
    fn should_stop(&mut self) -> bool;
}

pub struct DebugState {
    steps: usize
}

pub struct GameState {

}

impl DebugState {
    fn new() -> DebugState {
        DebugState { steps: 0 }
    }
}

impl GameState {
    fn new() -> GameState {
        GameState {}
    }
}

impl SimState for DebugState {
    fn delay(&mut self) {
        delays::wait(delays::one_second);
    }
    
    fn should_stop(&mut self) -> bool {
        self.steps += 1;
        self.steps > 20
    }
}

impl SimState for GameState {
    fn delay(&mut self) {
        delays::wait(delays::ten_millis)
    }
    
    fn should_stop(&mut self) -> bool {
        false // Extern termination 
    }
}

pub fn get_sim_state_by_name(name: &str) -> Box<dyn SimState> {
    match name {
        "Debug" => Box::new(DebugState::new()),
        "Game" => Box::new(GameState::new()),
        _ => panic!("Game State was not found")
    }
}