use crate::term::Term;
use std::{time::{Duration, Instant}, thread};

pub enum ExitCode {
    Running,
    Done,
}

pub trait Game {
    /// update game state given the input ctx
    fn update(&mut self, term: &Term) -> ExitCode;

    /// draw the current game state
    fn draw(&self, term: &Term);

    fn exit(&self, term: &Term);
}

pub fn run(mut game: impl Game, term: &Term, fps: f32) {
    let frame_time = Duration::from_secs_f32(1.0 / fps);
    loop {
        let start = Instant::now();

        match game.update(term) {
            ExitCode::Running => (),
            ExitCode::Done => {
                game.exit(term);
                return;
            }
        }

        term.clear();
        term.home();
        game.draw(term);
        term.flush();
        let elapsed = start.elapsed();
        if let Some(remain) = frame_time.checked_sub(elapsed) {
            thread::sleep(remain);
        }
    }
}
