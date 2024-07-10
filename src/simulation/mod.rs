mod snake;
mod states;
use snake::Snake;

use crate::config::get_configuration;
use crate::renderer::renderers::{CursesRenderer, DebugRenderer};
use crate::helper::snake_func::{get_random_snake_position_in_bound, spawn_snake_in_random_interval};
use crate::renderer::{get_renderer_by_name, Renderer};

use self::states::get_sim_state_by_name;

mod delays {
    use std::{time, thread};

    pub const one_second: time::Duration = time::Duration::from_secs(1);
    pub const ten_millis: time::Duration = time::Duration::from_millis(10);
    pub fn wait(delay: time::Duration) {
        thread::sleep(delay);
    }
}

pub fn run_simulation() {
    let mut snakes: Vec<Snake> = Vec::new();
    let cfg = get_configuration();
    let renderer = get_renderer_by_name(&cfg.renderer);
    let mut state = get_sim_state_by_name(&cfg.state);

    while !state.should_stop() {
        renderer.clear_scr(); 
        if spawn_snake_in_random_interval() {
            // add new snake
            let new_snake_pos = get_random_snake_position_in_bound((10,10)); // max_bounds Artificial will be set by renderer eventually 
            snakes.push(Snake::new(new_snake_pos));
        }

        for snake in &mut snakes { // update sycle 
            snake.update();
            snake.render(&renderer);
        }
        state.delay();
    }
}


