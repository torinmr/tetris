use std::io;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::block::Title;
use game::{Command, Game, HEIGHT, WIDTH};

use tetromino::{PositionedTetromino, Tetromino};

mod tetromino;
pub mod game;

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
        .title(Title::from(String::from(game.get_message()))
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
