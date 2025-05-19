use crate::models::generation::GenerationId;
use crate::models::pokemon_type::PokemonTypeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonTypeEfficacies(pub HashMap<(PokemonTypeId, PokemonTypeId), u8>);

impl PokemonTypeEfficacies {
    pub fn new(efficacies: HashMap<(PokemonTypeId, PokemonTypeId), u8>) -> Self {
        Self(efficacies)
    }

    pub fn get_factor(
        &self,
        attacking_type: PokemonTypeId,
        defending_type: PokemonTypeId,
    ) -> Option<f32> {
        self.0
            .get(&(attacking_type, defending_type))
            .map(|f| *f as f32 / 100.0)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonTypeEfficaciesByGeneration(pub HashMap<GenerationId, PokemonTypeEfficacies>);

impl PokemonTypeEfficaciesByGeneration {
    pub fn new(efficacies: HashMap<GenerationId, PokemonTypeEfficacies>) -> Self {
        Self(efficacies)
    }

    pub fn get_factor(
        &self,
        generation: GenerationId,
        attacking_type: PokemonTypeId,
        defending_type: PokemonTypeId,
    ) -> Option<f32> {
        self.0
            .get(&generation)
            .and_then(|efficacies| efficacies.get_factor(attacking_type, defending_type))
    }
}
