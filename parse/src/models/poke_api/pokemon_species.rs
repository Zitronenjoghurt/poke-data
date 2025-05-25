use crate::models::poke_api::evolution_chains::EvolutionChainId;
use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::color::ColorId;
use poke_data::models::generation::GenerationId;
use poke_data::models::growth_rate::GrowthRateId;
use poke_data::models::habitat::HabitatId;
use poke_data::models::shape::ShapeId;
use poke_data::models::species::{SpeciesId, UnlinkedSpecies};
use poke_data::types::capture_rate::CaptureRate;
use poke_data::types::gender_rate::GenderRate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesData {
    id: SpeciesId,
    identifier: String,
    generation_id: GenerationId,
    pub evolves_from_species_id: Option<SpeciesId>,
    pub evolution_chain_id: EvolutionChainId,
    pub color_id: ColorId,
    pub shape_id: ShapeId,
    pub habitat_id: Option<HabitatId>,
    pub gender_rate: i8,
    pub capture_rate: u8,
    pub base_happiness: u8,
    pub is_baby: u8,
    pub hatch_counter: u8,
    pub has_gender_differences: u8,
    pub growth_rate_id: GrowthRateId,
    pub forms_switchable: u8,
    pub is_legendary: u8,
    pub is_mythical: u8,
    pub order: u16,
}

impl PokeApiModel for PokemonSpeciesData {
    fn file_name() -> &'static str {
        "pokemon_species"
    }
}

impl HasId for PokemonSpeciesData {
    type Id = SpeciesId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedSpecies> for PokemonSpeciesData {
    fn into_model(self, data: &RawData) -> UnlinkedSpecies {
        let evolution_chain = data
            .evolution_chains
            .get(&self.evolution_chain_id)
            .unwrap_or_else(|| {
                panic!(
                    "No evolution chain '{}' for species '{}'",
                    self.evolution_chain_id, self.id
                )
            });

        let form_description = data
            .species_prose
            .get(&self.id)
            .map(|descriptions| descriptions.clone().into_model(data));

        let evolutions = data.evolutions.get_model(&self.id, data);

        UnlinkedSpecies {
            id: self.id,
            identifier: self.identifier,
            names: data.species_names.get_model(&self.id, data),
            flavor_texts: data.species_flavor_texts.get_model(&self.id, data),
            form_description,
            generation_id: self.generation_id,
            evolves_from_species_id: self.evolves_from_species_id,
            baby_trigger_item_id: evolution_chain.baby_trigger_item_id,
            evolutions,
            color_id: self.color_id,
            shape_id: self.shape_id,
            habitat_id: self.habitat_id,
            gender_rate: GenderRate::new(self.gender_rate),
            capture_rate: CaptureRate::new(self.capture_rate),
            base_happiness: self.base_happiness,
            is_baby: self.is_baby == 1,
            hatch_counter: self.hatch_counter,
            has_gender_differences: self.has_gender_differences == 1,
            growth_rate_id: self.growth_rate_id,
            forms_switchable: self.forms_switchable == 1,
            is_legendary: self.is_legendary == 1,
            is_mythical: self.is_mythical == 1,
            order: self.order,
        }
    }
}
