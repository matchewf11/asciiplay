use engine::{Term, Game, run, ExitCode, Color, ColorPair};

const MENU_FPS: f32 = 5.0;

struct Menu<'a> {
    games: &'a [&'static str],
    selected: usize,
}

impl<'a> Menu<'a> {
    fn new(games: &'a [&'static str]) -> Self {
        Self { games, selected: 0 }
    }
}

pub fn start() {
    let m = Menu::new(&["foo", "bar", "jimmy", "john"]);
    let t = Term::new();
    run(m, &t, MENU_FPS);
}

impl<'a> Game for Menu<'a> {
    fn exit(&self, term: &Term) {
        todo!();
    }

    fn update(&mut self, term: &Term) -> ExitCode {
        ExitCode::Running
    }

    fn draw(&self, term: &Term) {
        for (i, game) in self.games.iter().enumerate() {
            let i = i + 1;

            // term.write_str_at_color(&format!(">"), i, 0, &ColorPair {
            //     bg: Color::Black,
            //     fg: Color::Red,
            // });

        }
    }
}
