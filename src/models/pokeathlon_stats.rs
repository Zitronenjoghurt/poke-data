use crate::types::pokeathlon_stat::PokeathlonStat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Min, base and max pokeathlon stats for a certain Pokémon
pub struct PokeathlonStats {
    pub speed: (u8, u8, u8),
    pub power: (u8, u8, u8),
    pub skill: (u8, u8, u8),
    pub stamina: (u8, u8, u8),
    pub jump: (u8, u8, u8),
}

impl PokeathlonStats {
    pub fn get_stats(&self, stat: PokeathlonStat) -> (u8, u8, u8) {
        match stat {
            PokeathlonStat::Speed => self.speed,
            PokeathlonStat::Power => self.power,
            PokeathlonStat::Skill => self.skill,
            PokeathlonStat::Stamina => self.stamina,
            PokeathlonStat::Jump => self.jump,
        }
    }

    pub fn get_min(&self, stat: PokeathlonStat) -> u8 {
        match stat {
            PokeathlonStat::Speed => self.speed.0,
            PokeathlonStat::Power => self.power.0,
            PokeathlonStat::Skill => self.skill.0,
            PokeathlonStat::Stamina => self.stamina.0,
            PokeathlonStat::Jump => self.jump.0,
        }
    }

    pub fn get_base(&self, stat: PokeathlonStat) -> u8 {
        match stat {
            PokeathlonStat::Speed => self.speed.1,
            PokeathlonStat::Power => self.power.1,
            PokeathlonStat::Skill => self.skill.1,
            PokeathlonStat::Stamina => self.stamina.1,
            PokeathlonStat::Jump => self.jump.1,
        }
    }

    pub fn get_max(&self, stat: PokeathlonStat) -> u8 {
        match stat {
            PokeathlonStat::Speed => self.speed.2,
            PokeathlonStat::Power => self.power.2,
            PokeathlonStat::Skill => self.skill.2,
            PokeathlonStat::Stamina => self.stamina.2,
            PokeathlonStat::Jump => self.jump.2,
        }
    }

    pub fn set_stats(&mut self, stat: PokeathlonStat, min: u8, base: u8, max: u8) {
        match stat {
            PokeathlonStat::Speed => self.speed = (min, base, max),
            PokeathlonStat::Power => self.power = (min, base, max),
            PokeathlonStat::Skill => self.skill = (min, base, max),
            PokeathlonStat::Stamina => self.stamina = (min, base, max),
            PokeathlonStat::Jump => self.jump = (min, base, max),
        }
    }
}
