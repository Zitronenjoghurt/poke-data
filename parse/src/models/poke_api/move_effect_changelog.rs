use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::localized_names::LocalizedStrings;
use poke_data::models::pokemon_move_effect::PokemonMoveEffectId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MoveEffectChangelogId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEffectChangelogData {
    id: MoveEffectChangelogId,
    effect_id: PokemonMoveEffectId,
    changed_in_version_group_id: VersionGroupId,
}

impl PokeApiModel for MoveEffectChangelogData {
    fn file_name() -> &'static str {
        "move_effect_changelog"
    }
}

impl HasId for MoveEffectChangelogData {
    type Id = PokemonMoveEffectId;

    fn id(&self) -> Self::Id {
        self.effect_id
    }
}

impl IntoModel<HashMap<VersionGroupId, LocalizedStrings>> for Vec<MoveEffectChangelogData> {
    fn into_model(self, data: &RawData) -> HashMap<VersionGroupId, LocalizedStrings> {
        self.iter()
            .map(|entry| {
                let effects = data.move_effect_changelog_prose.get_model(&entry.id, data);
                (entry.changed_in_version_group_id, effects)
            })
            .collect()
    }
}
