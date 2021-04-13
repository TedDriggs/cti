use std::ops::{Deref, Index};

use indexmap::IndexMap;
use once_self_cell::sync_once_self_cell;
use serde::Deserialize;

use crate::{
    relationship::{self, Filter},
    AttackPattern, Bundle, CommonProperties, CourseOfAction, Id, Identity, IntrusionSet, Malware,
    Object, Relationship, RelationshipGraph, RelationshipType, Tool,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Declaration {
    AttackPattern(AttackPattern),
    CourseOfAction(CourseOfAction),
    Identity(Identity),
    IntrusionSet(IntrusionSet),
    Malware(Malware),
    MarkingDefinition,
    Relationship(Relationship),
    Tool(Tool),
    XMitreMatrix,
    XMitreTactic,
}

#[derive(Default)]
pub struct CollectionBuilder {
    attack_patterns: IndexMap<Id, AttackPattern>,
    courses_of_action: IndexMap<Id, CourseOfAction>,
    identities: IndexMap<Id, Identity>,
    intrusion_sets: IndexMap<Id, IntrusionSet>,
    malwares: IndexMap<Id, Malware>,
    relationships: IndexMap<Id, Relationship>,
    tools: IndexMap<Id, Tool>,
}

impl CollectionBuilder {
    pub fn add_bundle(&mut self, bundle: Bundle<Declaration>) {
        for item in bundle.objects {
            match item {
                Declaration::AttackPattern(v) => {
                    self.attack_patterns.insert(v.id().clone(), v);
                }
                Declaration::CourseOfAction(v) => {
                    self.courses_of_action.insert(v.id().clone(), v);
                }
                Declaration::Identity(v) => {
                    self.identities.insert(v.id().clone(), v);
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
                Declaration::MarkingDefinition
                | Declaration::XMitreMatrix
                | Declaration::XMitreTactic => {}
            }
        }
    }

    pub fn build(self) -> Collection {
        Collection {
            data: CollectionData::new(self),
        }
    }
}

impl<'a> Into<RelationshipGraph<'a>> for &'a CollectionBuilder {
    fn into(self) -> RelationshipGraph<'a> {
        self.relationships.values().collect()
    }
}

sync_once_self_cell!(CollectionData, CollectionBuilder, RelationshipGraph<'_>);

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
    typed_collection!(attack_patterns, AttackPattern);
    typed_collection!(courses_of_action, CourseOfAction);
    typed_collection!(identities, Identity);
    typed_collection!(intrusion_sets, IntrusionSet);
    typed_collection!(malwares, Malware);
    typed_collection!(relationships, Relationship);
    typed_collection!(tools, Tool);
}

impl Collection {
    fn data<'a>(&'a self) -> &'a CollectionBuilder {
        self.data.get_owner()
    }

    fn graph<'a>(&'a self) -> &'a RelationshipGraph<'a> {
        self.data.get_or_init_dependent()
    }

    fn peers_matching<'a>(
        &'a self,
        id: &Id,
        filter: relationship::Filter,
    ) -> impl Iterator<Item = &'a Id> {
        self.graph().peers_matching(id, filter)
    }
}

impl From<Bundle<Declaration>> for Collection {
    fn from(v: Bundle<Declaration>) -> Self {
        let mut builder = CollectionBuilder::default();
        builder.add_bundle(v);
        builder.build()
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
    ($name:ident -> $rel:expr, $data:ty) => {
        rel!($name, Filter::outgoing::<$data>($rel), $data, $name);
    };
    ($name:ident <- $rel:expr, $data:ty) => {
        rel!($name, Filter::incoming::<$data>($rel), $data, $name);
    };
    ($name:ident -> $rel:expr, $data:ty, $coll:ident) => {
        rel!($name, Filter::outgoing::<$data>($rel), $data, $coll);
    };
    ($name:ident <- $rel:expr, $data:ty, $coll:ident) => {
        rel!($name, Filter::incoming::<$data>($rel), $data, $coll);
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

impl<'a, D: Object> Node<'a, D> {
    pub fn created_by(&'a self) -> Option<Node<'a, Identity>> {
        Some(self.create(&self.coll.data().identities[self.created_by_ref()?]))
    }
}

impl<'a> Node<'a, AttackPattern> {
    rel!(subtechniques <- RelationshipType::SubtechniqueOf, AttackPattern, attack_patterns);
}

impl<'a> Node<'a, CourseOfAction> {
    rel!(mitigates_attack_patterns -> RelationshipType::Mitigates, AttackPattern, attack_patterns);
    rel!(mitigates_malwares -> RelationshipType::Mitigates, Malware, malwares);
    rel!(mitigates_tools -> RelationshipType::Mitigates, Tool, tools);
}

impl<'a> Node<'a, IntrusionSet> {
    rel!(attack_patterns -> RelationshipType::Uses, AttackPattern);
    rel!(malwares -> RelationshipType::Uses, Malware);
    rel!(tools -> RelationshipType::Uses, Tool);
}

impl<'a> Node<'a, Malware> {
    rel!(attack_patterns <- RelationshipType::Uses, AttackPattern);
    rel!(intrusion_sets <- RelationshipType::Uses, IntrusionSet);
}

impl<'a> Node<'a, Tool> {
    rel!(intrusion_sets <- RelationshipType::Uses, IntrusionSet);
}
