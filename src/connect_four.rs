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

        self.match_row(row, column);
        self.match_column(row, column);
        self.match_diagonal(row, column);
        self.match_alternate_diagonal(row, column);

        self.turns += 1;
        self.player = self.player.opponent();
    }

    fn match_indices(&mut self, indices: [usize; 4]) {
        let [p1, p2, p3, p4] = indices.map(|i| self.grid[i]);
        if p1.is_some() && p1 == p2 && p2 == p3 && p3 == p4 {
            self.winner = Some(self.player);
        }
    }

    fn match_row(&mut self, row: usize, column: usize) {
        let min_offset = 3 - column.min(3);
        let max_offset = (self.last_column() - column).min(3);
        for offset in min_offset..=max_offset {
            let column = column + offset;
            self.match_indices([
                self.get_index(row, column),
                self.get_index(row, column - 1),
                self.get_index(row, column - 2),
                self.get_index(row, column - 3),
            ]);
        }
    }

    fn match_column(&mut self, row: usize, column: usize) {
        let min_offset = 3 - row.min(3);
        let max_offset = (self.last_row() - row).min(3);
        for offset in min_offset..=max_offset {
            let row = row + offset;
            self.match_indices([
                self.get_index(row, column),
                self.get_index(row - 1, column),
                self.get_index(row - 2, column),
                self.get_index(row - 3, column),
            ]);
        }
    }

    fn match_diagonal(&mut self, row: usize, column: usize) {
        let min_offset = 3 - row.min(column).min(3);
        let max_offset = (self.last_row() - row)
            .min(self.last_column() - column)
            .min(3);

        for offset in min_offset..=max_offset {
            let row = row + offset;
            let column = column + offset;
            self.match_indices([
                self.get_index(row, column),
                self.get_index(row - 1, column - 1),
                self.get_index(row - 2, column - 2),
                self.get_index(row - 3, column - 3),
            ]);
        }
    }

    fn match_alternate_diagonal(&mut self, row: usize, column: usize) {
        let min_offset = 3 - row.min(self.last_column() - column).min(3);
        let max_offset = (self.last_row() - row).min(column).min(3);
        for offset in min_offset..=max_offset {
            let row = row + offset;
            let column = column - offset;
            self.match_indices([
                self.get_index(row, column),
                self.get_index(row - 1, column + 1),
                self.get_index(row - 2, column + 2),
                self.get_index(row - 3, column + 3),
            ]);
        }
    }

    pub fn reset(&mut self) {
        self.grid.fill(None);
        self.coins.fill(0);
        self.winner = None;
        self.turns = 0;
    }
}
