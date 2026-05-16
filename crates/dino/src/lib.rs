use engine::{ExitCode, Game, Term};

pub struct Dino;

impl Game for Dino {
    fn exit(&mut self, term: &Term) {
        todo!();
    }

    fn update(&mut self, term: &Term) -> ExitCode {
        todo!();
    }

    fn draw(&self, term: &Term) {
        todo!();
    }
}
