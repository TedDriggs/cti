use crate::{store::StoreLinked, store_linked, Actor, RelationshipType, Store};

use petgraph::EdgeDirection;
use serde::Deserialize;
use stix::{CommonProperties, Object};

#[derive(Deserialize)]
pub struct Data {
    #[serde(flatten)]
    base: CommonProperties,
    name: String,
    #[serde(default)]
    description: Option<String>,
}

impl AsRef<CommonProperties> for Data {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

store_linked!(Data, Tool);

pub struct Tool<'a> {
    data: &'a Data,
    store: &'a Store,
}

impl<'a> Tool<'a> {
    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn description(&self) -> Option<&str> {
        self.data.description.as_ref().map(|s| s.as_str())
    }

    pub fn actors(&'a self) -> impl Iterator<Item = Actor<'a>> {
        self.store
            .edges_directed(self.id(), EdgeDirection::Incoming)
            .filter(|e| e.relationship_type == RelationshipType::Uses)
            .map(move |e| self.store.get_indexed().actors_by_id[&e.source_ref].link(self.store))
    }
}

impl<'a> AsRef<CommonProperties> for Tool<'a> {
    fn as_ref(&self) -> &CommonProperties {
        &self.data.base
    }
}
