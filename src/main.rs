mod connect_four;

use std::{
    fmt::Display,
    io::{self, Write},
    time::{Duration, Instant},
};

use connect_four::{ConnectFour, Player};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

const SPACE: &str = " ";
const ARROW: &str = "▼";
const PLAYER: &str = "●";
const SEPARATOR: &str = "|";
const ANIMATION_DURATION: Duration = Duration::from_millis(100);

impl Player {
    fn color(&self) -> Color {
        match self {
            Player::One => Color::Red,
            Player::Two => Color::Yellow,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Player::One => "Player 1",
            Player::Two => "Player 2",
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

fn main() -> io::Result<()> {
    Game::new().run()
}

struct Game {
    game: ConnectFour,
    selected_column: usize,
    animation: Option<Animation>,
    looping: bool,
    stdout: io::Stdout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Animation {
    current_row: usize,
    target_row: usize,
    column: usize,
    start: Instant,
}

impl Game {
    fn new() -> Self {
        Self {
            game: ConnectFour::new(),
            selected_column: 0,
            animation: None,
            looping: true,
            stdout: io::stdout(),
        }
    }

    fn run(&mut self) -> io::Result<()> {
        self.setup()?;
        self.game_loop()?;
        self.teardown()
    }

    fn setup(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, Hide)
    }

    fn game_loop(&mut self) -> io::Result<()> {
        while self.looping {
            self.render()?;
            self.tick_animation();

            if !event::poll(Duration::from_millis(50))? {
                continue;
            }

            if let Event::Key(event) = event::read()? {
                self.handle_key_event(event);
            }
        }

        Ok(())
    }

    fn teardown(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen, Show)
    }

    fn render(&mut self) -> io::Result<()> {
        self.clear_terminal()?;
        self.render_message()?;
        self.render_arrow()?;

        for row in 0..self.game.rows() {
            for column in 0..self.game.columns() {
                write!(self.stdout, "{SEPARATOR}")?;

                if let Some(animation) = self.animation {
                    if animation.current_row == row && animation.column == column {
                        self.render_cell(self.game.get(animation.target_row, column))?;
                        continue;
                    }

                    if animation.target_row == row && animation.column == column {
                        self.render_empty_cell()?;
                        continue;
                    }
                }

                self.render_cell(self.game.get(row, column))?;
            }

            write!(self.stdout, "{SEPARATOR}\r\n")?;
        }

        Ok(())
    }

    fn clear_terminal(&mut self) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    fn render_message(&mut self) -> io::Result<()> {
        if !self.game.over() {
            return write!(
                self.stdout,
                "{}{}'s turn{}\r\n",
                SetForegroundColor(self.game.player().color()),
                self.game.player(),
                ResetColor
            );
        }

        if let Some(winner) = self.game.winner() {
            return write!(
                self.stdout,
                "{}{} won!{}\r\n",
                SetForegroundColor(winner.color()),
                winner,
                ResetColor,
            );
        }

        write!(self.stdout, "Draw!\r\n")
    }

    fn render_arrow(&mut self) -> io::Result<()> {
        let padding = SPACE.repeat(2 * self.selected_column + 1);
        write!(self.stdout, "{padding}{ARROW}\r\n")
    }

    fn render_cell(&mut self, cell: Option<Player>) -> io::Result<()> {
        match cell {
            None => self.render_empty_cell(),
            Some(player) => self.render_player(player),
        }
    }

    fn render_empty_cell(&mut self) -> io::Result<()> {
        write!(self.stdout, "{SPACE}")
    }

    fn render_player(&mut self, player: Player) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}{}",
            SetForegroundColor(player.color()),
            PLAYER,
            ResetColor
        )
    }

    fn tick_animation(&mut self) {
        if let Some(animation) = &mut self.animation {
            if animation.current_row == animation.target_row {
                self.animation = None;
                return;
            }

            if animation.start.elapsed() >= ANIMATION_DURATION {
                animation.current_row += 1;
                animation.start = Instant::now();
            }
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Enter | KeyCode::Char(' ') => self.handle_play(),
            KeyCode::Char('r') => self.handle_restart(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.looping = false;
    }

    fn move_left(&mut self) {
        if self.game.over() {
            return;
        }

        loop {
            if self.selected_column == 0 {
                self.selected_column = self.game.last_column();
            } else {
                self.selected_column -= 1;
            }

            if !self.game.is_column_full(self.selected_column) {
                break;
            }
        }
    }

    fn move_right(&mut self) {
        if self.game.over() {
            return;
        }

        loop {
            if self.selected_column == self.game.last_column() {
                self.selected_column = 0;
            } else {
                self.selected_column += 1;
            }

            if !self.game.is_column_full(self.selected_column) {
                break;
            }
        }
    }

    fn handle_play(&mut self) {
        if self.game.over() || self.animation.is_some() {
            return;
        }

        let column = self.selected_column;
        let row = self.game.play(column);

        if row != 0 {
            self.animation = Some(Animation {
                current_row: 0,
                target_row: row,
                column,
                start: Instant::now(),
            });
        }

        if self.game.is_column_full(column) {
            self.move_right();
        }
    }

    fn handle_restart(&mut self) {
        self.game.reset();
        self.selected_column = 0;
    }
}
