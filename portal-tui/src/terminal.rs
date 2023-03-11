use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{io::Stdout, time::Duration};
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

pub(crate) type Term = Terminal<CrosstermBackend<Stdout>>;

pub(crate) fn setup_terminal() -> std::io::Result<Terminal<CrosstermBackend<Stdout>>> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

pub(crate) fn exit_terminal(term: &mut Term) -> std::io::Result<()> {
    disable_raw_mode()?;
    term.show_cursor()?;
    Ok(())
}
