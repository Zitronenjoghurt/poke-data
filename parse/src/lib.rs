use crate::models::poke_api::ability::AbilityData;
use crate::models::poke_api::ability_changelog::{AbilityChangelogData, AbilityChangelogId};
use crate::models::poke_api::ability_changelog_prose::AbilityChangelogProseData;
use crate::models::poke_api::ability_flavor_text::AbilityFlavorTextData;
use crate::models::poke_api::ability_names::AbilityNameData;
use crate::models::poke_api::ability_prose::AbilityProseData;
use crate::models::RemoteModel;
use crate::traits::has_id::IntoIdMap;
use crate::traits::into_model::IntoModel;
use poke_data::models::ability::AbilityId;
use poke_data::PokeData;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

mod models;
mod traits;

#[derive(Debug)]
pub struct RawData {
    pub abilities: HashMap<AbilityId, AbilityData>,
    pub ability_changelogs: HashMap<AbilityId, Vec<AbilityChangelogData>>,
    pub ability_changelog_prose: HashMap<AbilityChangelogId, Vec<AbilityChangelogProseData>>,
    pub ability_flavor_texts: HashMap<AbilityId, Vec<AbilityFlavorTextData>>,
    pub ability_names: HashMap<AbilityId, Vec<AbilityNameData>>,
    pub ability_prose: HashMap<AbilityId, Vec<AbilityProseData>>,
}

impl RawData {
    pub async fn load(base_path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            abilities: AbilityData::load(base_path).await?.into_id_map(),
            ability_changelogs: AbilityChangelogData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_changelog_prose: AbilityChangelogProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_flavor_texts: AbilityFlavorTextData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_names: AbilityNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_prose: AbilityProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
        })
    }

    pub fn parse(&self) -> PokeData {
        PokeData {
            abilities: self.abilities.clone().into_model(self),
        }
    }
}
