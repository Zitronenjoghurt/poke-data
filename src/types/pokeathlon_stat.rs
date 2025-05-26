use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PokeathlonStat {
    Speed = 1,
    Power = 2,
    Skill = 3,
    Stamina = 4,
    Jump = 5,
}

impl From<u8> for PokeathlonStat {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            1 => Self::Speed,
            2 => Self::Power,
            3 => Self::Skill,
            4 => Self::Stamina,
            5 => Self::Jump,
            _ => unreachable!("Invalid pokeathlon stat value: {}", value),
        }
    }
}
