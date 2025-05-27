use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::color::{Color, ColorId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorData {
    id: ColorId,
    identifier: String,
}

impl PokeApiModel for ColorData {
    fn file_name() -> &'static str {
        "pokemon_colors"
    }
}

impl HasId for ColorData {
    type Id = ColorId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<Color> for ColorData {
    fn into_model(self, data: &RawData) -> Color {
        Color {
            id: self.id,
            identifier: self.identifier,
            names: data.color_names.get_model(&self.id, data),
        }
    }
}
