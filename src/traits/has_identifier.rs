use std::collections::HashMap;
use std::sync::Arc;

pub trait HasIdentifier {
    fn identifier(&self) -> &str;
}

pub trait IdentifierDictionary<K, V> {
    fn build_identifier_dictionary(&self) -> HashMap<String, K>;
}

impl<K, E> IdentifierDictionary<K, String> for HashMap<K, Arc<E>>
where
    K: Clone,
    E: HasIdentifier,
{
    fn build_identifier_dictionary(&self) -> HashMap<String, K> {
        let mut result = HashMap::new();

        for (key, entity) in self {
            result.insert(entity.identifier().to_string(), key.clone());
        }

        result
    }
}
