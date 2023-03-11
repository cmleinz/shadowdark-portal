use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Class {
    #[default]
    Fighter,
    Priest,
    Wizard,
    Theif,
}
