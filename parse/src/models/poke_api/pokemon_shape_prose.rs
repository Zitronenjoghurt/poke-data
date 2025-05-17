use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::shape::{ShapeId, ShapeProse, ShapeProseEntry};
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

impl IntoModel<ShapeProseEntry> for ShapeProseData {
    fn into_model(self, _data: &RawData) -> ShapeProseEntry {
        ShapeProseEntry {
            name: self.name,
            awesome_name: self.awesome_name,
            description: self.description,
        }
    }
}

impl IntoModel<ShapeProse> for Vec<ShapeProseData> {
    fn into_model(self, data: &RawData) -> ShapeProse {
        let entries = self
            .iter()
            .map(|prose| (prose.local_language_id, prose.clone().into_model(data)))
            .collect();
        ShapeProse::new(entries)
    }
}
