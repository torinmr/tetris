use super::{HEIGHT, WIDTH};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tetromino {
    I(u8),
    J(u8),
    L(u8),
    O,
    S(u8),
    T(u8),
    Z(u8),
}

#[derive(Debug, PartialEq)]
pub struct PositionedTetromino {
    t: Tetromino,
    y: i32,
    x: i32,
}

impl PositionedTetromino {
    pub fn new(t: Tetromino, y: i32, x: i32) -> Self {
        Self { t, y, x }
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

    fn is_position_valid(&self) -> bool {
        for (y, x) in &self.get_coords() {
            if y < &0 || y >= &HEIGHT || x < &0 || x >= &WIDTH {
                return false;
            }
        }
        return true;
    }

    pub fn rotate_cw(&mut self) {
        let old_t = self.t;
        self.t = match self.t {
            Tetromino::I(r) => Tetromino::I((r + 1) % 2),
            Tetromino::J(r) => Tetromino::J((r + 1) % 4),
            Tetromino::L(r) => Tetromino::L((r + 1) % 4),
            Tetromino::O => Tetromino::O,
            Tetromino::S(r) => Tetromino::S((r + 1) % 2),
            Tetromino::T(r) => Tetromino::T((r + 1) % 4),
            Tetromino::Z(r) => Tetromino::Z((r + 1) % 2),
        };

        if !self.is_position_valid() {
            self.t = old_t;
        }
    }

    pub fn rotate_ccw(&mut self) {
        let old_t = self.t;
        self.t = match self.t {
            Tetromino::I(r) => Tetromino::I((r + 1) % 2),
            Tetromino::J(r) => Tetromino::J((r + 3) % 4),
            Tetromino::L(r) => Tetromino::L((r + 3) % 4),
            Tetromino::O => Tetromino::O,
            Tetromino::S(r) => Tetromino::S((r + 1) % 2),
            Tetromino::T(r) => Tetromino::T((r + 3) % 4),
            Tetromino::Z(r) => Tetromino::Z((r + 1) % 2),
        };

         if !self.is_position_valid() {
            self.t = old_t;
        }
    }

    pub fn move_down(&mut self) {
        self.y += 1;
        if !self.is_position_valid() {
            self.y -= 1;
        }
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
        if !self.is_position_valid() {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
        if !self.is_position_valid() {
            self.x += 1;
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
        if !self.is_position_valid() {
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
}