mod attack_pattern;
mod bundle;
mod campaign;
mod collection;
mod course_of_action;
mod id;
pub mod identity;
mod infrastructure;
mod intrusion_set;
pub mod location;
mod malware;
mod marking_definition;
mod object;
mod reference;
pub mod relationship;
mod relationship_graph;
mod threat_actor;
mod tool;
mod vulnerability;

pub use attack_pattern::AttackPattern;
pub use bundle::Bundle;
pub use campaign::Campaign;
pub use collection::{Collection, Declaration, Node, TypedCollection};
pub use course_of_action::CourseOfAction;
pub use id::{Id, IdParseError};
#[doc(inline)]
pub use identity::Identity;
pub use infrastructure::Infrastructure;
pub use intrusion_set::IntrusionSet;
pub use location::Location;
pub use malware::Malware;
pub use marking_definition::MarkingDefinition;
pub use object::{CommonProperties, Object, TypedObject};
pub use reference::{ExternalReference, KillChainPhase};
pub use relationship::{Relationship, RelationshipType};
pub use relationship_graph::RelationshipGraph;
pub use threat_actor::ThreatActor;
pub use tool::Tool;
pub use vulnerability::Vulnerability;

pub use stix_derive::*;

#[doc(hidden)]
pub mod export {
    pub use indexmap::IndexMap;
    pub use once_self_cell::sync_once_self_cell;
    pub mod petgraph {
        pub use ::petgraph::{graph::NodeIndex, Graph};
    }
}

/// Trait for turning a reference in a STIX collection into a data-carrying node.
pub trait Resolve {
    /// The node type, containing a reference to the data and the backing collection.
    type Output;

    /// Produce a collection-attached node for the object identified by the ID.
    fn resolve(self) -> Option<Self::Output>;
}
