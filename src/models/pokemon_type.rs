use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::damage_class::{DamageClass, DamageClassId};
use crate::models::generation::{Generation, GenerationId};
use crate::models::localized_names::LocalizedNames;
use crate::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type PokemonTypeId = u16;

#[derive(Debug)]
pub struct PokemonType {
    /// The ID which uniquely identifies this Pokémon type
    pub id: PokemonTypeId,
    /// A slug string for identification
    pub identifier: String,
    /// The enum value of this Pokémon type; can be used for easier identification
    pub type_enum: Type,
    /// Names in different languages (availability might differ between objects)
    pub names: LocalizedNames,
    /// The generation this type was introduced in
    pub generation: Arc<Generation>,
    /// The type-based physical/special split ended in Generation 4
    pub damage_class: Option<Arc<DamageClass>>,
    /// The index this Pokémon type was displayed at in different generations
    pub game_indices: PokemonTypeGameIndices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeGameIndices(HashMap<GenerationId, u16>);

impl PokemonTypeGameIndices {
    pub fn new(indices: HashMap<GenerationId, u16>) -> Self {
        Self(indices)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonType {
    pub id: PokemonTypeId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub generation_id: GenerationId,
    pub damage_class_id: Option<DamageClassId>,
    pub game_indices: PokemonTypeGameIndices,
}

impl Linkable for UnlinkedPokemonType {
    type Linked = Arc<PokemonType>;

    fn link(&self, data: &PokeData) -> Self::Linked {
        let generation = data
            .generations
            .get(&self.generation_id)
            .unwrap_or_else(|| {
                panic!(
                    "No generation '{}' found for pokemon type '{}'",
                    self.generation_id, self.id
                )
            })
            .clone();

        let damage_class = self.damage_class_id.map(|damage_class_id| {
            data.damage_classes
                .get(&damage_class_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No damage class '{}' found for pokemon type '{}'",
                        damage_class_id, self.id
                    )
                })
                .clone()
        });

        let pokemon_type = PokemonType {
            id: self.id,
            identifier: self.identifier.clone(),
            type_enum: Type::from(self.id),
            names: self.names.clone(),
            generation,
            damage_class,
            game_indices: self.game_indices.clone(),
        };

        Arc::new(pokemon_type)
    }
}
