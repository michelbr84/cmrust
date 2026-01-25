//! TUI application logic.

use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// App state.
pub enum AppState {
    MainMenu,
    InGame,
    Quit,
}

/// Run the TUI application.
pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> anyhow::Result<()> {
    let mut state = AppState::MainMenu;

    loop {
        terminal.draw(|f| ui(f, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    state = AppState::Quit;
                }
                KeyCode::Enter => {
                    if matches!(state, AppState::MainMenu) {
                        state = AppState::InGame;
                    }
                }
                KeyCode::Esc => {
                    if matches!(state, AppState::InGame) {
                        state = AppState::MainMenu;
                    }
                }
                _ => {}
            }
        }

        if matches!(state, AppState::Quit) {
            break;
        }
    }

    Ok(())
}

/// Render UI.
fn ui(f: &mut Frame, state: &AppState) {
    let area = f.area();

    let block = Block::default()
        .title(" CM Rust ")
        .borders(Borders::ALL);

    let content = match state {
        AppState::MainMenu => {
            "╔═══════════════════════════════════════╗\n\
             ║     CM RUST - Football Manager        ║\n\
             ╠═══════════════════════════════════════╣\n\
             ║                                       ║\n\
             ║     [Enter] Start Game                ║\n\
             ║     [Q]     Quit                      ║\n\
             ║                                       ║\n\
             ╚═══════════════════════════════════════╝"
        }
        AppState::InGame => {
            "In Game Mode\n\n\
             Press [Esc] to return to menu\n\
             Press [Q] to quit"
        }
        AppState::Quit => "Goodbye!",
    };

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
