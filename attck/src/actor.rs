use std::collections::BTreeSet;

use crate::{store::StoreLinked, store_linked, Malware, RelationshipType, Store, Technique, Tool};

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
    #[serde(default)]
    aliases: BTreeSet<String>,
}

impl AsRef<CommonProperties> for Data {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

store_linked!(Data, Actor);

pub struct Actor<'a> {
    pub(crate) data: &'a Data,
    pub(crate) store: &'a Store,
}

impl<'a> Actor<'a> {
    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn description(&self) -> Option<&str> {
        self.data.description.as_ref().map(|s| s.as_str())
    }

    pub fn aliases(&self) -> &BTreeSet<String> {
        &self.data.aliases
    }

    pub fn malwares(&'a self) -> impl Iterator<Item = Malware<'a>> {
        self.store
            .edges_directed(self.id(), EdgeDirection::Outgoing)
            .filter(|r| {
                r.relationship_type == RelationshipType::Uses
                    && r.target_ref.namespace() == "malware"
            })
            .map(move |r| self.store.get_indexed().malwares_by_id[&r.target_ref].link(self.store))
    }

    pub fn techniques(&'a self) -> impl Iterator<Item = Technique<'a>> {
        self.store
            .edges_directed(self.id(), EdgeDirection::Outgoing)
            .filter(|r| {
                r.relationship_type == RelationshipType::Uses
                    && r.target_ref.namespace() == "attack-pattern"
            })
            .map(move |r| self.store.get_indexed().techniques_by_id[&r.target_ref].link(self.store))
    }

    pub fn tools(&'a self) -> impl Iterator<Item = Tool<'a>> {
        self.store
            .edges_directed(self.id(), EdgeDirection::Outgoing)
            .filter(|r| {
                r.relationship_type == RelationshipType::Uses && r.target_ref.namespace() == "tool"
            })
            .map(move |r| self.store.get_indexed().tools_by_id[&r.target_ref].link(self.store))
    }
}

impl<'a> AsRef<CommonProperties> for Actor<'a> {
    fn as_ref(&self) -> &CommonProperties {
        &self.data.base
    }
}
