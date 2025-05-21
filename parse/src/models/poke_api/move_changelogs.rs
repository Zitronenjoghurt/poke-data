use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon_move::changelog::UnlinkedPokemonMoveChangelog;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::pokemon_move_effect::PokemonMoveEffectId;
use poke_data::models::pokemon_move_target::PokemonMoveTargetId;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::models::version_group::VersionGroupId;
use poke_data::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveChangelogData {
    move_id: PokemonMoveId,
    changed_in_version_group_id: VersionGroupId,
    type_id: Option<PokemonTypeId>,
    power: Option<u8>,
    pp: Option<u8>,
    accuracy: Option<u8>,
    priority: Option<i8>,
    target_id: Option<PokemonMoveTargetId>,
    effect_id: Option<PokemonMoveEffectId>,
    effect_chance: Option<u8>,
}

impl PokeApiModel for MoveChangelogData {
    fn file_name() -> &'static str {
        "move_changelog"
    }
}

impl HasId for MoveChangelogData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.move_id
    }
}

impl IntoModel<UnlinkedPokemonMoveChangelog> for MoveChangelogData {
    fn into_model(self, _data: &RawData) -> UnlinkedPokemonMoveChangelog {
        UnlinkedPokemonMoveChangelog {
            pokemon_type: self.type_id.map(Type::from),
            power: self.power,
            pp: self.pp,
            accuracy: self.accuracy,
            priority: self.priority,
            target_id: self.target_id,
            effect_id: self.effect_id,
            effect_chance: self.effect_chance,
        }
    }
}

impl IntoModel<HashMap<VersionGroupId, UnlinkedPokemonMoveChangelog>> for Vec<MoveChangelogData> {
    fn into_model(self, data: &RawData) -> HashMap<VersionGroupId, UnlinkedPokemonMoveChangelog> {
        self.iter()
            .map(|entry| {
                (
                    entry.changed_in_version_group_id,
                    entry.clone().into_model(data),
                )
            })
            .collect()
    }
}
