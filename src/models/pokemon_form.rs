use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::localized_names::LocalizedStrings;
use crate::models::pokeathlon_stats::PokeathlonStats;
use crate::models::pokemon::PokemonId;
use crate::models::version_group::{VersionGroup, VersionGroupId};
use crate::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type PokemonFormId = u16;

#[derive(Debug)]
pub struct PokemonForm {
    pub id: PokemonFormId,
    pub identifier: String,
    pub form_identifier: Option<String>,
    pub form_names: Option<LocalizedStrings>,
    pub type_overrides: Option<Vec<Type>>,
    pub pokeathlon_stats: Option<PokeathlonStats>,
    pub pokemon_id: PokemonId,
    pub introduced_in_version_group: Arc<VersionGroup>,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub form_order: u8,
    pub order: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonForm {
    pub id: PokemonFormId,
    pub identifier: String,
    pub form_identifier: Option<String>,
    pub form_names: Option<LocalizedStrings>,
    pub type_overrides: Option<Vec<Type>>,
    pub pokeathlon_stats: Option<PokeathlonStats>,
    pub pokemon_id: PokemonId,
    pub introduced_in_version_group_id: VersionGroupId,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub form_order: u8,
    pub order: u16,
}

impl Linkable for UnlinkedPokemonForm {
    type Linked = Arc<PokemonForm>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let introduced_in_version_group = context
            .version_groups
            .get(&self.introduced_in_version_group_id)
            .unwrap_or_else(|| {
                panic!(
                    "No version group '{}' found for pokemon form '{}'",
                    self.introduced_in_version_group_id, self.id
                )
            })
            .clone();

        let form = PokemonForm {
            id: self.id,
            identifier: self.identifier.clone(),
            form_identifier: self.form_identifier.clone(),
            form_names: self.form_names.clone(),
            type_overrides: self.type_overrides.clone(),
            pokeathlon_stats: self.pokeathlon_stats.clone(),
            pokemon_id: self.pokemon_id.clone(),
            introduced_in_version_group,
            is_default: self.is_default,
            is_battle_only: self.is_battle_only,
            is_mega: self.is_mega,
            form_order: self.form_order,
            order: self.order,
        };

        Arc::new(form)
    }
}
