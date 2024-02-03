mod connect_four;

use std::{
    fmt::Display,
    io::{self, Stdout, Write},
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
    looping: bool,
    stdout: Stdout,
}

impl Game {
    fn new() -> Self {
        Self {
            game: ConnectFour::new(),
            selected_column: 0,
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
        execute!(self.stdout, EnterAlternateScreen, Hide)?;
        self.render()
    }

    fn game_loop(&mut self) -> io::Result<()> {
        while self.looping {
            if let Event::Key(event) = event::read()? {
                self.handle_key_event(event)?;
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

                match self.game.get(row, column) {
                    None => write!(self.stdout, "{SPACE}")?,
                    Some(player) => self.render_player(player)?,
                }
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

    fn render_player(&mut self, player: Player) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}{}",
            SetForegroundColor(player.color()),
            PLAYER,
            ResetColor
        )
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> io::Result<()> {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::Left => self.handle_left()?,
            KeyCode::Right => self.handle_right()?,
            KeyCode::Enter | KeyCode::Char(' ') => self.handle_play()?,
            KeyCode::Char('r') => self.handle_restart()?,
            _ => {}
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.looping = false;
    }

    fn handle_left(&mut self) -> io::Result<()> {
        if self.game.over() {
            return Ok(());
        }

        self.move_left();
        self.render()
    }

    fn move_left(&mut self) {
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

    fn handle_right(&mut self) -> io::Result<()> {
        if self.game.over() {
            return Ok(());
        }

        self.move_right();
        self.render()
    }

    fn move_right(&mut self) {
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

    fn handle_play(&mut self) -> io::Result<()> {
        if self.game.over() {
            return Ok(());
        }

        self.game.play(self.selected_column);

        if !self.game.over() && self.game.is_column_full(self.selected_column) {
            self.move_right();
        }

        self.render()
    }

    fn handle_restart(&mut self) -> io::Result<()> {
        self.game.reset();
        self.selected_column = 0;
        self.render()
    }
}
