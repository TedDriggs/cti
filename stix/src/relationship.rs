use petgraph::EdgeDirection;
use serde::Deserialize;

use crate::{CommonProperties, Id, ObjectType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RelationshipType {
    Mitigates,
    Uses,
    RevokedBy,
    SubtechniqueOf,
}

#[derive(Deserialize)]
pub struct Relationship {
    #[serde(flatten)]
    base: CommonProperties,
    pub source_ref: Id,
    pub target_ref: Id,
    pub relationship_type: RelationshipType,
}

impl AsRef<CommonProperties> for Relationship {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

pub(crate) struct Filter {
    pub direction: EdgeDirection,
    pub relationship_type: RelationshipType,
    pub peer_type: ObjectType,
}

impl Filter {
    pub fn outgoing(relationship_type: RelationshipType, peer_type: ObjectType) -> Self {
        Filter {
            direction: EdgeDirection::Outgoing,
            relationship_type,
            peer_type,
        }
    }

    pub fn incoming(relationship_type: RelationshipType, peer_type: ObjectType) -> Self {
        Filter {
            direction: EdgeDirection::Incoming,
            relationship_type,
            peer_type,
        }
    }
}

impl PartialEq<Filter> for Relationship {
    fn eq(&self, other: &Filter) -> bool {
        let peer = match other.direction {
            EdgeDirection::Outgoing => &self.target_ref,
            EdgeDirection::Incoming => &self.source_ref,
        };

        *peer == other.peer_type
    }
}
