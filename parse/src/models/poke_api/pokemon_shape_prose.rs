use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::language::LanguageId;
use poke_data::models::shape::ShapeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeProseData {
    pokemon_shape_id: ShapeId,
    local_language_id: LanguageId,
    name: String,
    awesome_name: String,
    description: String,
}

impl PokeApiModel for ShapeProseData {
    fn file_name() -> &'static str {
        "pokemon_shape_prose"
    }
}

impl HasId for ShapeProseData {
    type Id = ShapeId;

    fn id(&self) -> Self::Id {
        self.pokemon_shape_id
    }
}

impl HasLocalizedNameDescription for ShapeProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}
