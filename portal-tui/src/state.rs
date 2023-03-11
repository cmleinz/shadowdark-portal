use crate::ui::TextInputState;
use shadowdark::Character;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GuiElements {
    #[default]
    NameInput,
}

#[derive(Debug, Clone)]
pub struct GuiState {
    pub(crate) character: Character,
    pub(crate) name_ti: TextInputState,
    pub(crate) focused_element: GuiElements,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            character: Character::default(),
            name_ti: TextInputState::Focused,
            focused_element: GuiElements::NameInput,
        }
    }
}
