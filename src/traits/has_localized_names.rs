use crate::models::localized_names::LocalizedStrings;
use std::collections::HashMap;
use std::sync::Arc;

pub trait HasLocalizedNames {
    fn localized_names(&self) -> &LocalizedStrings;
}

pub trait LocalizedNamesDictionary<K, V> {
    fn build_localized_name_dictionary(&self) -> HashMap<String, K>;
}

impl<K, E> LocalizedNamesDictionary<K, String> for HashMap<K, Arc<E>>
where
    K: Clone,
    E: HasLocalizedNames,
{
    fn build_localized_name_dictionary(&self) -> HashMap<String, K> {
        let mut result = HashMap::new();

        for (key, entity) in self {
            for name in entity.localized_names().localizations().values() {
                result.insert(name.clone(), key.clone());
            }
        }

        result
    }
}
