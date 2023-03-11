use crossterm::event::KeyCode;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{StatefulWidget, Widget};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum TextInputState {
    #[default]
    Unfocused,
    Focused,
    Edit(usize),
}

pub struct TextInputBuilder<'a> {
    contents: &'a mut String,
    placeholder_text: Option<&'static str>,
    key: Option<&'a KeyCode>,
    block: Option<Block<'a>>,
    style: Option<Style>,
    alignment: Option<Alignment>,
    wrap: Option<Wrap>,
}

impl<'a> TextInputBuilder<'a> {
    pub fn new(contents: &'a mut String, key: Option<&'a KeyCode>) -> Self {
        Self {
            contents,
            key,
            block: None,
            placeholder_text: None,
            style: None,
            alignment: None,
            wrap: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn placeholder_text(mut self, text: &'static str) -> Self {
        self.placeholder_text = Some(text);
        self
    }

    pub fn build(self) -> TextInput<'a> {
        TextInput {
            contents: self.contents,
            key: self.key,
            placeholder_text: self.placeholder_text.unwrap_or("Your Text Here"),
            block: self.block.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
            alignment: self.alignment.unwrap_or(Alignment::Left),
            wrap: self.wrap.unwrap_or(Wrap { trim: true }),
        }
    }
}

pub struct TextInput<'a> {
    contents: &'a mut String,
    placeholder_text: &'static str,
    key: Option<&'a KeyCode>,
    block: Block<'a>,
    style: Style,
    alignment: Alignment,
    wrap: Wrap,
}

impl<'a> TextInput<'a> {
    fn handle_input(&mut self, state: &mut <TextInput<'a> as StatefulWidget>::State) {
        match state {
            TextInputState::Unfocused => {}
            TextInputState::Focused => match self.key {
                Some(KeyCode::Enter) => {
                    *state = <TextInput<'a> as StatefulWidget>::State::Edit(self.contents.len())
                }
                _ => {}
            },
            TextInputState::Edit(cursor) => match self.key {
                Some(key) => match key {
                    KeyCode::Char(c) => self.contents.push(*c),
                    KeyCode::Delete => {
                        println!("Should pop char after cursor");
                    }
                    KeyCode::Backspace => {
                        println!("Should pop char before cursor");
                        let _ = self.contents.pop();
                    }
                    KeyCode::Enter | KeyCode::Esc => {
                        *state = <TextInput<'a> as StatefulWidget>::State::Focused;
                    }
                    KeyCode::Left => println!("Should decrement cursor"),
                    KeyCode::Right => println!("Should increment cursor"),
                    KeyCode::Up => todo!(),
                    KeyCode::Down => todo!(),
                    KeyCode::Home => println!("Should set cursor position to 0"),
                    KeyCode::End => println!("Should set cursor position to len"),
                    _ => {}
                    // KeyCode::PageUp => todo!(),
                    // KeyCode::PageDown => todo!(),
                    // KeyCode::Tab => todo!(),
                    // KeyCode::BackTab => todo!(),
                    // KeyCode::Insert => todo!(),
                    // KeyCode::F(_) => todo!(),
                    // KeyCode::Null => todo!(),
                    // KeyCode::CapsLock => todo!(),
                    // KeyCode::ScrollLock => todo!(),
                    // KeyCode::NumLock => todo!(),
                    // KeyCode::PrintScreen => todo!(),
                    // KeyCode::Pause => todo!(),
                    // KeyCode::Menu => todo!(),
                    // KeyCode::KeypadBegin => todo!(),
                    // KeyCode::Media(_) => todo!(),
                    // KeyCode::Modifier(_) => todo!(),
                },
                None => (),
            },
        }
    }
}

impl<'a> StatefulWidget for TextInput<'a> {
    type State = TextInputState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.handle_input(state);
        let (fg_color, content) = match self.contents.is_empty() {
            true => (Color::Cyan, self.placeholder_text),
            false => (Color::White, self.contents.as_str()),
        };
        let input = Paragraph::new(content)
            .block(self.block)
            .style(self.style.fg(fg_color))
            .alignment(self.alignment)
            .wrap(self.wrap);

        input.render(area, buf);
    }
}
