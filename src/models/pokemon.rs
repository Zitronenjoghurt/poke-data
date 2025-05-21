use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::encounter::Encounter;
use crate::models::generation::GenerationId;
use crate::models::pokemon::ability::{PokemonAbility, UnlinkedPokemonAbility};
use crate::models::species::{Species, SpeciesId};
use crate::models::version::VersionId;
use crate::traits::has_identifier::HasIdentifier;
use crate::types::base_stats::BaseStats;
use crate::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod ability;

pub type PokemonId = u16;

#[derive(Debug)]
pub struct Pokemon {
    pub id: PokemonId,
    pub identifier: String,
    pub species: Arc<Species>,
    types: Vec<Type>,
    /// The last generation this Pokémon had the specified type combination
    past_types: HashMap<GenerationId, Vec<Type>>,
    /// Height of this Pokémon in decimeters
    pub height: u16,
    /// Weight of this Pokémon in hectograms
    pub weight: u16,
    pub base_stats: BaseStats,
    pub base_experience: u16,
    pub order: Option<u16>,
    pub is_default: bool,
    pub abilities: Vec<PokemonAbility>,
    pub encounters: HashMap<VersionId, Vec<Arc<Encounter>>>,
}

impl Pokemon {
    pub fn get_types(&self, generation_id: GenerationId) -> &Vec<Type> {
        if self.past_types.is_empty() {
            return &self.types;
        }

        match self
            .past_types
            .keys()
            .filter(|key| generation_id <= **key)
            .max()
        {
            Some(latest_applicable_gen) => &self.past_types[latest_applicable_gen],
            None => &self.types,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemon {
    pub id: PokemonId,
    pub identifier: String,
    pub species_id: SpeciesId,
    pub types: Vec<Type>,
    pub past_types: HashMap<GenerationId, Vec<Type>>,
    pub height: u16,
    pub weight: u16,
    pub base_stats: BaseStats,
    pub base_experience: u16,
    pub order: Option<u16>,
    pub is_default: bool,
    pub abilities: Vec<UnlinkedPokemonAbility>,
}

impl Linkable for UnlinkedPokemon {
    type Linked = Arc<Pokemon>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let species = context
            .species
            .get(&self.species_id)
            .unwrap_or_else(|| {
                panic!(
                    "No species '{}' found for pokemon '{}'",
                    self.species_id, self.id
                )
            })
            .clone();

        let abilities = self.abilities.link(context);

        let relevant_encounters: Vec<Arc<Encounter>> = context
            .encounters
            .iter()
            .filter_map(|(_, encounter)| {
                if encounter.pokemon_id == self.id {
                    Some(encounter.clone())
                } else {
                    None
                }
            })
            .collect();

        let encounters = relevant_encounters.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<VersionId, Vec<Arc<Encounter>>>, encounter| {
                acc.entry(encounter.version.id)
                    .or_default()
                    .push(encounter.clone());
                acc
            },
        );

        let pokemon = Pokemon {
            id: self.id,
            identifier: self.identifier.clone(),
            species,
            types: self.types.clone(),
            past_types: self.past_types.clone(),
            height: self.height,
            weight: self.weight,
            base_stats: self.base_stats.clone(),
            base_experience: self.base_experience,
            order: self.order,
            is_default: self.is_default,
            abilities,
            encounters,
        };

        Arc::new(pokemon)
    }
}

impl HasIdentifier for Pokemon {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
