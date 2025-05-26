use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::pokemon_form::{PokemonFormId, UnlinkedPokemonForm};
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonFormData {
    pub id: PokemonFormId,
    identifier: String,
    form_identifier: Option<String>,
    pub pokemon_id: PokemonId,
    introduced_in_version_group_id: VersionGroupId,
    is_default: u8,
    is_battle_only: u8,
    is_mega: u8,
    form_order: u8,
    order: u16,
}

impl PokeApiModel for PokemonFormData {
    fn file_name() -> &'static str {
        "pokemon_forms"
    }
}

impl HasId for PokemonFormData {
    type Id = PokemonFormId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedPokemonForm> for PokemonFormData {
    fn into_model(self, data: &RawData) -> UnlinkedPokemonForm {
        let form_names = data
            .pokemon_form_names
            .get(&self.id)
            .map(|names| names.clone().into_model(data));

        let type_overrides = data
            .pokemon_form_types
            .get(&self.id)
            .map(|types| types.clone().into_model(data));

        let pokeathlon_stats = data
            .pokemon_form_pokeathlon_stats
            .get(&self.id)
            .map(|stats| stats.clone().into_model(data));

        UnlinkedPokemonForm {
            id: self.id,
            identifier: self.identifier,
            form_identifier: self.form_identifier,
            form_names,
            type_overrides,
            pokeathlon_stats,
            pokemon_id: self.pokemon_id,
            introduced_in_version_group_id: self.introduced_in_version_group_id,
            is_default: self.is_default == 1,
            is_battle_only: self.is_battle_only == 1,
            is_mega: self.is_mega == 1,
            form_order: self.form_order,
            order: self.order,
        }
    }
}
