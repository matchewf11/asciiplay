mod term;
mod color;
mod game;

pub use term::Term;
pub use game::{Game, run, ExitCode};
pub use color::{Color, ColorPair};
