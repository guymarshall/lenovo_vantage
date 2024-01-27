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
use crate::file_utilities::{file_exists, read_file, write_to_file};

mod file_utilities;
mod constants;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Setting {
    FnLock,
    ConservationMode,
}

struct App {
    selected_setting: Setting,
}

impl App {
    fn new() -> Self {
        App {
            selected_setting: Setting::FnLock,
        }
    }

    fn toggle_selected_setting(&mut self) {
        match self.selected_setting {
            Setting::FnLock => self.selected_setting = Setting::ConservationMode,
            Setting::ConservationMode => self.selected_setting = Setting::FnLock,
        }
    }
}

fn main() -> io::Result<()> {
    check_files_exist();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    let mut should_quit: bool = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events(&mut app)?;
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

fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press { match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => {
                    app.selected_setting = match app.selected_setting {
                        Setting::FnLock => Setting::ConservationMode,
                        Setting::ConservationMode => Setting::FnLock,
                    };
                }
                KeyCode::Down => {
                    app.selected_setting = match app.selected_setting {
                        Setting::FnLock => Setting::ConservationMode,
                        Setting::ConservationMode => Setting::FnLock,
                    };
                }
                KeyCode::Enter => write_selected_setting(app), // Handle enter key
                _ => {}
            } }
        }
    }
    Ok(false)
}

fn write_selected_setting(app: &App) {
    match app.selected_setting {
        Setting::FnLock => write_to_file(FN_LOCK, toggle_value(read_file(FN_LOCK))),
        Setting::ConservationMode => write_to_file(CONSERVATION_MODE, toggle_value(read_file(CONSERVATION_MODE))),
    }.expect("Failed to write to file");
    println!("Setting saved!");
}

fn toggle_value(value: String) -> String {
    match value.as_str() {
        "0" => "1".to_string(),
        "1" => "0".to_string(),
        _ => value,
    }
}

fn ui(frame: &mut Frame, app: &App) {
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

    let settings = ["Fn Lock", "Conservation Mode"];
    let values = [read_file(FN_LOCK), read_file(CONSERVATION_MODE)];

    let settings_list = List::new(settings.iter().cloned().map(Text::raw))
        .block(Block::default().title("Settings").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .direction(ListDirection::TopToBottom);

    frame.render_widget(settings_list, inner_layout[0]);

    let values_list = List::new(
        values
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, text)| {
                let style = if index == app.selected_setting as usize {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                Text::styled(text, style)
            }),
    )
        .block(Block::default().title("Values").borders(Borders::ALL))
        .direction(ListDirection::TopToBottom);

    frame.render_widget(values_list, inner_layout[1]);
}