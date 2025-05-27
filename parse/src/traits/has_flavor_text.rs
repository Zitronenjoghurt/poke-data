use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::flavor_texts::{FlavorText, FlavorTexts};
use poke_data::models::language::LanguageId;
use poke_data::models::version_group::VersionGroupId;
use std::collections::HashMap;

pub trait HasFlavorText {
    fn language(&self) -> LanguageId;
    fn version_group(&self) -> VersionGroupId;
    fn text(&self) -> String;
}

impl<T> IntoModel<FlavorText> for T
where
    T: HasFlavorText,
{
    fn into_model(self, _data: &RawData) -> FlavorText {
        FlavorText {
            version_group_id: self.version_group(),
            flavor_text: self.text(),
        }
    }
}

impl<T> IntoModel<FlavorTexts> for Vec<T>
where
    T: HasFlavorText,
{
    fn into_model(self, data: &RawData) -> FlavorTexts {
        let texts = self.into_iter().fold(
            HashMap::new(),
            |mut map: HashMap<LanguageId, Vec<FlavorText>>, entry| {
                let language_id = entry.language();
                let flavor_text = entry.into_model(data);
                map.entry(language_id).or_default().push(flavor_text);
                map
            },
        );
        FlavorTexts::new(texts)
    }
}
