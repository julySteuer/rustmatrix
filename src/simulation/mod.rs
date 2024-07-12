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
    pub const five_hundred_millis: time::Duration = time::Duration::from_millis(500);
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
        let mut delete_list: Vec<usize> = Vec::new();

        renderer.clear_scr(); 
        if spawn_snake_in_random_interval() {
            // add new snake
            let new_snake_pos = get_random_snake_position_in_bound((10,10)); // max_bounds Artificial will be set by renderer eventually 
            snakes.push(Snake::new(new_snake_pos));
        }

        for (i, snake) in (&mut snakes).into_iter().enumerate() { // update sycle 
            snake.update();
            snake.render(&renderer);
            if snake.should_delete() {
                delete_list.push(i);
            }
        }
        
        delete_list.sort_by(|a, b| b.cmp(a));
        for to_be_deleted in delete_list { // Biggest one has to be removed first 
            snakes.remove(to_be_deleted);
        }
        state.delay();
    }
}


