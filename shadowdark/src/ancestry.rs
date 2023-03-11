use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ancestry {
    Dwarf,
    Goblin,
    Elf,
    Halfling,
    HalfOrc,
    #[default]
    Human,
}
