use std::time::{Duration, Instant};
use crate::{PositionedTetromino};

pub const HEIGHT: i32 = 20;
pub const WIDTH: i32 = 10;

pub const PIECE_START_Y: i32 = 1;
pub const PIECE_START_X: i32 = 5;
const DROP_INTERVAL: Duration = Duration::from_millis(750);

#[derive(Debug, PartialEq)]
pub struct Game {
    debug_msg: String,
    settled_pieces: [[Cell; WIDTH as usize]; HEIGHT as usize],
    active_piece: PositionedTetromino,
    last_drop: Instant,
}

impl Game {
    pub fn new() -> Self {
        Self {
            debug_msg: String::from("Welcome to Tetris!"),
            settled_pieces: [[Cell::Empty; WIDTH as usize]; HEIGHT as usize],
            active_piece: PositionedTetromino::spawn(),
            last_drop: Instant::now(),
        }
    }

    pub fn tick(&mut self, command: Command) {
        if command != Command::NoOp {
            self.debug_msg = match &command {
                Command::Left => String::from("Pressed left"),
                Command::Right => String::from("Pressed right"),
                Command::Drop => String::from("Pressed drop"),
                Command::Up => String::from("Pressed up"),
                Command::CounterClockwise => String::from("Pressed CCW"),
                Command::Clockwise => String::from("Pressed CW"),
                Command::ChangePiece => String::from("Pressed change piece"),
                _ => format!("Unknown command {command:?}")
            }
        }

        match &command {
            Command::CounterClockwise => self.active_piece.rotate_ccw(),
            Command::Clockwise => self.active_piece.rotate_cw(),
            Command::Left => self.active_piece.move_left(),
            Command::Right => self.active_piece.move_right(),
            Command::Drop => self.active_piece.move_down(),
            Command::Up => self.active_piece.move_up(),
            Command::ChangePiece => self.active_piece.change(),
            _ => (),
        };

        if self.last_drop.elapsed() >= DROP_INTERVAL {
            if self.active_piece.can_move_down() {
                self.active_piece.move_down();
            } else {
                for (y, x) in self.active_piece.get_coords() {
                    self.settled_pieces[y as usize][x as usize] = Cell::InactiveBlock;
                }
                self.active_piece = PositionedTetromino::spawn();
            }
            self.last_drop += DROP_INTERVAL;
        }
    }

    pub fn render_text(&self) -> String {
        let mut output = String::new();
        let mut board = self.settled_pieces.clone();
        for (y, x) in self.active_piece.get_coords() {
            board[y as usize][x as usize] = Cell::ActiveBlock;
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

#[derive(Clone, Debug, PartialEq)]
enum Cell {
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
