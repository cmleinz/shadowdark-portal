#![allow(dead_code, unused)]
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use shadowdark::Character;
use std::time::Duration;
use std::{sync::mpsc, time::Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

mod input;
mod state;
mod terminal;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("Failed to open terminal in raw mode. Terminal may be incompatible");

    let rx = input::spawn_input_handle_loop();
    let mut terminal = terminal::setup_terminal()?;
    let mut state = state::GuiState::default();

    ui::render_loop(terminal, state, rx)?;

    Ok(())
}
