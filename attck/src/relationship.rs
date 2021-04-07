use crate::store::{Store, StoreLinked};

use serde::Deserialize;
use stix::{CommonProperties, Id};

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
pub struct Data {
    #[serde(flatten)]
    base: CommonProperties,
    pub source_ref: Id,
    pub target_ref: Id,
    pub relationship_type: RelationshipType,
}

impl AsRef<CommonProperties> for Data {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

impl<'store> StoreLinked<'store> for Data {
    type Linked = Relationship<'store>;

    fn link(&'store self, _store: &'store Store) -> Self::Linked {
        Relationship { data: self }
    }
}

pub struct Relationship<'a> {
    data: &'a Data,
}

impl<'a> Relationship<'a> {
    pub fn source_ref(&self) -> &Id {
        &self.data.source_ref
    }

    pub fn target_ref(&self) -> &Id {
        &self.data.target_ref
    }

    pub fn relationship_type(&self) -> &RelationshipType {
        &self.data.relationship_type
    }
}

impl<'a> AsRef<CommonProperties> for Relationship<'a> {
    fn as_ref(&self) -> &CommonProperties {
        &self.data.base
    }
}

#[cfg(test)]
mod tests {
    use super::RelationshipType;

    #[test]
    fn display_rt() {
        println!("{}", RelationshipType::RevokedBy);
    }
}
