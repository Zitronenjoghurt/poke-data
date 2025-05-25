use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Gender {
    Female = 1,
    Male = 2,
    #[default]
    Genderless = 3,
}

impl From<u8> for Gender {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Female,
            2 => Self::Male,
            3 => Self::Genderless,
            _ => Self::default(),
        }
    }
}
