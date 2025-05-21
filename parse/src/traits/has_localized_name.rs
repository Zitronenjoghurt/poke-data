use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::localized_names::LocalizedStrings;

pub trait HasLocalizedString {
    fn language(&self) -> LanguageId;
    fn string(&self) -> String;
}

impl<T> IntoModel<LocalizedStrings> for Vec<T>
where
    T: HasLocalizedString,
{
    fn into_model(self, _data: &RawData) -> LocalizedStrings {
        let localizations = self
            .iter()
            .map(|data| (data.language(), data.string()))
            .collect();
        LocalizedStrings::new(localizations)
    }
}
