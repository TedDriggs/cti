use serde::Deserialize;
use stix::{
    AttackPattern, CourseOfAction, Identity, IntrusionSet, Location, Malware, MarkingDefinition,
    Relationship, Tool, Vulnerability,
};

use crate::{Matrix, Tactic};

#[derive(Deserialize, stix::Collection)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Declaration {
    #[stix(
        rel(Delivers, Malware),
        rel(Targets, Identity),
        rel(Targets, Location),
        rel(Targets, Vulnerability),
        rel(Uses, Tool),
        rel(Uses, Malware),
        rel(Uses, Vulnerability)
    )]
    AttackPattern(AttackPattern),
    #[stix(
        rel(Mitigates, AttackPattern),
        rel(Mitigates, Malware),
        rel(Mitigates, Tool),
        rel(Mitigates, Vulnerability),
        rel(Remediates, Malware),
        rel(Remediates, Vulnerability)
    )]
    CourseOfAction(CourseOfAction),
    #[stix(rel(LocatedAt, Location))]
    Identity(Identity),
    #[stix(
        rel(Targets, Identity),
        rel(Targets, Location),
        rel(Targets, Vulnerability),
        rel(Uses, AttackPattern),
        rel(Uses, Malware),
        rel(Uses, Tool)
    )]
    IntrusionSet(IntrusionSet),
    Location(Location),
    #[stix(
        rel(Controls, Malware),
        // rel(Downloads, File),
        rel(Downloads, Malware),
        rel(Downloads, Tool),
        // rel(Drops, File),
        rel(Drops, Malware),
        rel(Drops, Tool),
        rel(OriginatesFrom, Location),
        rel(Targets, Identity),
        // rel(Targets, Infrastructure),
        rel(Targets, Location),
        rel(Targets, Vulnerability),
        rel(Uses, AttackPattern),
        // rel(Uses, Infrastructure),
        rel(Uses, Malware),
        rel(Uses, Tool),
    )]
    Malware(Malware),
    MarkingDefinition(MarkingDefinition),
    #[serde(rename = "x-mitre-matrix")]
    Matrix(Matrix),
    #[serde(rename = "x-mitre-tactic")]
    Tactic(Tactic),
    Relationship(Relationship),
    #[stix(
        rel(Delivers, Malware),
        rel(Drops, Malware),
        rel(Has, Vulnerability),
        rel(Targets, Identity),
        // rel(Targets, Infrastructure),
        rel(Targets, Location),
        rel(Targets, Vulnerability),
        // rel(Uses, Infrastructure),
    )]
    Tool(Tool),
    Vulnerability(Vulnerability),
}
