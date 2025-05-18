use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::localized_effects::{LocalizedEffect, LocalizedEffects};

pub trait HasLocalizedEffects {
    fn language(&self) -> LanguageId;
    fn effect(&self) -> String;
    fn short_effect(&self) -> String;
}

impl<T> IntoModel<LocalizedEffect> for T
where
    T: HasLocalizedEffects,
{
    fn into_model(self, _data: &RawData) -> LocalizedEffect {
        LocalizedEffect {
            effect: self.effect(),
            short_effect: self.short_effect(),
        }
    }
}

impl<T> IntoModel<LocalizedEffects> for Vec<T>
where
    T: HasLocalizedEffects,
{
    fn into_model(self, data: &RawData) -> LocalizedEffects {
        let effects = self
            .into_iter()
            .map(|entry| (entry.language(), entry.into_model(data)))
            .collect();
        LocalizedEffects::new(effects)
    }
}
