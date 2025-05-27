use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::location::{LocalizedLocationName, LocalizedLocationNames, LocationId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationNameData {
    location_id: LocationId,
    local_language_id: LanguageId,
    name: String,
    subtitle: String,
}

impl PokeApiModel for LocationNameData {
    fn file_name() -> &'static str {
        "location_names"
    }
}

impl HasId for LocationNameData {
    type Id = LocationId;

    fn id(&self) -> Self::Id {
        self.location_id
    }
}

impl IntoModel<LocalizedLocationName> for LocationNameData {
    fn into_model(self, _data: &RawData) -> LocalizedLocationName {
        LocalizedLocationName {
            name: self.name,
            subtitle: self.subtitle,
        }
    }
}

impl IntoModel<LocalizedLocationNames> for Vec<LocationNameData> {
    fn into_model(self, data: &RawData) -> LocalizedLocationNames {
        let names = self
            .iter()
            .map(|name| (name.local_language_id, name.clone().into_model(data)))
            .collect();
        LocalizedLocationNames::new(names)
    }
}
