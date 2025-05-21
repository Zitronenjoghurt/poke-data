use crate::types::stat::Stat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Mapping base stat values with their respective EV for a certain Pokémon
pub struct BaseStats {
    pub hp: (u8, u8),
    pub attack: (u8, u8),
    pub defense: (u8, u8),
    pub special_attack: (u8, u8),
    pub special_defense: (u8, u8),
    pub speed: (u8, u8),
}

impl BaseStats {
    pub fn get_value(&self, stat: Stat) -> u8 {
        match stat {
            Stat::Hp => self.hp.0,
            Stat::Attack => self.attack.0,
            Stat::Defense => self.defense.0,
            Stat::SpAttack => self.special_attack.0,
            Stat::SpDefense => self.special_defense.0,
            Stat::Speed => self.speed.0,
            _ => 0,
        }
    }

    pub fn get_effort(&self, stat: Stat) -> u8 {
        match stat {
            Stat::Hp => self.hp.1,
            Stat::Attack => self.attack.1,
            Stat::Defense => self.defense.1,
            Stat::SpAttack => self.special_attack.1,
            Stat::SpDefense => self.special_defense.1,
            Stat::Speed => self.speed.1,
            _ => 0,
        }
    }

    pub fn set_stat(&mut self, stat: Stat, base_value: u8, effort: u8) {
        match stat {
            Stat::Hp => self.hp = (base_value, effort),
            Stat::Attack => self.attack = (base_value, effort),
            Stat::Defense => self.defense = (base_value, effort),
            Stat::SpAttack => self.special_attack = (base_value, effort),
            Stat::SpDefense => self.special_defense = (base_value, effort),
            Stat::Speed => self.speed = (base_value, effort),
            _ => {}
        }
    }
}
