use super::{text_input_widget::TextInputBuilder, TextInput};
use crate::state::GuiState;
use crate::terminal::Term;
use crate::{input::Event, state::GuiElements};
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
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
        Wrap,
    },
    Terminal,
};

pub fn render_loop(
    mut term: Term,
    mut state: GuiState,
    rx: mpsc::Receiver<Event<KeyEvent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let key = match rx.recv()? {
            Event::Input(event) => Some(event.code),
            Event::Tick => None,
        };
        // Should write a state contoller which mutates.state.focused_element
        term.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            if state.focused_element == GuiElements::NameInput {
                let name_widget = render_name(&mut state.character.name, &key);
                rect.render_stateful_widget(name_widget, chunks[0], &mut state.name_ti);
            } else {
                let name_widget = render_name(&mut state.character.name, &None);
                rect.render_stateful_widget(name_widget, chunks[0], &mut state.name_ti);
            }
        })?;
    }
}

fn render_name<'a>(name: &'a mut String, key: &'a Option<KeyCode>) -> TextInput<'a> {
    TextInputBuilder::new(name, key.as_ref())
        .block(Block::default().title("Name").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .build()
}
