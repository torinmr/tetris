use std::io;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::block::Title;

use tetromino::{PositionedTetromino, Tetromino};

mod tetromino;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 20;

#[derive(Debug, PartialEq)]
pub struct Game {
    debug_msg: String,
    settled_pieces: [[Cell; WIDTH as usize]; HEIGHT as usize],
    active_piece: PositionedTetromino,
}

impl Game {
    pub fn new() -> Self {
        Self {
            debug_msg: String::from("Welcome to Tetris!"),
            settled_pieces: [[Cell::Empty; WIDTH as usize]; HEIGHT as usize],
            active_piece: PositionedTetromino::new(Tetromino::T(0), 5, 5),
        }
    }

    fn tick(&mut self, command: Command, _tick_rate: &Duration) {
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
    }

    fn render_text(&self) -> String {
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
}

#[derive(Copy, Debug, PartialEq, Clone)]
enum Cell {
    Empty,
    ActiveBlock,
    InactiveBlock,
}

fn render<B: Backend>(f: &mut Frame<B>, game: &Game) {
    let board_width = (WIDTH * 2) as u16;
    let board_width_with_border = board_width + 2;
    let board_height = HEIGHT as u16;
    let board_height_with_border = board_height + 2;
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(board_height_with_border),
            Constraint::Length(1)
        ].as_ref())
        .split(f.size());

    let msg_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(board_width),
            Constraint::Length(1)
        ].as_ref())
        .split(main_layout[1]);
    let msg = Block::default()
        .title(Title::from(game.debug_msg.clone())
            .alignment(Alignment::Center));
    f.render_widget(msg, msg_layout[1]);

    let game_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(board_width_with_border),
            Constraint::Min(0),
        ].as_ref())
        .split(main_layout[0]);
    let paragraph = Paragraph::new(game.render_text())
        .block(Block::default()
            .borders(Borders::ALL));
    f.render_widget(paragraph, game_area[0]);
}

#[derive(Debug, PartialEq)]
enum Command {
    Left,
    Right,
    Drop,
    Up,
    CounterClockwise,
    Clockwise,
    ChangePiece,
    NoOp,
}

pub fn run_game<B: Backend>(
    terminal: &mut Terminal<B>,
    mut game: Game,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut command = Command::NoOp;
    loop {
        terminal.draw(|f| render(f, &game))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let new_command = match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Char('\'') => return Ok(()),
                    KeyCode::Char('h') => Command::Left,
                    KeyCode::Char('n') => Command::Right,
                    KeyCode::Char('t') => Command::Drop,
                    KeyCode::Char('c') => Command::Up,
                    KeyCode::Char(';') => Command::CounterClockwise,
                    KeyCode::Char('j') => Command::Clockwise,
                    KeyCode::Char(',') => Command::ChangePiece,
                    _ => Command::NoOp,
                };
                if new_command != Command::NoOp {
                    command = new_command;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            game.tick(command, &tick_rate);
            last_tick = Instant::now();
            command = Command::NoOp;
        }
    }
}
