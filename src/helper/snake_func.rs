use rand::Rng;

pub fn get_random_snake_position_in_bound(max_bounds: (usize, usize)) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..max_bounds.0), rng.gen_range(0..max_bounds.1))
}

pub fn spawn_snake_in_random_interval() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..4) == 3 // Should generate one snake every 6 cycles
}