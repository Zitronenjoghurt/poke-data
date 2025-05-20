use crate::data::link_context::LinkContext;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Linkable {
    type Linked;
    fn link(&self, context: &LinkContext) -> Self::Linked;
}

impl<K, V> Linkable for HashMap<K, V>
where
    K: Eq + Hash + Copy,
    V: Linkable,
{
    type Linked = HashMap<K, V::Linked>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        self.iter()
            .map(|(id, item)| (*id, item.link(context)))
            .collect()
    }
}

impl<V> Linkable for Vec<V>
where
    V: Linkable,
{
    type Linked = Vec<V::Linked>;
    fn link(&self, context: &LinkContext) -> Self::Linked {
        self.iter().map(|item| item.link(context)).collect()
    }
}
