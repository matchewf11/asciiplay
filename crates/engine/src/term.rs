use crate::color::{Color, ColorPair};
use std::io::{self, Write};

const RESET: &str = "\x1B[0m";

pub struct Term;

impl Term {
    pub fn new() -> Self {
        Self
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap()
    }

    pub fn clear(&self) {
        print!("\x1B[2J");
    }

    pub fn home(&self) {
        print!("\x1B[H");
    }

    pub fn move_cursor(&self, line: usize, column: usize) {
        print!("\x1B[{line};{column}H");
    }

    pub fn write_str(&self, text: &str) {
        print!("{text}");
    }

    pub fn set_bg(&self, c: &Color) {
        print!("\x1B[{}m", c.bg_code());
    }

    pub fn set_fg(&self, c: &Color) {
        print!("\x1B[{}m", c.fg_code());
    }

    pub fn reset_color(&self) {
        print!("{RESET}");
    }
}

// impl Drop for Term {
//     fn drop(&mut self) {}
// }
