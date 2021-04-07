use crate::{actor, malware, relationship, technique, tool};
use serde::Deserialize;
use stix::Id;

#[derive(Deserialize)]
pub struct Bundle {
    pub id: Id,
    pub spec_version: String,
    pub objects: Vec<Object>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Object {
    #[serde(rename = "intrusion-set")]
    Actor(actor::Data),
    #[serde(rename = "attack-pattern")]
    Technique(technique::Data),
    CourseOfAction(serde::de::IgnoredAny),
    Identity(serde::de::IgnoredAny),
    Malware(malware::Data),
    MarkingDefinition(serde::de::IgnoredAny),
    XMitreMatrix(serde::de::IgnoredAny),
    XMitreTactic(serde::de::IgnoredAny),
    Relationship(relationship::Data),
    Tool(tool::Data),
}
