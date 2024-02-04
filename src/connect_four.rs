#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    /// Returns the opponent of this player.
    pub fn opponent(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

const ROWS: usize = 6;
const COLUMNS: usize = 7;

pub struct ConnectFour {
    /// The game grid in row-major order.
    grid: [Option<Player>; ROWS * COLUMNS],

    /// The number of coins in each column.
    coins: [usize; COLUMNS],

    /// The current player.
    player: Player,

    /// The winner of the game.
    winner: Option<Player>,

    /// The number of turns that have passed.
    turns: usize,
}

impl ConnectFour {
    /// Creates a new game.
    pub fn new() -> Self {
        Self {
            grid: [None; ROWS * COLUMNS],
            coins: [0; COLUMNS],
            player: Player::One,
            winner: None,
            turns: 0,
        }
    }

    /// Returns the current player.
    pub fn player(&self) -> Player {
        self.player
    }

    /// Returns the winner of the game.
    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    /// Returns the number of rows in the grid.
    pub fn rows(&self) -> usize {
        ROWS
    }

    /// Returns the number of columns in the grid.
    pub fn columns(&self) -> usize {
        COLUMNS
    }

    /// Returns the size of the grid.
    pub fn size(&self) -> usize {
        self.grid.len()
    }

    /// Returns `true` if the game is over.
    pub fn over(&self) -> bool {
        self.winner.is_some() || self.turns == self.size()
    }

    /// Returns the last valid row in the grid.
    pub fn last_row(&self) -> usize {
        self.rows() - 1
    }

    /// Returns the last valid column in the grid.
    pub fn last_column(&self) -> usize {
        self.columns() - 1
    }

    /// Returns `true` if the given column is full.
    pub fn is_column_full(&self, column: usize) -> bool {
        self.coins[column] == self.rows()
    }

    /// Returns the index of the given position in the grid.
    fn get_index(&self, row: usize, column: usize) -> usize {
        assert!(
            row < self.rows() && column < self.columns(),
            "Position out of bounds: ({row}, {column})",
        );
        return self.columns() * row + column;
    }

    /// Returns the player at the given position.
    pub fn get(&self, row: usize, column: usize) -> Option<Player> {
        self.grid[self.get_index(row, column)]
    }

    /// Plays the game at the given column.
    pub fn play(&mut self, column: usize) {
        assert!(column < self.columns(), "Column out of bounds: {column}");
        assert!(!self.is_column_full(column), "Column is full: {column}");

        let row = self.last_row() - self.coins[column];
        let index = self.get_index(row, column);
        self.grid[index] = Some(self.player);
        self.coins[column] += 1;

        if self.match_row(row, column)
            || self.match_column(row, column)
            || self.match_diagonal(row, column)
            || self.match_alternate_diagonal(row, column)
        {
            self.winner = Some(self.player);
        }

        self.turns += 1;
        self.player = self.player.opponent();
    }

    fn match_players(
        &self,
        p1: Option<Player>,
        p2: Option<Player>,
        p3: Option<Player>,
        p4: Option<Player>,
    ) -> bool {
        p1.is_some() && p1 == p2 && p2 == p3 && p3 == p4
    }

    fn match_row(&self, row: usize, column: usize) -> bool {
        let min_offset = 3 - column.min(3);
        let max_offset = (self.last_column() - column).min(3);

        (min_offset..=max_offset).any(|offset| {
            let column = column + offset;
            self.match_players(
                self.get(row, column),
                self.get(row, column - 1),
                self.get(row, column - 2),
                self.get(row, column - 3),
            )
        })
    }

    fn match_column(&self, row: usize, column: usize) -> bool {
        let min_offset = 3 - row.min(3);
        let max_offset = (self.last_row() - row).min(3);

        (min_offset..=max_offset).any(|offset| {
            let row = row + offset;
            self.match_players(
                self.get(row, column),
                self.get(row - 1, column),
                self.get(row - 2, column),
                self.get(row - 3, column),
            )
        })
    }

    fn match_diagonal(&self, row: usize, column: usize) -> bool {
        let min_offset = 3 - row.min(column).min(3);
        let max_offset = (self.last_row() - row)
            .min(self.last_column() - column)
            .min(3);

        (min_offset..=max_offset).any(|offset| {
            let row = row + offset;
            let column = column + offset;
            self.match_players(
                self.get(row, column),
                self.get(row - 1, column - 1),
                self.get(row - 2, column - 2),
                self.get(row - 3, column - 3),
            )
        })
    }

    fn match_alternate_diagonal(&self, row: usize, column: usize) -> bool {
        let min_offset = 3 - row.min(self.last_column() - column).min(3);
        let max_offset = (self.last_row() - row).min(column).min(3);

        (min_offset..=max_offset).any(|offset| {
            let row = row + offset;
            let column = column - offset;
            self.match_players(
                self.get(row, column),
                self.get(row - 1, column + 1),
                self.get(row - 2, column + 2),
                self.get(row - 3, column + 3),
            )
        })
    }

    pub fn reset(&mut self) {
        self.grid.fill(None);
        self.coins.fill(0);
        self.winner = None;
        self.turns = 0;
    }
}
