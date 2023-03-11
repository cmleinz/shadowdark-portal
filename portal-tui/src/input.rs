use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
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

pub enum Event<I> {
    Input(I),
    Tick,
}

// Should return the thread handle as well for proper error handling
pub(crate) fn spawn_input_handle_loop() -> mpsc::Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let handle = std::thread::spawn(move || {
        let tick_rate = Duration::from_millis(200);
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Failed polling") {
                if let CEvent::Key(key) = event::read().expect("Failed to read crossterm event") {
                    tx.send(Event::Input(key))
                        .expect("Failed to send event message");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if tx.send(Event::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });
    rx
}
