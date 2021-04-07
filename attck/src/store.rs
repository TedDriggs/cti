use std::{collections::HashMap, convert::TryFrom};

use crate::{
    actor,
    bundle::{self, Bundle},
    malware, relationship, technique, tool, Actor, Malware, Relationship, Technique, Tool,
};
use once_self_cell::sync_once_self_cell;
use petgraph::{
    graph::{Edges, NodeIndex},
    EdgeDirection, Graph,
};
use stix::{Id, Object};

struct Data {
    actors: Vec<actor::Data>,
    malwares: Vec<malware::Data>,
    relationships: Vec<relationship::Data>,
    techniques: Vec<technique::Data>,
    tools: Vec<tool::Data>,
}

impl From<Bundle> for Data {
    fn from(v: Bundle) -> Self {
        let mut actors = vec![];
        let mut malwares = vec![];
        let mut relationships = vec![];
        let mut techniques = vec![];
        let mut tools = vec![];

        for object in v.objects {
            match object {
                bundle::Object::Actor(a) => actors.push(a),
                bundle::Object::Malware(m) => malwares.push(m),
                bundle::Object::Relationship(r) => relationships.push(r),
                bundle::Object::Technique(t) => techniques.push(t),
                bundle::Object::Tool(t) => tools.push(t),
                _ => {}
            }
        }

        Data {
            actors,
            malwares,
            relationships,
            techniques,
            tools,
        }
    }
}

pub(crate) struct Index<'a> {
    pub(crate) actors_by_id: HashMap<&'a Id, &'a actor::Data>,
    pub(crate) malwares_by_id: HashMap<&'a Id, &'a malware::Data>,
    pub(crate) techniques_by_id: HashMap<&'a Id, &'a technique::Data>,
    pub(crate) tools_by_id: HashMap<&'a Id, &'a tool::Data>,
    graph_indices: HashMap<&'a Id, NodeIndex>,
    relationships: Graph<&'a Id, &'a relationship::Data>,
}

impl<'a> From<&'a Data> for Index<'a> {
    fn from(v: &'a Data) -> Self {
        let mut graph_indices = HashMap::new();
        let mut graph = Graph::new();
        for rel in &v.relationships {
            let source_idx = *graph_indices
                .entry(&rel.source_ref)
                .or_insert_with(|| graph.add_node(&rel.source_ref));
            let target_idx = *graph_indices
                .entry(&rel.target_ref)
                .or_insert_with(|| graph.add_node(&rel.target_ref));

            graph.add_edge(source_idx, target_idx, rel);
        }

        Self {
            graph_indices,
            actors_by_id: key_by_id(&v.actors),
            malwares_by_id: key_by_id(&v.malwares),
            techniques_by_id: key_by_id(&v.techniques),
            tools_by_id: key_by_id(&v.tools),
            relationships: graph,
        }
    }
}

sync_once_self_cell!(StoreIndex, Data, Index<'_>);

pub struct Store {
    inner: StoreIndex,
}

impl Store {
    pub fn actors<'a>(&'a self) -> impl Iterator<Item = Actor<'a>> {
        self.link(&self.data().actors)
    }

    pub fn malwares<'a>(&'a self) -> impl Iterator<Item = Malware<'a>> {
        self.link(&self.data().malwares)
    }

    pub fn relationships<'a>(&'a self) -> impl Iterator<Item = Relationship<'a>> {
        self.link(&self.data().relationships)
    }

    pub fn techniques<'a>(&'a self) -> impl Iterator<Item = Technique<'a>> {
        self.link(&self.data().techniques)
    }

    pub fn tools<'a>(&'a self) -> impl Iterator<Item = Tool<'a>> {
        self.link(&self.data().tools)
    }
}

impl Store {
    fn link<'a, D: 'a + StoreLinked<'a>>(
        &'a self,
        items: impl IntoIterator<Item = &'a D>,
    ) -> impl Iterator<Item = <D as StoreLinked>::Linked> {
        items.into_iter().map(move |d| d.link(self))
    }

    fn data(&self) -> &Data {
        self.inner.get_owner()
    }

    pub(crate) fn get_indexed(&self) -> &Index {
        self.inner.get_or_init_dependent()
    }

    pub(crate) fn node_index(&self, id: &Id) -> Option<NodeIndex> {
        self.inner
            .get_or_init_dependent()
            .graph_indices
            .get(id)
            .copied()
    }

    pub(crate) fn rel_graph(&self) -> &Graph<&Id, &relationship::Data> {
        &self.inner.get_or_init_dependent().relationships
    }

    pub(crate) fn edges_directed(
        &self,
        id: &Id,
        direction: EdgeDirection,
    ) -> impl Iterator<Item = &relationship::Data> {
        match self.node_index(id) {
            None => EdgeIter::Empty,
            Some(idx) => EdgeIter::Edges(self.rel_graph().edges_directed(idx, direction)),
        }
    }
}

impl TryFrom<Bundle> for Store {
    type Error = ();

    fn try_from(value: Bundle) -> Result<Self, Self::Error> {
        Ok(Store::from(Data::from(value)))
    }
}

impl From<Data> for Store {
    fn from(v: Data) -> Self {
        Self {
            inner: StoreIndex::new(v),
        }
    }
}

enum EdgeIter<'a> {
    Empty,
    Edges(Edges<'a, &'a relationship::Data, petgraph::Directed>),
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = &'a relationship::Data;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EdgeIter::Empty => None,
            EdgeIter::Edges(ref mut edges) => edges.next().map(|e| *e.weight()),
        }
    }
}

pub(crate) trait StoreLinked<'store> {
    type Linked;

    fn link(&'store self, store: &'store Store) -> Self::Linked;
}

#[macro_export]
macro_rules! store_linked {
    ($data:ident, $linked:ident) => {
        impl<'store> $crate::store::StoreLinked<'store> for $data {
            type Linked = $linked<'store>;

            fn link(&'store self, store: &'store $crate::store::Store) -> Self::Linked {
                $linked { data: self, store }
            }
        }
    };
}

fn key_by_id<'a, D: 'a + Object>(items: impl IntoIterator<Item = &'a D>) -> HashMap<&'a Id, &'a D> {
    items.into_iter().map(|i| (i.id(), i)).collect()
}
