#![forbid(unsafe_code)]
use std::io::{self, stdout, Stdout};
use std::rc::Rc;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use crate::constants::{CONSERVATION_MODE, FN_LOCK};
use crate::file_utilities::file_exists;

mod file_utilities;
mod constants;

fn main() -> io::Result<()> {
    check_files_exist();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit: bool = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn check_files_exist() {
    if let Err(err) = file_exists(CONSERVATION_MODE) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    if let Err(err) = file_exists(FN_LOCK) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let main_layout: Rc<[Rect]> = Layout::new(
        Direction::Vertical,
        [
            Constraint::Max(1),
            Constraint::Min(0),
        ],
    )
        .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Lenovo Vantage"),
        main_layout[0],
    );

    let inner_layout: Rc<[Rect]> = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
        .split(main_layout[1]);
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Key"),
        inner_layout[0],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Value"),
        inner_layout[1],
    );
}