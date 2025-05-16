use crate::RawData;
use std::collections::HashMap;

pub trait IntoModel<T> {
    fn into_model(self, data: &RawData) -> T;
}

impl<T, M> IntoModel<Vec<M>> for Vec<T>
where
    T: IntoModel<M>,
{
    fn into_model(self, data: &RawData) -> Vec<M> {
        self.into_iter().map(|item| item.into_model(data)).collect()
    }
}

impl<K, T, M> IntoModel<HashMap<K, M>> for HashMap<K, T>
where
    K: Clone + Eq + std::hash::Hash,
    T: IntoModel<M>,
{
    fn into_model(self, data: &RawData) -> HashMap<K, M> {
        self.into_iter()
            .map(|(k, v)| (k.clone(), v.into_model(data)))
            .collect()
    }
}
