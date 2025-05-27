use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::shape::{Shape, ShapeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeData {
    id: ShapeId,
    identifier: String,
}

impl PokeApiModel for ShapeData {
    fn file_name() -> &'static str {
        "pokemon_shapes"
    }
}

impl HasId for ShapeData {
    type Id = ShapeId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<Shape> for ShapeData {
    fn into_model(self, data: &RawData) -> Shape {
        Shape {
            id: self.id,
            identifier: self.identifier,
            prose: data.shape_prose.get_model(&self.id, data),
        }
    }
}
