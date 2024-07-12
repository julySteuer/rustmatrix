use crate::helper::character_func::get_random_character;
use crate::renderer::{RenderableSymbol, Renderer};
use std::collections::LinkedList;

mod config {
    pub const MAX_HEIGHT: usize = 10;
}

#[derive(Debug)]
struct Pixel {
    pub intensity: usize,
    pub symbol: char,
}

#[derive(PartialEq, Debug)]
pub enum SnakeState {
    GROWING,
    LIVING,
    DECAYING,
    DEAD,
}

#[derive(Debug)]
pub struct Snake {
    position: (usize, usize),
    pixels: LinkedList<Pixel>,
    state: SnakeState,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel {
            intensity: 3,
            symbol: get_random_character(),
        } // Hardcoded for now Will add settings later
    }
}

impl Snake {
    pub fn new(start_position: (usize, usize)) -> Snake {
        Snake {
            position: start_position,
            pixels: LinkedList::new(),
            state: SnakeState::GROWING,
        }
    }
    // TODO: Fix error why does the snake reproduce after MAX_HEIGHT
    pub fn update(&mut self) {
        match self.state {
            SnakeState::GROWING => {
                self.pixels.push_back(Pixel::new());
                if self.pixels.len() == 3 {
                    self.state = SnakeState::LIVING;
                }
            }
            SnakeState::LIVING | SnakeState::DECAYING => {
                if self.state == SnakeState::LIVING {
                    self.pixels.push_back(Pixel::new());
                }
                self.pixels.pop_front();
                self.position.1 += 1;
                if self.position.1 >= config::MAX_HEIGHT {
                    self.state = SnakeState::DECAYING;
                }
                if self.pixels.len() == 0 {
                    self.state = SnakeState::DEAD;
                }
            }
            SnakeState::DEAD => (),
        }
    }

    pub fn should_delete(&self) -> bool {
        self.state == SnakeState::DEAD
    }

    pub fn render(&self, renderer: &Box<dyn Renderer>) {
        for (i, pixel) in self.pixels.iter().enumerate() {
            let renderbale = RenderableSymbol::new(
                (self.position.0, self.position.1 + i),
                pixel.intensity,
                pixel.symbol,
            );
            renderer.render(renderbale);
        }
    }
}
