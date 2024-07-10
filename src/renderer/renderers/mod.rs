/*
This Renderer just prints to stdout every bumfuck shit i want to so there wont be a whole lot of attributes
*/

use crate::{helper::curses_func::{clear_all_characters, close_curses, init_curses, render_character}, renderer::Renderer};

pub struct DebugRenderer {
    
}

impl DebugRenderer {
    pub fn new() -> DebugRenderer {
        DebugRenderer{}
    }
}

impl Renderer for DebugRenderer {
    fn render(&self, sym: crate::renderer::RenderableSymbol) {
        println!("{:?}", sym);
    }
    
    fn clear_scr(&self) {
        println!("---Clearing Screen---")
    }
}

/*
This is the main renderer using the terminal ncurses library 
*/
pub struct CursesRenderer {

}

impl CursesRenderer {
    pub fn new() -> CursesRenderer {
        // Due to renderers being treated as singletons the initialisation is done here
        init_curses();
        CursesRenderer{}
    }
}

impl Renderer for CursesRenderer {
    fn render(&self, sym: super::RenderableSymbol) {
        render_character(sym.position, sym.symbol)
    }
    
    fn clear_scr(&self) {
        clear_all_characters();
    }
}

impl Drop for CursesRenderer {
    fn drop(&mut self) {
        close_curses();
    }
}