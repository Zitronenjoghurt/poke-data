use crate::models::poke_api::ability::AbilityData;
use crate::models::poke_api::ability_changelog::{AbilityChangelogData, AbilityChangelogId};
use crate::models::poke_api::ability_changelog_prose::AbilityChangelogProseData;
use crate::models::poke_api::ability_flavor_text::AbilityFlavorTextData;
use crate::models::poke_api::ability_names::AbilityNameData;
use crate::models::poke_api::ability_prose::AbilityProseData;
use crate::models::poke_api::egg_group::EggGroupData;
use crate::models::poke_api::egg_group_prose::EggGroupProseData;
use crate::models::poke_api::generation::GenerationData;
use crate::models::poke_api::generation_names::GenerationNameData;
use crate::models::poke_api::pokemon::PokemonData;
use crate::models::poke_api::pokemon_abilities::PokemonAbilityData;
use crate::models::poke_api::region::RegionData;
use crate::models::poke_api::region_names::RegionNameData;
use crate::models::RemoteModel;
use crate::traits::has_id::IntoIdMap;
use crate::traits::into_model::IntoModel;
use poke_data::models::ability::AbilityId;
use poke_data::models::egg_group::EggGroupId;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::region::RegionId;
use poke_data::UnlinkedPokeData;
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
    pub egg_groups: HashMap<EggGroupId, EggGroupData>,
    pub egg_group_prose: HashMap<EggGroupId, Vec<EggGroupProseData>>,
    pub generations: HashMap<GenerationId, GenerationData>,
    pub generation_names: HashMap<GenerationId, Vec<GenerationNameData>>,
    pub pokemon: HashMap<PokemonId, PokemonData>,
    pub pokemon_abilities: HashMap<PokemonId, Vec<PokemonAbilityData>>,
    pub regions: HashMap<RegionId, RegionData>,
    pub region_names: HashMap<RegionId, Vec<RegionNameData>>,
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
            egg_groups: EggGroupData::load(base_path).await?.into_id_map(),
            egg_group_prose: EggGroupProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            generations: GenerationData::load(base_path).await?.into_id_map(),
            generation_names: GenerationNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon: PokemonData::load(base_path).await?.into_id_map(),
            pokemon_abilities: PokemonAbilityData::load(base_path)
                .await?
                .into_id_map_grouped(),
            regions: RegionData::load(base_path).await?.into_id_map(),
            region_names: RegionNameData::load(base_path).await?.into_id_map_grouped(),
        })
    }

    pub fn parse(&self) -> UnlinkedPokeData {
        UnlinkedPokeData {
            abilities: self.abilities.clone().into_model(self),
            egg_groups: self.egg_groups.clone().into_model(self),
            generations: self.generations.clone().into_model(self),
            regions: self.regions.clone().into_model(self),
            pokemon: self.pokemon.clone().into_model(self),
        }
    }
}
