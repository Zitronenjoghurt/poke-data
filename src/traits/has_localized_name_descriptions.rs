use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use std::collections::HashMap;
use std::sync::Arc;

pub trait HasLocalizedNameDescriptions {
    fn localized_name_descriptions(&self) -> &LocalizedNameDescriptions;
}

pub trait LocalizedNameDescriptionDictionary<K, V> {
    fn build_localized_name_dictionary(&self) -> HashMap<String, K>;
}

impl<K, E> LocalizedNameDescriptionDictionary<K, String> for HashMap<K, Arc<E>>
where
    K: Clone,
    E: HasLocalizedNameDescriptions,
{
    fn build_localized_name_dictionary(&self) -> HashMap<String, K> {
        let mut result = HashMap::new();

        for (key, entity) in self {
            for entry in entity
                .localized_name_descriptions()
                .localizations()
                .values()
            {
                result.insert(entry.name.clone(), key.clone());
            }
        }

        result
    }
}
