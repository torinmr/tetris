use std::io;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::block::Title;

use crate::game::{Board, Cell, Command, Game, HEIGHT, NextPiece, WIDTH};
use crate::tetromino::PositionedTetromino;

mod tetromino;
pub mod game;

fn render_cell(cell: &Cell) -> Span<'static> {
    match cell {
        Cell::IBlock => Span::styled("██", Style::new().fg(Color::Rgb(49, 199, 239))),
        Cell::JBlock => Span::styled("██", Style::new().fg(Color::Rgb(90, 101, 173))),
        Cell::LBlock => Span::styled("██", Style::new().fg(Color::Rgb(239, 121, 33))),
        Cell::OBlock => Span::styled("██", Style::new().fg(Color::Rgb(247, 211, 8))),
        Cell::SBlock => Span::styled("██", Style::new().fg(Color::Rgb(72, 208, 72))),
        Cell::TBlock => Span::styled("██", Style::new().fg(Color::Rgb(173, 77, 156))),
        Cell::ZBlock => Span::styled("██", Style::new().fg(Color::Rgb(239, 32, 41))),
        Cell::IGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(49, 199, 239))),
        Cell::JGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(90, 101, 173))),
        Cell::LGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(239, 121, 33))),
        Cell::OGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(247, 211, 8))),
        Cell::SGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(72, 208, 72))),
        Cell::TGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(173, 77, 156))),
        Cell::ZGhostBlock => Span::styled("[]", Style::new().fg(Color::Rgb(239, 32, 41))),
        Cell::Empty => Span::raw("  "),
    }
}

fn render_board_to_text(board: Board) -> Vec<Line<'static, >> {
    board.iter().map(|row|
        Line::from(
            row.iter().map(|cell|
                render_cell(cell)
            ).collect::<Vec<Span>>()
        )
    ).collect()
}

fn render_next_piece_to_text(piece: NextPiece) -> Vec<Line<'static, >> {
    piece.iter().map(|row|
        Line::from(
            row.iter().map(|cell|
                render_cell(cell)
            ).collect::<Vec<Span>>()
        )
    ).collect()
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
        .title(Title::from(String::from(game.render_message()))
            .alignment(Alignment::Center));
    f.render_widget(msg, msg_layout[1]);

    let game_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(board_width_with_border),
            Constraint::Length(10),
            Constraint::Min(0),
        ].as_ref())
        .split(main_layout[0]);

    let board = Paragraph::new(render_board_to_text(game.render_board()))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double));
    f.render_widget(board, game_area[0]);

    let side_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
        ].as_ref())
        .split(game_area[1]);
    let next_piece = Paragraph::new(
        render_next_piece_to_text(game.render_next_piece())
    ).block(Block::default()
        .title("Next")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded));
    f.render_widget(next_piece, side_bar[0]);
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
            game.tick(command);
            last_tick = Instant::now();
            command = Command::NoOp;
        }
    }
}
