use rand::seq::SliceRandom;

use crate::{HEIGHT, WIDTH};
use crate::game::{Board, Cell, PIECE_START_X, PIECE_START_Y};

#[derive(Debug, PartialEq, Clone)]
pub enum Tetromino {
    I(u8),
    J(u8),
    L(u8),
    O,
    S(u8),
    T(u8),
    Z(u8),
}

impl Tetromino {
    pub fn new(prev_piece: Option<&Self>) -> Self {
        let choices = vec![
            Tetromino::I(0),
            Tetromino::J(0),
            Tetromino::L(0),
            Tetromino::O,
            Tetromino::S(0),
            Tetromino::T(0),
            Tetromino::Z(0),
        ];
        let mut choice = choices.choose(&mut rand::thread_rng()).unwrap().clone();
        match prev_piece {
            None => choice,
            Some(prev) => {
                while *prev == choice {
                    choice = choices.choose(&mut rand::thread_rng()).unwrap().clone();
                }
                choice
            }
        }
    }

    // Unrotated pieces, shifted so they fit in a 4/2 grid.
    pub fn get_preview_coords(&self) -> Vec<(i32, i32)> {
        match self {
            Tetromino::I(_) => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Tetromino::J(_) => vec![(0, 0), (1, 0), (1, 1), (1, 2)],
            Tetromino::L(_) => vec![(1, 0), (1, 1), (1, 2), (0, 2)],
            Tetromino::O => vec![(0, 1), (1, 1), (0, 2), (1, 2)],
            Tetromino::Z(_) => vec![(0, 0), (0, 1), (1, 1), (1, 2)],
            Tetromino::T(_) => vec![(0, 1), (1, 1), (1, 0), (1, 2)],
            Tetromino::S(_) => vec![(1, 0), (1, 1), (0, 1), (0, 2)],
        }
    }

    pub fn get_cell_type(&self) -> Cell {
        match self {
            Tetromino::I(_) => Cell::IBlock,
            Tetromino::J(_) => Cell::JBlock,
            Tetromino::L(_) => Cell::LBlock,
            Tetromino::O => Cell::OBlock,
            Tetromino::S(_) => Cell::SBlock,
            Tetromino::T(_) => Cell::TBlock,
            Tetromino::Z(_) => Cell::ZBlock,
        }
    }

     pub fn get_ghost_cell_type(&self) -> Cell {
        match self {
            Tetromino::I(_) => Cell::IGhostBlock,
            Tetromino::J(_) => Cell::JGhostBlock,
            Tetromino::L(_) => Cell::LGhostBlock,
            Tetromino::O => Cell::OGhostBlock,
            Tetromino::S(_) => Cell::SGhostBlock,
            Tetromino::T(_) => Cell::TGhostBlock,
            Tetromino::Z(_) => Cell::ZGhostBlock,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PositionedTetromino {
    t: Tetromino,
    y: i32,
    x: i32,
}

impl PositionedTetromino {
    pub fn place(t: &Tetromino, board: &Board) -> Option<Self> {
        let mut piece = Self {
            t: t.clone(),
            y: PIECE_START_Y,
            x: PIECE_START_X,
        };
        piece.move_up(board);  // Ensure it's in the topmost position
        if piece.is_position_valid(board) {
            Some(piece)
        } else {
            None
        }
    }


    // Change type of tetromino - used just for debugging.
    pub fn change(&mut self) {
        self.t = match self.t {
            Tetromino::I(_) => Tetromino::J(0),
            Tetromino::J(_) => Tetromino::L(0),
            Tetromino::L(_) => Tetromino::O,
            Tetromino::O => Tetromino::S(0),
            Tetromino::S(_) => Tetromino::T(0),
            Tetromino::T(_) => Tetromino::Z(0),
            Tetromino::Z(_) => Tetromino::I(0),
        };
        self.y = 5;
        self.x = 5;
    }

    fn is_position_valid(&self, board: &Board) -> bool {
        for (y, x) in self.get_coords() {
            if y < 0 || y >= HEIGHT || x < 0 || x >= WIDTH
                || board[y as usize][x as usize] != Cell::Empty {
                return false;
            }
        }
        true
    }

    pub fn rotate_cw(&mut self, board: &Board) {
        let old_t = self.t.clone();
        self.t = match self.t {
            Tetromino::I(r) => Tetromino::I((r + 1) % 2),
            Tetromino::J(r) => Tetromino::J((r + 1) % 4),
            Tetromino::L(r) => Tetromino::L((r + 1) % 4),
            Tetromino::O => Tetromino::O,
            Tetromino::S(r) => Tetromino::S((r + 1) % 2),
            Tetromino::T(r) => Tetromino::T((r + 1) % 4),
            Tetromino::Z(r) => Tetromino::Z((r + 1) % 2),
        };

        if !self.is_position_valid(board) {
            self.t = old_t;
        }
    }

    pub fn rotate_ccw(&mut self, board: &Board) {
        let old_t = self.t.clone();
        self.t = match self.t {
            Tetromino::I(r) => Tetromino::I((r + 1) % 2),
            Tetromino::J(r) => Tetromino::J((r + 3) % 4),
            Tetromino::L(r) => Tetromino::L((r + 3) % 4),
            Tetromino::O => Tetromino::O,
            Tetromino::S(r) => Tetromino::S((r + 1) % 2),
            Tetromino::T(r) => Tetromino::T((r + 3) % 4),
            Tetromino::Z(r) => Tetromino::Z((r + 1) % 2),
        };

        if !self.is_position_valid(board) {
            self.t = old_t;
        }
    }

    pub fn move_down(&mut self, board: &Board) {
        self.y += 1;
        if !self.is_position_valid(board) {
            self.y -= 1;
        }
    }

    pub fn can_move_down(&self, board: &Board) -> bool {
        let mut tmp = (*self).clone();
        tmp.y += 1;
        tmp.is_position_valid(board)
    }

    pub fn move_up(&mut self, board: &Board) {
        self.y -= 1;
        if !self.is_position_valid(board) {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self, board: &Board) {
        self.x -= 1;
        if !self.is_position_valid(board) {
            self.x += 1;
        }
    }

    pub fn move_right(&mut self, board: &Board) {
        self.x += 1;
        if !self.is_position_valid(board) {
            self.x -= 1;
        }
    }

    pub fn get_coords(&self) -> Vec<(i32, i32)> {
        let mut coords: Vec<(i32, i32)> = match self.t {
            Tetromino::I(0) => vec![(0, -1), (0, 0), (0, 1), (0, 2)],
            Tetromino::I(1) => vec![(-1, 0), (0, 0), (1, 0), (2, 0)],
            Tetromino::J(0) => vec![(-1, -1), (0, -1), (0, 0), (0, 1)],
            Tetromino::J(1) => vec![(-1, 1), (-1, 0), (0, 0), (1, 0)],
            Tetromino::J(2) => vec![(1, 1), (0, 1), (0, 0), (0, -1)],
            Tetromino::J(3) => vec![(1, -1), (1, 0), (0, 0), (-1, 0)],
            Tetromino::L(0) => vec![(0, -1), (0, 0), (0, 1), (-1, 1)],
            Tetromino::L(1) => vec![(-1, 0), (0, 0), (1, 0), (1, 1)],
            Tetromino::L(2) => vec![(0, 1), (0, 0), (0, -1), (1, -1)],
            Tetromino::L(3) => vec![(1, 0), (0, 0), (-1, 0), (-1, -1)],
            Tetromino::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            Tetromino::Z(0) => vec![(0, -1), (0, 0), (1, 0), (1, 1)],
            Tetromino::Z(1) => vec![(-1, 0), (0, 0), (0, -1), (1, -1)],
            Tetromino::T(0) => vec![(-1, 0), (0, 0), (0, -1), (0, 1)],
            Tetromino::T(1) => vec![(-1, 0), (0, 0), (1, 0), (0, 1)],
            Tetromino::T(2) => vec![(1, 0), (0, 0), (0, -1), (0, 1)],
            Tetromino::T(3) => vec![(-1, 0), (0, 0), (1, 0), (0, -1)],
            Tetromino::S(0) => vec![(1, -1), (1, 0), (0, 0), (0, 1)],
            Tetromino::S(1) => vec![(-1, -1), (0, -1), (0, 0), (1, 0)],
            _ => panic!("Invalid Tetromino {self:?}"),
        };
        for (y, x) in &mut coords {
            *y += self.y;
            *x += self.x;
        }
        return coords;
    }

    pub fn get_cell_type(&self) -> Cell {
        self.t.get_cell_type()
    }

    pub fn get_ghost_cell_type(&self) -> Cell {
        self.t.get_ghost_cell_type()
    }
}
