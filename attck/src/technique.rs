use serde::Deserialize;
use stix::{CommonProperties, KillChainPhase, Object};

use crate::{store::StoreLinked, store_linked, RelationshipType, Store};

#[derive(Deserialize)]
pub struct Data {
    #[serde(flatten)]
    base: CommonProperties,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    kill_chain_phases: Vec<KillChainPhase>,
}

impl AsRef<CommonProperties> for Data {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

store_linked!(Data, Technique);

pub struct Technique<'a> {
    data: &'a Data,
    store: &'a Store,
}

impl<'a> Technique<'a> {
    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn description(&self) -> Option<&str> {
        self.data.description.as_ref().map(|s| s.as_str())
    }

    pub fn kill_chain_phases(&self) -> &[KillChainPhase] {
        &self.data.kill_chain_phases
    }

    pub fn subtechniques(&'a self) -> impl Iterator<Item = Technique<'a>> {
        self.store
            .edges_directed(self.id(), petgraph::EdgeDirection::Incoming)
            .filter(|e| e.relationship_type == RelationshipType::SubtechniqueOf)
            .map(move |e| self.store.get_indexed().techniques_by_id[&e.source_ref].link(self.store))
    }
}

impl<'a> AsRef<CommonProperties> for Technique<'a> {
    fn as_ref(&self) -> &CommonProperties {
        &self.data.base
    }
}
