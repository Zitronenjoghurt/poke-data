use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

pub trait IntoArcMap<K, V> {
    fn into_arc_map(self) -> HashMap<K, Arc<V>>;
}

impl<K, V> IntoArcMap<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn into_arc_map(self) -> HashMap<K, Arc<V>> {
        self.into_iter().map(|(k, v)| (k, Arc::new(v))).collect()
    }
}
