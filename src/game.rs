use std::time::{Duration, Instant};

use crate::PositionedTetromino;
use crate::tetromino::Tetromino;

pub const HEIGHT: i32 = 20;
pub const WIDTH: i32 = 10;

pub type Board = [[Cell; WIDTH as usize]; HEIGHT as usize];
pub type NextPiece = [[Cell; 4]; 2];

pub const PIECE_START_Y: i32 = 1;
pub const PIECE_START_X: i32 = 5;
const INITIAL_DROP_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, PartialEq)]
pub struct Game {
    debug_msg: String,
    settled_pieces: Board,
    active_piece: Option<PositionedTetromino>,
    next_piece: Tetromino,
    last_drop: Instant,
    score: i32,
    drop_interval: Duration,
}

impl Game {
    pub fn new() -> Self {
        let settled_pieces = [[Cell::Empty; WIDTH as usize]; HEIGHT as usize];
        let first_piece = Tetromino::new(None);
        Self {
            debug_msg: String::from("Welcome to Tetris!"),
            settled_pieces,
            active_piece: PositionedTetromino::place(&first_piece, &settled_pieces),
            next_piece: Tetromino::new(Some(&first_piece)),
            last_drop: Instant::now(),
            score: 0,
            drop_interval: INITIAL_DROP_INTERVAL,
        }
    }

    pub fn tick(&mut self, command: Command) {
        if let Some(ref mut active_piece) = self.active_piece {
            match &command {
                Command::CounterClockwise => active_piece.rotate_ccw(&self.settled_pieces),
                Command::Clockwise => active_piece.rotate_cw(&self.settled_pieces),
                Command::Left => active_piece.move_left(&self.settled_pieces),
                Command::Right => active_piece.move_right(&self.settled_pieces),
                Command::Drop => active_piece.move_down(&self.settled_pieces),
                Command::Up => active_piece.move_up(&self.settled_pieces),
                Command::ChangePiece => active_piece.change(),
                _ => (),
            };

            if self.last_drop.elapsed() >= self.drop_interval {
                if active_piece.can_move_down(&self.settled_pieces) {
                    active_piece.move_down(&self.settled_pieces);
                } else {
                    for (y, x) in active_piece.get_coords() {
                        self.settled_pieces[y as usize][x as usize] = active_piece.get_cell_type();
                    }
                    self.clear_full_rows();

                    self.active_piece = PositionedTetromino::place(
                        &self.next_piece, &self.settled_pieces,
                    );
                    self.next_piece = Tetromino::new(Some(&self.next_piece));

                    if self.active_piece == None {
                        self.debug_msg = String::from("You lost!");
                    }
                }
                self.last_drop += self.drop_interval;
            }
        }
    }

    fn clear_full_rows(&mut self) {
        let mut num_cleared = 0;
        for y in (0..HEIGHT).rev() {
            loop {
                let mut row_full = true;
                for x in 0..WIDTH {
                    if self.settled_pieces[y as usize][x as usize] == Cell::Empty {
                        row_full = false;
                        break;
                    }
                }
                if row_full {
                    self.shift_rows_down(y);
                    num_cleared += 1;
                } else {
                    break;
                }
            }
        }

        match num_cleared {
            1 => {
                self.debug_msg = String::from("Good job!");
                self.increase_score(100);
            }
            2 => {
                self.debug_msg = String::from("Wow!");
                self.increase_score(300);
            }
            3 => {
                self.debug_msg = String::from("That's amazing!");
                self.increase_score(500);
            }
            4 => {
                self.debug_msg = String::from("TETRIS!!!!");
                self.increase_score(800);
            }
            _ => self.debug_msg = String::from("You can do it!")
        };
    }

    fn shift_rows_down(&mut self, cleared_y: i32) {
        for y in (0..cleared_y).rev() {
            for x in 0..WIDTH {
                self.settled_pieces[(y + 1) as usize][x as usize]
                    = self.settled_pieces[y as usize][x as usize];
            }
        }
        for x in 0..WIDTH {
            self.settled_pieces[0][x as usize] = Cell::Empty;
        }
    }

    fn increase_score(&mut self, points: i32) {
        let old_score = self.score;
        self.score += points;
        if self.score / 1000 > old_score / 1000 {
            self.drop_interval = (self.drop_interval * 8) / 10;
        }
    }

    pub fn render_board(&self) -> Board {
        let mut board = self.settled_pieces.clone();
        if let Some(ref active_piece) = self.active_piece {
            let mut preview_piece = active_piece.clone();
            while preview_piece.can_move_down(&self.settled_pieces) {
                preview_piece.move_down(&self.settled_pieces);
            }
            for (y, x) in preview_piece.get_coords() {
                board[y as usize][x as usize] = preview_piece.get_ghost_cell_type();
            }

            for (y, x) in active_piece.get_coords() {
                board[y as usize][x as usize] = active_piece.get_cell_type();
            }
        }
        board
    }

    pub fn render_next_piece(&self) -> [[Cell; 4]; 2] {
        let mut grid = [[Cell::Empty; 4]; 2];
        for (y, x) in self.next_piece.get_preview_coords() {
            grid[y as usize][x as usize] = self.next_piece.get_cell_type();
        }
        grid
    }

    pub fn render_message(&self) -> &str {
        self.debug_msg.as_str()
    }

    pub fn render_score(&self) -> i32 { self.score }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock,
    IGhostBlock,
    JGhostBlock,
    LGhostBlock,
    OGhostBlock,
    SGhostBlock,
    TGhostBlock,
    ZGhostBlock,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Left,
    Right,
    Drop,
    Up,
    CounterClockwise,
    Clockwise,
    ChangePiece,
    NoOp,
}
