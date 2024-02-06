use std::collections::HashMap;

use leptos::Attribute;

#[derive(Debug, Clone)]
pub struct Attributes {
    pub map: HashMap<&'static str, Attribute>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: &'static str, v: Attribute) -> Option<Attribute> {
        self.map.insert(k, v)
    }

    pub fn merge(&mut self, mut other: Attributes) {
        self.map.extend(other.map.drain())
    }
}

impl IntoIterator for Attributes {
    type Item = (&'static str, Attribute);

    type IntoIter = std::collections::hash_map::IntoIter<&'static str, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
