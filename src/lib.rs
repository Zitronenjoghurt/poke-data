use crate::models::ability::{Ability, AbilityId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod models;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PokeData {
    pub abilities: HashMap<AbilityId, Ability>,
}
