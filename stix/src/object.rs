use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    attack_pattern, intrusion_set, malware, reference::ExternalReference, relationship, tool, Id,
};

#[derive(Deserialize)]
pub struct CommonProperties {
    id: Id,
    created_by_ref: Option<Id>,
    #[serde(default)]
    revoked: bool,
    #[serde(default)]
    labels: BTreeSet<String>,
    #[serde(default)]
    object_marking_refs: BTreeSet<Id>,
    #[serde(default)]
    external_references: Vec<ExternalReference>,
    #[serde(default)]
    created: Option<DateTime<Utc>>,
    #[serde(default)]
    modified: Option<DateTime<Utc>>,
}

/// Common properties for a STIX Domain Object.
pub trait Object {
    fn id(&self) -> &Id;
    fn created_by_ref(&self) -> Option<&Id>;
    fn revoked(&self) -> bool;
    fn labels(&self) -> &BTreeSet<String>;
    fn object_marking_refs(&self) -> &BTreeSet<Id>;
    fn external_references(&self) -> &[ExternalReference];
    fn created(&self) -> Option<&DateTime<Utc>>;
    fn modified(&self) -> Option<&DateTime<Utc>>;
}

impl Object for CommonProperties {
    fn id(&self) -> &Id {
        &self.id
    }

    fn created_by_ref(&self) -> Option<&Id> {
        self.created_by_ref.as_ref()
    }

    fn external_references(&self) -> &[ExternalReference] {
        &self.external_references
    }

    fn created(&self) -> Option<&DateTime<Utc>> {
        self.created.as_ref()
    }

    fn modified(&self) -> Option<&DateTime<Utc>> {
        self.modified.as_ref()
    }

    fn revoked(&self) -> bool {
        self.revoked
    }

    fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }

    fn object_marking_refs(&self) -> &BTreeSet<Id> {
        &self.object_marking_refs
    }
}

impl<T: AsRef<CommonProperties>> Object for T {
    fn id(&self) -> &Id {
        self.as_ref().id()
    }

    fn created_by_ref(&self) -> Option<&Id> {
        self.as_ref().created_by_ref()
    }

    fn revoked(&self) -> bool {
        self.as_ref().revoked()
    }

    fn labels(&self) -> &BTreeSet<String> {
        self.as_ref().labels()
    }

    fn object_marking_refs(&self) -> &BTreeSet<Id> {
        self.as_ref().object_marking_refs()
    }

    fn external_references(&self) -> &[ExternalReference] {
        self.as_ref().external_references()
    }

    fn created(&self) -> Option<&DateTime<Utc>> {
        self.as_ref().created()
    }

    fn modified(&self) -> Option<&DateTime<Utc>> {
        self.as_ref().modified()
    }
}

#[derive(Deserialize, strum::EnumDiscriminants)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[strum_discriminants(
    name(ObjectType),
    derive(strum::Display, strum::EnumString, PartialOrd, Ord, Hash),
    strum(serialize_all = "kebab-case")
)]
pub enum Declaration {
    AttackPattern(attack_pattern::Data),
    Bundle,
    CourseOfAction(serde::de::IgnoredAny),
    Identity(serde::de::IgnoredAny),
    IntrusionSet(intrusion_set::Data),
    Malware(malware::Data),
    MarkingDefinition,
    Relationship(relationship::Data),
    Tool(tool::Data),
    XMitreMatrix,
    XMitreTactic,
}
