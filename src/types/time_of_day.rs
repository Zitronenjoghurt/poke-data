use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TimeOfDay {
    Day = 0,
    Night = 1,
    Dusk = 2,
    FullMoon = 3,
}

impl FromStr for TimeOfDay {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "day" => Ok(Self::Day),
            "night" => Ok(Self::Night),
            "dusk" => Ok(Self::Dusk),
            "full-moon" => Ok(Self::FullMoon),
            _ => Err(format!("Invalid time of day '{string}'").into()),
        }
    }
}
