use std::collections::HashMap;
use strsim::jaro;

pub struct StringSearchIndex<V> {
    dictionary: HashMap<String, V>,
}

impl<V> StringSearchIndex<V> {
    pub fn new(dictionary: HashMap<String, V>) -> Self {
        Self { dictionary }
    }

    pub fn fuzzy_search(&self, query: &str, threshold: f64) -> Vec<(&V, f64)> {
        let mut results = Vec::new();

        for (key, value) in &self.dictionary {
            let similarity = jaro(query, key);
            if similarity >= threshold {
                results.push((value, similarity));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results
    }

    pub fn fuzzy_search_single(&self, query: &str, threshold: f64) -> Option<&V> {
        let results = self.fuzzy_search(query, threshold);
        results.first().map(|r| r.0)
    }
}
