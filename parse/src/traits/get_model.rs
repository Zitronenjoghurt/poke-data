use crate::traits::into_model::IntoModel;
use crate::RawData;
use std::collections::HashMap;
use std::hash::Hash;

pub trait GetModel<K, V> {
    fn get_model<T>(&self, key: &K, data: &RawData) -> T
    where
        V: Default + Clone + IntoModel<T>;
}

impl<K, V> GetModel<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn get_model<T>(&self, key: &K, data: &RawData) -> T
    where
        V: Default + Clone + IntoModel<T>,
    {
        self.get(key).cloned().unwrap_or_default().into_model(data)
    }
}
