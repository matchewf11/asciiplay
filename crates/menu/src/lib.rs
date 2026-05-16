use dino::Dino;
use engine::{Color, ColorPair, ExitCode, Game, Term, run};
use snake::Snake;

const FPS: f32 = 5.0;

struct Menu<'a> {
    games: &'a mut [(&'static str, &'a mut dyn Game)],
    selected: usize,
}

impl<'a> Menu<'a> {
    fn new(games: &'a mut [(&'static str, &'a mut dyn Game)]) -> Self {
        Self { games, selected: 1 }
    }
}

pub fn start() {
    let mut list: &mut [(&str, &mut dyn Game)] = &mut [("dino", &mut Dino), ("snake", &mut Snake)];
    let mut m = Menu::new(&mut list);
    let t = Term::new();
    run(&mut m, &t, FPS);
}

impl<'a> Game for Menu<'a> {
    fn exit(&mut self, term: &Term) {
        run(self.games[self.selected].1, term, FPS);
    }

    fn update(&mut self, term: &Term) -> ExitCode {
        ExitCode::Running
    }

    fn draw(&self, term: &Term) {
        for (i, (game, _)) in self.games.iter().enumerate() {
            let i = i + 1;

            term.move_cursor(i, 0);

            if i == self.selected {
                term.write_str("> ");
            }

            term.write_str(game);
        }
    }
}
