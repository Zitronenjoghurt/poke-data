use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::localized_names::LocalizedNames;

pub trait HasLocalizedName {
    fn language(&self) -> LanguageId;
    fn name(&self) -> String;
}

impl<T> IntoModel<LocalizedNames> for Vec<T>
where
    T: HasLocalizedName,
{
    fn into_model(self, _data: &RawData) -> LocalizedNames {
        let localizations = self
            .iter()
            .map(|data| (data.language(), data.name()))
            .collect();
        LocalizedNames::new(localizations)
    }
}
