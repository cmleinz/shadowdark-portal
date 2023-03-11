use serde::{Deserialize, Serialize};

pub type AbilityScore = i8;

pub type AbilityMod = i8;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Abilities {
    strength: AbilityScore,
    dexterity: AbilityScore,
    constitution: AbilityScore,
    intelligence: AbilityScore,
    wisdom: AbilityScore,
    charisma: AbilityScore,
}

impl Default for Abilities {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}

pub(crate) const fn ability_score_to_mod(score: AbilityScore) -> AbilityMod {
    match score {
        i8::MIN..=3 => -4,
        4 | 5 => -3,
        6 | 7 => -2,
        8 | 9 => -1,
        10 | 11 => 0,
        12 | 13 => 1,
        14 | 15 => 2,
        16 | 17 => 3,
        18..=i8::MAX => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_to_modifier() {
        for i in i8::MIN..=3 {
            assert_eq!(ability_score_to_mod(i), -4);
        }
        for i in 4..=5 {
            assert_eq!(ability_score_to_mod(i), -3);
        }
        for i in 6..=7 {
            assert_eq!(ability_score_to_mod(i), -2);
        }
        for i in 8..=9 {
            assert_eq!(ability_score_to_mod(i), -1);
        }
        for i in 10..=11 {
            assert_eq!(ability_score_to_mod(i), 0);
        }
        for i in 12..=13 {
            assert_eq!(ability_score_to_mod(i), 1);
        }
        for i in 14..=15 {
            assert_eq!(ability_score_to_mod(i), 2);
        }
        for i in 16..=17 {
            assert_eq!(ability_score_to_mod(i), 3);
        }
        for i in 18..=i8::MAX {
            assert_eq!(ability_score_to_mod(i), 4);
        }
    }
}
