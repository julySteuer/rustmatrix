use self::renderers::{CursesRenderer, DebugRenderer};

pub mod renderers;

#[derive(Debug)]
pub struct RenderableSymbol {
    position: (usize, usize),
    intensity: usize,
    symbol: char
}

impl RenderableSymbol {
    pub fn new(position: (usize, usize), intensity: usize, symbol: char) -> RenderableSymbol {
        RenderableSymbol { position, intensity, symbol }
    }
}

pub trait Renderer {
    fn render(&self, sym: RenderableSymbol);
    fn clear_scr(&self);
}

pub fn get_renderer_by_name(name: &str) -> Box<dyn Renderer> {
    match name {
        "Debug" => Box::new(DebugRenderer::new()),
        "Curses" => Box::new(CursesRenderer::new()),
        _ => panic!("Renderer was not found")
    }
}