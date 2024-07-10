use simulation::run_simulation;

mod simulation;
pub mod helper;
pub mod renderer;
pub mod ffi;
pub mod config;

fn main() {
    run_simulation()
}
