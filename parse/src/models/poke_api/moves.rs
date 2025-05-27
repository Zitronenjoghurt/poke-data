use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::contest_effect::ContestEffectId;
use poke_data::models::contest_type::ContestTypeId;
use poke_data::models::damage_class::DamageClassId;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon_move::{PokemonMoveId, UnlinkedPokemonMove};
use poke_data::models::pokemon_move_effect::PokemonMoveEffectId;
use poke_data::models::pokemon_move_target::PokemonMoveTargetId;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::models::super_contest_effect::SuperContestEffectId;
use poke_data::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveData {
    id: PokemonMoveId,
    identifier: String,
    generation_id: GenerationId,
    type_id: PokemonTypeId,
    power: Option<u8>,
    pp: Option<u8>,
    accuracy: Option<u8>,
    priority: i8,
    target_id: PokemonMoveTargetId,
    damage_class_id: DamageClassId,
    effect_id: Option<PokemonMoveEffectId>,
    effect_chance: Option<u8>,
    contest_type_id: Option<ContestTypeId>,
    contest_effect_id: Option<ContestEffectId>,
    super_contest_effect_id: Option<SuperContestEffectId>,
}

impl PokeApiModel for MoveData {
    fn file_name() -> &'static str {
        "moves"
    }
}

impl HasId for MoveData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedPokemonMove> for MoveData {
    fn into_model(self, data: &RawData) -> UnlinkedPokemonMove {
        let meta_data = data
            .move_meta
            .get(&self.id)
            .map(|meta| meta.clone().into_model(data));

        UnlinkedPokemonMove {
            id: self.id,
            identifier: self.identifier,
            names: data.move_names.get_model(&self.id, data),
            flavor_texts: data.move_flavor_texts.get_model(&self.id, data),
            generation_id: self.generation_id,
            pokemon_type: Type::from(self.type_id),
            power: self.power,
            pp: self.pp,
            accuracy: self.accuracy,
            priority: self.priority,
            target_id: self.target_id,
            damage_class_id: self.damage_class_id,
            effect_id: self.effect_id,
            effect_chance: self.effect_chance,
            contest_type_id: self.contest_type_id,
            contest_effect_id: self.contest_effect_id,
            super_contest_effect_id: self.super_contest_effect_id,
            changelogs: data.move_changelogs.get_model(&self.id, data),
            flag_ids: data.move_flag_map.get_model(&self.id, data),
            meta_data,
        }
    }
}
