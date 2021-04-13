use std::borrow::Cow;

use petgraph::EdgeDirection;
use serde::Deserialize;

use crate::{CommonProperties, Id, TypedObject};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RelationshipType {
    AttributedTo,
    AuthoredBy,
    BeaconsTo,
    Compromises,
    Controls,
    Delivers,
    DerivedFrom,
    Downloads,
    Drops,
    DuplicateOf,
    ExfiltratesTo,
    Has,
    Hosts,
    Impersonates,
    Indicates,
    LocatedAt,
    Mitigates,
    OriginatesFrom,
    Owns,
    Remediates,
    RevokedBy,
    SubtechniqueOf,
    Targets,
    Uses,
}

#[derive(Deserialize)]
pub struct Relationship {
    #[serde(flatten)]
    base: CommonProperties,
    pub source_ref: Id,
    pub target_ref: Id,
    pub relationship_type: RelationshipType,
}

impl TypedObject for Relationship {
    const TYPE: &'static str = "relationship";
}

impl AsRef<CommonProperties> for Relationship {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

pub struct Filter {
    pub(crate) direction: EdgeDirection,
    pub(crate) relationship_type: RelationshipType,
    pub(crate) peer_type: Cow<'static, str>,
}

impl Filter {
    pub fn outgoing<Peer: TypedObject>(relationship_type: RelationshipType) -> Self {
        Self {
            direction: EdgeDirection::Outgoing,
            relationship_type,
            peer_type: Cow::Borrowed(Peer::TYPE),
        }
    }

    pub fn incoming<Peer: TypedObject>(relationship_type: RelationshipType) -> Self {
        Filter {
            direction: EdgeDirection::Incoming,
            relationship_type,
            peer_type: Cow::Borrowed(Peer::TYPE),
        }
    }
}

impl Filter {
    pub fn matches(&self, rel: &Relationship) -> bool {
        let peer = match self.direction {
            EdgeDirection::Outgoing => &rel.target_ref,
            EdgeDirection::Incoming => &rel.source_ref,
        };

        rel.relationship_type == self.relationship_type && peer.object_type() == self.peer_type
    }
}
