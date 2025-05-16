use std::collections::HashMap;
use std::hash::Hash;

pub trait HasId {
    type Id: Eq + Hash;
    fn id(&self) -> Self::Id;
}

pub trait IntoIdMap<T: HasId> {
    fn into_id_map(self) -> HashMap<T::Id, T>;
    fn into_id_map_grouped(self) -> HashMap<T::Id, Vec<T>>;
}

impl<T: HasId> IntoIdMap<T> for Vec<T> {
    fn into_id_map(self) -> HashMap<T::Id, T> {
        self.into_iter().map(|item| (item.id(), item)).collect()
    }

    fn into_id_map_grouped(self) -> HashMap<T::Id, Vec<T>> {
        self.into_iter().fold(HashMap::new(), |mut map, item| {
            map.entry(item.id()).or_default().push(item);
            map
        })
    }
}
