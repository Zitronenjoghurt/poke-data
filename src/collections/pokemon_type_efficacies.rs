use crate::data::unlinked::UnlinkedPokeData;
use crate::models::generation::GenerationId;
use crate::models::pokemon_type::PokemonTypeId;
use crate::models::pokemon_type_efficacies::{
    PokemonTypeEfficacies, PokemonTypeEfficaciesByGeneration,
};
use crate::types::pokemon_type::Type;
use std::collections::HashMap;

/// Represents a collection of Pokémon type efficacies.
///
/// To calculate the damage factors between Pokémon types, see:
/// [`get_damage_factor`](PokemonTypeEfficaciesCollection::get_damage_factor)
#[derive(Debug, Default)]
pub struct PokemonTypeEfficaciesCollection {
    default: PokemonTypeEfficacies,
    by_generation: PokemonTypeEfficaciesByGeneration,
}

impl PokemonTypeEfficaciesCollection {
    pub fn build_from_unlinked(data: &UnlinkedPokeData) -> Self {
        let default = data.pokemon_type_efficacies.clone();

        let mut by_generation = HashMap::new();
        let mut past_generation_ids: Vec<&GenerationId> =
            data.pokemon_type_efficacies_past.0.keys().collect();
        past_generation_ids.sort();

        for &past_generation_id in &past_generation_ids {
            if let Some(efficacies) = data.pokemon_type_efficacies_past.0.get(past_generation_id) {
                for generation_id in 1..=*past_generation_id {
                    let entry = by_generation
                        .entry(generation_id)
                        .or_insert_with(|| default.clone());
                    for (&type_pair, &damage_factor) in &efficacies.0 {
                        entry.0.insert(type_pair, damage_factor);
                    }
                }
            }
        }

        Self {
            default,
            by_generation: PokemonTypeEfficaciesByGeneration::new(by_generation),
        }
    }

    /// Calculates the damage factor between one attacking type and one defending type.
    ///
    /// # Arguments
    ///
    /// * `generation_id`: ID of the generation this damage factor check should take place in
    /// * `attacking_type`: Enum value of the attacking type
    /// * `defending_type`: Enum value of the defending type
    ///
    /// Returns: Option<f32>
    pub fn get_damage_factor_single(
        &self,
        generation_id: GenerationId,
        attacking_type: Type,
        defending_type: Type,
    ) -> Option<f32> {
        if let Some(efficacy) = self.by_generation.get_factor(
            generation_id,
            attacking_type as PokemonTypeId,
            defending_type as PokemonTypeId,
        ) {
            Some(efficacy)
        } else {
            self.default.get_factor(
                attacking_type as PokemonTypeId,
                defending_type as PokemonTypeId,
            )
        }
    }

    /// Calculates the damage factor between one attacking type and one or multiple defending types.
    ///
    /// # Arguments
    ///
    /// * `generation_id`: ID of the generation this damage factor check should take place in
    /// * `attacking_type`: Enum value of the attacking type
    /// * `defending_types`: Enum values of the defending types
    ///
    /// Returns: f32
    pub fn get_damage_factor(
        &self,
        generation_id: GenerationId,
        attacking_type: Type,
        defending_types: Vec<Type>,
    ) -> f32 {
        defending_types
            .iter()
            .map(|type_id| self.get_damage_factor_single(generation_id, attacking_type, *type_id))
            .fold(1.0, |factor_cum, factor| factor_cum * factor.unwrap_or(1.0))
    }
}
