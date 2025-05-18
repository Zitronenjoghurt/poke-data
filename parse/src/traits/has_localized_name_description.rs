use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::localized_name_descriptions::{
    LocalizedNameDescription, LocalizedNameDescriptions,
};

pub trait HasLocalizedNameDescription {
    fn language(&self) -> LanguageId;
    fn name(&self) -> String;
    fn description(&self) -> String;
}

impl<T> IntoModel<LocalizedNameDescriptions> for Vec<T>
where
    T: HasLocalizedNameDescription,
{
    fn into_model(self, _data: &RawData) -> LocalizedNameDescriptions {
        let localizations = self
            .iter()
            .map(|data| {
                (
                    data.language(),
                    LocalizedNameDescription {
                        name: data.name(),
                        description: data.description(),
                    },
                )
            })
            .collect();
        LocalizedNameDescriptions::new(localizations)
    }
}
