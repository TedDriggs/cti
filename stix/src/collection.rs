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
    relationship::{self, Filter},
    AttackPattern, Bundle, CommonProperties, Declaration, Id, IntrusionSet, Malware, Object,
    ObjectType, Relationship, RelationshipType, Tool,
};

#[derive(Default)]
pub struct CollectionBuilder {
    attack_patterns: IndexMap<Id, AttackPattern>,
    intrusion_sets: IndexMap<Id, IntrusionSet>,
    malwares: IndexMap<Id, Malware>,
    relationships: IndexMap<Id, Relationship>,
    tools: IndexMap<Id, Tool>,
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
    relationship_graph: Graph<&'a Id, &'a Relationship>,
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
    typed_collection!(intrusion_sets, IntrusionSet);
    typed_collection!(malwares, Malware);
    typed_collection!(tools, Tool);
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
    ) -> impl Iterator<Item = &'a Relationship> {
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

impl<'a, E: Iterator<Item = EdgeReference<'a, &'a Relationship>>> Iterator for EdgeIter<E> {
    type Item = &'a Relationship;

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
    type Item = Node<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|data| Node {
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
        pub fn $name(&'a self) -> impl Iterator<Item = Node<'a, $data>> {
            self.coll
                .peers_matching(self.id(), $filter)
                .map(move |d: &Id| self.create(&self.coll.data().$coll[d]))
        }
    };
}

/// A STIX domain object in an in-memory `Collection`.
///
/// This allows easy traversal of the threat intelligence graph.
pub struct Node<'a, D> {
    data: &'a D,
    coll: &'a Collection,
}

impl<'a, D> Node<'a, D> {
    fn create<T>(&'a self, data: &'a T) -> Node<'a, T> {
        Node {
            data,
            coll: self.coll,
        }
    }

    pub fn data(&self) -> &'a D {
        self.data
    }
}

impl<'a, D> Deref for Node<'a, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, D: AsRef<CommonProperties>> AsRef<CommonProperties> for Node<'a, D> {
    fn as_ref(&self) -> &CommonProperties {
        self.data.as_ref()
    }
}

impl<'a> Node<'a, AttackPattern> {
    rel!(
        subtechniques,
        Filter::incoming(RelationshipType::SubtechniqueOf, ObjectType::AttackPattern),
        AttackPattern,
        attack_patterns
    );
}

impl<'a> Node<'a, IntrusionSet> {
    rel!(
        tools,
        Filter::outgoing(RelationshipType::Uses, ObjectType::Tool),
        Tool
    );
    rel!(
        attack_patterns,
        Filter::outgoing(RelationshipType::Uses, ObjectType::AttackPattern),
        AttackPattern
    );
    rel!(
        malwares,
        Filter::outgoing(RelationshipType::Uses, ObjectType::Malware),
        Malware
    );
}

impl<'a> Node<'a, Malware> {
    rel!(
        intrusion_sets,
        Filter::incoming(RelationshipType::Uses, ObjectType::IntrusionSet),
        IntrusionSet
    );

    rel!(
        attack_patterns,
        Filter::incoming(RelationshipType::Uses, ObjectType::AttackPattern),
        AttackPattern
    );
}

impl<'a> Node<'a, Tool> {
    rel!(
        intrusion_sets,
        Filter::incoming(RelationshipType::Uses, ObjectType::IntrusionSet),
        IntrusionSet
    );
}
