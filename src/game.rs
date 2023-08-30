use std::time::{Duration, Instant};

use crate::PositionedTetromino;

pub const HEIGHT: i32 = 20;
pub const WIDTH: i32 = 10;

pub type GridType = [[Cell; WIDTH as usize]; HEIGHT as usize];

pub const PIECE_START_Y: i32 = 1;
pub const PIECE_START_X: i32 = 5;
const DROP_INTERVAL: Duration = Duration::from_millis(750);

#[derive(Debug, PartialEq)]
pub struct Game {
    debug_msg: String,
    settled_pieces: GridType,
    active_piece: Option<PositionedTetromino>,
    last_drop: Instant,
}

impl Game {
    pub fn new() -> Self {
        let settled_pieces = [[Cell::Empty; WIDTH as usize]; HEIGHT as usize];
        Self {
            debug_msg: String::from("Welcome to Tetris!"),
            settled_pieces,
            active_piece: PositionedTetromino::spawn(&settled_pieces),
            last_drop: Instant::now(),
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

            if self.last_drop.elapsed() >= DROP_INTERVAL {
                if active_piece.can_move_down(&self.settled_pieces) {
                    active_piece.move_down(&self.settled_pieces);
                } else {
                    for (y, x) in active_piece.get_coords() {
                        self.settled_pieces[y as usize][x as usize] = Cell::InactiveBlock;
                    }
                    self.clear_full_rows();
                    self.active_piece = PositionedTetromino::spawn(&self.settled_pieces);
                    if self.active_piece == None {
                        self.debug_msg = String::from("You lost!");
                    }
                }
                self.last_drop += DROP_INTERVAL;
            }
        }
    }

    fn clear_full_rows(&mut self) {
        let mut num_cleared = 0;
        for y in (0..HEIGHT).rev() {
            loop {
                let mut row_full = true;
                for x in 0..WIDTH {
                    if self.settled_pieces[y as usize][x as usize] != Cell::InactiveBlock {
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
            1 => self.debug_msg = String::from("Good job!"),
            2 => self.debug_msg = String::from("Wow!"),
            3 => self.debug_msg = String::from("That's amazing!"),
            4 => self.debug_msg = String::from("TETRIS!!!!"),
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

    pub fn render_text(&self) -> String {
        let mut output = String::new();
        let mut board = self.settled_pieces.clone();
        if let Some(ref active_piece) = self.active_piece {
            for (y, x) in active_piece.get_coords() {
                board[y as usize][x as usize] = Cell::ActiveBlock;
            }
        }
        for row in board {
            for cell in row {
                match cell {
                    Cell::Empty => output.push_str("  "),
                    Cell::InactiveBlock => output.push_str("██"),
                    Cell::ActiveBlock => output.push_str("██"),
                }
            }
            output.push('\n');
        }
        output
    }


    pub fn get_message(&self) -> &str {
        self.debug_msg.as_str()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    ActiveBlock,
    InactiveBlock,
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
