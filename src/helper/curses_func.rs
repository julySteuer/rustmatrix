use crate::ffi::curses::{clear, endwin, getch, initscr, mvaddch, refresh, raw};

pub fn init_curses() { // Use this to init clolor and stuff 
    unsafe { 
        initscr();
        //  raw();
    }
}

pub fn render_character(position: (usize, usize), symbol: char) {
    unsafe {
        mvaddch(position.1 as i32, position.0 as i32, symbol.into());
        refresh();
    }
}

pub fn clear_all_characters() {
    unsafe {
        clear();
    }
}

pub fn close_curses() {
    unsafe {
        getch();
        endwin();
    }
}