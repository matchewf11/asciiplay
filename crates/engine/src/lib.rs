mod color;
mod game;
mod term;

pub use color::{Color, ColorPair};
pub use game::{ExitCode, Game, run};
pub use term::Term;
