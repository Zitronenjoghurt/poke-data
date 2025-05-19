use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::encounter::Encounter;
use crate::models::pokemon::ability::{PokemonAbility, UnlinkedPokemonAbility};
use crate::models::species::{Species, SpeciesId};
use crate::models::version::VersionId;
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
    pub height: u16,
    pub weight: u16,
    pub base_experience: u16,
    pub order: Option<u16>,
    pub is_default: bool,
    pub abilities: Vec<PokemonAbility>,
    pub encounters: HashMap<VersionId, Vec<Arc<Encounter>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemon {
    pub id: PokemonId,
    pub identifier: String,
    pub species_id: SpeciesId,
    pub height: u16,
    pub weight: u16,
    pub base_experience: u16,
    pub order: Option<u16>,
    pub is_default: bool,
    pub abilities: Vec<UnlinkedPokemonAbility>,
}

impl Linkable for UnlinkedPokemon {
    type Linked = Arc<Pokemon>;

    fn link(&self, data: &PokeData) -> Self::Linked {
        let species = data
            .species
            .get(&self.species_id)
            .unwrap_or_else(|| {
                panic!(
                    "No species '{}' found for pokemon '{}'",
                    self.species_id, self.id
                )
            })
            .clone();

        let abilities = self.abilities.link(data);

        let relevant_encounters: Vec<Arc<Encounter>> = data
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
            height: self.height,
            weight: self.weight,
            base_experience: self.base_experience,
            order: self.order,
            is_default: self.is_default,
            abilities,
            encounters,
        };

        Arc::new(pokemon)
    }
}
