use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Stat {
    Hp = 1,
    Attack = 2,
    Defense = 3,
    SpAttack = 4,
    SpDefense = 5,
    Speed = 6,
    Accuracy = 7,
    Evasion = 8,
}

impl From<u8> for Stat {
    fn from(value: u8) -> Self {
        match value & 0b1111 {
            1 => Self::Hp,
            2 => Self::Attack,
            3 => Self::Defense,
            4 => Self::SpAttack,
            5 => Self::SpDefense,
            6 => Self::Speed,
            7 => Self::Accuracy,
            8 => Self::Evasion,
            _ => unreachable!("Invalid stat value: {}", value),
        }
    }
}
