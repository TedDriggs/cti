use std::{
    collections::HashMap,
    ops::{Deref, Index},
};

use indexmap::IndexMap;
use once_self_cell::sync_once_self_cell;
use petgraph::{
    graph::{EdgeReference, NodeIndex},
    EdgeDirection, Graph,
};

use crate::{
    attack_pattern, intrusion_set, malware,
    relationship::{self, Filter},
    tool, Bundle, CommonProperties, Declaration, Id, Object, ObjectType, RelationshipType,
};

#[derive(Default)]
pub struct CollectionBuilder {
    attack_patterns: IndexMap<Id, attack_pattern::Data>,
    intrusion_sets: IndexMap<Id, intrusion_set::Data>,
    malwares: IndexMap<Id, malware::Data>,
    relationships: IndexMap<Id, relationship::Data>,
    tools: IndexMap<Id, tool::Data>,
}

impl CollectionBuilder {
    pub fn add_bundle(&mut self, bundle: Bundle) {
        for item in bundle.objects {
            match item {
                Declaration::Bundle => {
                    panic!("What does a nested bundle mean?");
                }
                Declaration::Identity(_)
                | Declaration::CourseOfAction(_)
                | Declaration::MarkingDefinition
                | Declaration::XMitreMatrix
                | Declaration::XMitreTactic => {}
                Declaration::AttackPattern(v) => {
                    self.attack_patterns.insert(v.id().clone(), v);
                }
                Declaration::IntrusionSet(v) => {
                    self.intrusion_sets.insert(v.id().clone(), v);
                }
                Declaration::Malware(v) => {
                    self.malwares.insert(v.id().clone(), v);
                }
                Declaration::Relationship(v) => {
                    self.relationships.insert(v.id().clone(), v);
                }
                Declaration::Tool(v) => {
                    self.tools.insert(v.id().clone(), v);
                }
            }
        }
    }

    pub fn build(self) -> Collection {
        Collection {
            data: CollectionData::new(self),
        }
    }
}

struct Indexed<'a> {
    id_nodes: HashMap<&'a Id, NodeIndex>,
    relationship_graph: Graph<&'a Id, &'a relationship::Data>,
}

impl<'a> From<&'a CollectionBuilder> for Indexed<'a> {
    fn from(v: &'a CollectionBuilder) -> Self {
        let mut id_nodes = HashMap::new();
        let mut relationship_graph = Graph::new();

        for relationship in v.relationships.values() {
            let source_idx = *id_nodes
                .entry(&relationship.source_ref)
                .or_insert_with_key(|k| relationship_graph.add_node(*k));

            let target_idx = *id_nodes
                .entry(&relationship.target_ref)
                .or_insert_with_key(|k| relationship_graph.add_node(*k));
            relationship_graph.add_edge(source_idx, target_idx, relationship);
        }

        Self {
            id_nodes,
            relationship_graph,
        }
    }
}

sync_once_self_cell!(CollectionData, CollectionBuilder, Indexed<'_>);

pub struct Collection {
    data: CollectionData,
}

macro_rules! typed_collection {
    ($field:ident, $data:ty) => {
        pub fn $field<'a>(&'a self) -> TypedCollection<'a, $data> {
            TypedCollection::new(self, &self.data().$field)
        }
    };
}

impl Collection {
    typed_collection!(intrusion_sets, intrusion_set::Data);
    typed_collection!(malwares, malware::Data);
    typed_collection!(tools, tool::Data);
}

impl Collection {
    fn data<'a>(&'a self) -> &'a CollectionBuilder {
        self.data.get_owner()
    }

    fn indexed<'a>(&'a self) -> &'a Indexed<'a> {
        self.data.get_or_init_dependent()
    }

    fn node_index(&self, id: &Id) -> Option<NodeIndex> {
        self.indexed().id_nodes.get(id).copied()
    }

    fn edges_directed<'a>(
        &'a self,
        id: &Id,
        dir: EdgeDirection,
    ) -> impl Iterator<Item = &'a relationship::Data> {
        match self.node_index(id) {
            None => EdgeIter::Empty,
            Some(idx) => {
                EdgeIter::Edges(self.indexed().relationship_graph.edges_directed(idx, dir))
            }
        }
    }

    fn peers_matching<'a>(
        &'a self,
        id: &Id,
        filter: relationship::Filter,
    ) -> impl Iterator<Item = &'a Id> {
        self.edges_directed(id, filter.direction)
            .filter_map(move |d| {
                if *d == filter {
                    match filter.direction {
                        EdgeDirection::Incoming => Some(&d.source_ref),
                        EdgeDirection::Outgoing => Some(&d.target_ref),
                    }
                } else {
                    None
                }
            })
    }
}

impl From<Bundle> for Collection {
    fn from(v: Bundle) -> Self {
        let mut builder = CollectionBuilder::default();
        builder.add_bundle(v);
        builder.build()
    }
}

enum EdgeIter<E> {
    Empty,
    Edges(E),
}

impl<'a, E: Iterator<Item = EdgeReference<'a, &'a relationship::Data>>> Iterator for EdgeIter<E> {
    type Item = &'a relationship::Data;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EdgeIter::Empty => None,
            EdgeIter::Edges(ref mut edges) => edges.next().map(|e| *e.weight()),
        }
    }
}

pub struct TypedCollection<'a, D> {
    collection: &'a Collection,
    backer: &'a IndexMap<Id, D>,
    iter: indexmap::map::Values<'a, Id, D>,
}

impl<'a, D> TypedCollection<'a, D> {
    fn new(collection: &'a Collection, backer: &'a IndexMap<Id, D>) -> Self {
        Self {
            collection,
            backer,
            iter: backer.values(),
        }
    }
}

impl<'a, D> Iterator for TypedCollection<'a, D> {
    type Item = Attached<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|data| Attached {
            data,
            coll: self.collection,
        })
    }
}

impl<'id, 'a, D> Index<&'id Id> for TypedCollection<'a, D> {
    type Output = D;

    fn index(&self, index: &'id Id) -> &Self::Output {
        &self.backer[index]
    }
}

macro_rules! rel {
    ($name:ident, $filter:expr, $data:ty) => {
        rel!($name, $filter, $data, $name);
    };
    ($name:ident, $filter:expr, $data:ty, $coll:ident) => {
        pub fn $name(&'a self) -> impl Iterator<Item = Attached<'a, $data>> {
            self.coll
                .peers_matching(self.id(), $filter)
                .map(move |d: &Id| self.create(&self.coll.data().$coll[d]))
        }
    };
}

pub struct Attached<'a, D> {
    data: &'a D,
    coll: &'a Collection,
}

impl<'a, D> Attached<'a, D> {
    fn create<T>(&'a self, data: &'a T) -> Attached<'a, T> {
        Attached {
            data,
            coll: self.coll,
        }
    }

    pub fn data(&self) -> &'a D {
        self.data
    }
}

impl<'a, D> Deref for Attached<'a, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, D: AsRef<CommonProperties>> AsRef<CommonProperties> for Attached<'a, D> {
    fn as_ref(&self) -> &CommonProperties {
        self.data.as_ref()
    }
}

impl<'a> Attached<'a, attack_pattern::Data> {
    rel!(
        subtechniques,
        Filter::incoming(RelationshipType::SubtechniqueOf, ObjectType::AttackPattern),
        attack_pattern::Data,
        attack_patterns
    );
}

impl<'a> Attached<'a, intrusion_set::Data> {
    rel!(
        tools,
        Filter::outgoing(RelationshipType::Uses, ObjectType::Tool),
        tool::Data
    );
    rel!(
        attack_patterns,
        Filter::outgoing(RelationshipType::Uses, ObjectType::AttackPattern),
        attack_pattern::Data
    );
    rel!(
        malwares,
        Filter::outgoing(RelationshipType::Uses, ObjectType::Malware),
        malware::Data
    );
}

impl<'a> Attached<'a, malware::Data> {
    rel!(
        intrusion_sets,
        Filter::incoming(RelationshipType::Uses, ObjectType::IntrusionSet),
        intrusion_set::Data
    );

    rel!(
        attack_patterns,
        Filter::incoming(RelationshipType::Uses, ObjectType::AttackPattern),
        attack_pattern::Data
    );
}

impl<'a> Attached<'a, tool::Data> {
    rel!(
        intrusion_sets,
        Filter::incoming(RelationshipType::Uses, ObjectType::IntrusionSet),
        intrusion_set::Data
    );
}
