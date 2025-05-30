use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::base_stats::BaseStats;
use crate::models::encounter::Encounter;
use crate::models::generation::GenerationId;
use crate::models::pokemon::ability::{
    PokemonAbilitiesPast, PokemonAbility, UnlinkedPokemonAbilitiesPast, UnlinkedPokemonAbility,
};
use crate::models::pokemon::moveset::{Moveset, UnlinkedMoveset};
use crate::models::pokemon::wild_item::{PokemonWildItems, UnlinkedPokemonWildItems};
use crate::models::pokemon_form::{PokemonForm, PokemonFormId};
use crate::models::species::{Species, SpeciesId};
use crate::models::version::VersionId;
use crate::traits::has_identifier::HasIdentifier;
use crate::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod ability;
pub mod moveset;
pub mod wild_item;

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
    abilities: Vec<PokemonAbility>,
    past_abilities: PokemonAbilitiesPast,
    pub moveset: Moveset,
    pub encounters: HashMap<VersionId, Vec<Arc<Encounter>>>,
    pub default_form: Option<Arc<PokemonForm>>,
    pub forms: Vec<Arc<PokemonForm>>,
    pub wild_items: PokemonWildItems,
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

    pub fn get_abilities(&self, generation_id: GenerationId) -> Vec<PokemonAbility> {
        if self.past_abilities.is_empty() {
            return self.abilities.clone();
        }

        match self
            .past_abilities
            .get_map()
            .keys()
            .filter(|key| generation_id <= **key)
            .max()
        {
            Some(latest_applicable_gen) => {
                let past_abilities = self
                    .past_abilities
                    .get_abilities(latest_applicable_gen)
                    .unwrap();

                let mut abilities: Vec<PokemonAbility> = self
                    .abilities
                    .iter()
                    .filter_map(|ability| {
                        if let Some(past_ability) = past_abilities
                            .iter()
                            .find(|past_ability| past_ability.slot == ability.slot)
                        {
                            past_ability.to_ability()
                        } else {
                            Some(ability.clone())
                        }
                    })
                    .collect();

                past_abilities.iter().for_each(|past_ability| {
                    if !abilities
                        .iter()
                        .any(|ability| ability.slot == past_ability.slot)
                    {
                        if let Some(past_pokemon_ability) = past_ability.to_ability() {
                            abilities.push(past_pokemon_ability);
                        }
                    }
                });

                abilities
            }
            None => self.abilities.clone(),
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
    pub past_abilities: UnlinkedPokemonAbilitiesPast,
    pub moveset: UnlinkedMoveset,
    pub form_ids: Vec<PokemonFormId>,
    pub wild_items: UnlinkedPokemonWildItems,
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
        let past_abilities = self.past_abilities.link(context);
        let moveset = self.moveset.link(context);

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

        let forms: Vec<Arc<PokemonForm>> = self
            .form_ids
            .iter()
            .map(|form_id| {
                context
                    .pokemon_forms
                    .get(form_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "No pokemon form '{}' found for pokemon '{}'",
                            form_id, self.id
                        )
                    })
                    .clone()
            })
            .collect();

        let default_form = forms.iter().find(|form| form.is_default).cloned();

        let wild_items = self.wild_items.link(context);

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
            past_abilities,
            moveset,
            encounters,
            default_form,
            forms,
            wild_items,
        };

        Arc::new(pokemon)
    }
}

impl HasIdentifier for Pokemon {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
