use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    // Common
    #[default]
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Goblin,
    Merran,
    Orcish,
    Reptilian,
    Sylvan,
    Thanian,
    // Rare
    Celestial,
    Diabolic,
    Draconic,
    Primordial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageRarity {
    Common,
    Rare,
}

impl Language {
    const fn who_speaks_it(&self) -> &'static str {
        match self {
            Language::Common => "Most humanoids",
            Language::Dwarvish => "Dwarves",
            Language::Elvish => "Elves",
            Language::Giant => "Giants, ogres, trolls",
            Language::Goblin => "Bugbears, goblins, hobgoblins",
            Language::Merran => "Merfolk, sahuagin, sirens",
            Language::Orcish => "Orcs",
            Language::Reptilian => "Sylvan",
            Language::Sylvan => "Centaurs, dryads, faeries",
            Language::Thanian => "Minotaurs, beastmen, manticores",
            Language::Celestial => "Angels",
            Language::Diabolic => "Demons, devils",
            Language::Draconic => "Dragons",
            Language::Primordial => "Elder things, elementals",
        }
    }
}

impl From<Language> for LanguageRarity {
    fn from(value: Language) -> Self {
        match value {
            Language::Common => LanguageRarity::Common,
            Language::Dwarvish => LanguageRarity::Common,
            Language::Elvish => LanguageRarity::Common,
            Language::Giant => LanguageRarity::Common,
            Language::Goblin => LanguageRarity::Common,
            Language::Merran => LanguageRarity::Common,
            Language::Orcish => LanguageRarity::Common,
            Language::Reptilian => LanguageRarity::Common,
            Language::Sylvan => LanguageRarity::Common,
            Language::Thanian => LanguageRarity::Common,
            Language::Celestial => LanguageRarity::Rare,
            Language::Diabolic => LanguageRarity::Rare,
            Language::Draconic => LanguageRarity::Rare,
            Language::Primordial => LanguageRarity::Rare,
        }
    }
}
