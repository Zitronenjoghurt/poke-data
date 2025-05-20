use crate::data::link_context::LinkContext;
use crate::data_structures::string_search_index::StringSearchIndex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

pub trait EntityCollection<K, E>
where
    K: Hash + Eq + Clone + 'static,
    E: 'static,
{
    fn new(link_context: &LinkContext) -> Self;
    fn entities(&self) -> &HashMap<K, Arc<E>>;

    fn get<Q>(&self, key: &Q) -> Option<&Arc<E>>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        self.entities().get(key)
    }

    fn iter(&self) -> impl Iterator<Item = &Arc<E>> {
        self.entities().values()
    }
}

pub trait HasNameSearchIndex<K, E>: EntityCollection<K, E>
where
    K: Hash + Eq + Clone + 'static,
    E: 'static,
{
    fn name_search_index(&self) -> &StringSearchIndex<K>;

    fn find_by_name(&self, name: &str, threshold: f64) -> Option<&Arc<E>> {
        let key = self
            .name_search_index()
            .fuzzy_search_single(name, threshold)?;
        self.get(key)
    }
}
