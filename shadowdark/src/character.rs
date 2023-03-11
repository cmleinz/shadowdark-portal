use crate::{
    abilities::Abilities, alignment::Alignment, ancestry::Ancestry, class::Class,
    languages::Language,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder, Default, Clone, Serialize, Deserialize)]
pub struct Character {
    abilities: Abilities,
    ancestry: Ancestry,
    armor_class: u32,
    class: Class,
    level: u8,
    experience: u8,
    title: String,
    alignment: Alignment,
    background: String,
    deity: String,
    hit_points: u32,
    languages: Vec<Language>,
    pub name: String,
}
