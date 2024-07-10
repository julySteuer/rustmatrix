use std::collections::LinkedList;
use crate::renderer::{RenderableSymbol, Renderer};
use crate::helper::character_func::get_random_character;

mod config {
    pub const MAX_HEIGHT: usize = 10;
}

struct Pixel {
    pub intensity: usize,
    pub symbol: char,
}

#[derive(PartialEq)]
pub enum SnakeState {
    GROWING,
    LIVING,
    DECAYING,
    DEAD
}

pub struct Snake {
    position: (usize, usize),
    pixels: LinkedList<Pixel>,
    state: SnakeState
}

impl Pixel {
    fn new() -> Pixel {
        Pixel {intensity:3, symbol: get_random_character()} // Hardcoded for now Will add settings later
    }
}

impl Snake {
    pub fn new(start_position: (usize, usize)) -> Snake {
        Snake { position: start_position,pixels: LinkedList::new(), state: SnakeState::GROWING }
    }
    // TODO: Fix error why does the snake reproduce after MAX_HEIGHT
    pub fn update(&mut self) {
        if self.state != SnakeState::DECAYING {
            self.pixels.push_back(Pixel::new());
        }
        if self.state == SnakeState::GROWING {
            if self.pixels.len() == 3 {
                self.state = SnakeState::LIVING;
            }
            return;
        }
        if self.state == SnakeState::LIVING || self.state == SnakeState::DECAYING {
            self.pixels.pop_front();
            self.position.1 += 1;
            if self.position.1 >= config::MAX_HEIGHT {
                self.state = SnakeState::DECAYING;
            }
            if self.pixels.len() == 0 {
                self.state = SnakeState::DEAD;
            }
        }
    }

    pub fn should_delete(&self) -> bool {
        self.state == SnakeState::DEAD
    }

    pub fn render(&self, renderer: &Box<dyn Renderer>) {
        for (i, pixel) in self.pixels.iter().enumerate() {
            let renderbale = RenderableSymbol::new((self.position.0, self.position.1+i), pixel.intensity, pixel.symbol);
            renderer.render(renderbale);
        }
    }
}