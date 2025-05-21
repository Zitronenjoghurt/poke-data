use crate::models::pokemon_type::PokemonTypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Type {
    #[default]
    None = 0,
    Normal = 1,
    Fighting = 2,
    Flying = 3,
    Poison = 4,
    Ground = 5,
    Rock = 6,
    Bug = 7,
    Ghost = 8,
    Steel = 9,
    Fire = 10,
    Water = 11,
    Grass = 12,
    Electric = 13,
    Psychic = 14,
    Ice = 15,
    Dragon = 16,
    Dark = 17,
    Fairy = 18,
}

impl From<PokemonTypeId> for Type {
    fn from(value: PokemonTypeId) -> Self {
        match value {
            1 => Self::Normal,
            2 => Self::Fighting,
            3 => Self::Flying,
            4 => Self::Poison,
            5 => Self::Ground,
            6 => Self::Rock,
            7 => Self::Bug,
            8 => Self::Ghost,
            9 => Self::Steel,
            10 => Self::Fire,
            11 => Self::Water,
            12 => Self::Grass,
            13 => Self::Electric,
            14 => Self::Psychic,
            15 => Self::Ice,
            16 => Self::Dragon,
            17 => Self::Dark,
            18 => Self::Fairy,
            _ => Self::None,
        }
    }
}
