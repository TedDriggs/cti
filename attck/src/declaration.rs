use serde::Deserialize;

use crate::{
    AttackPattern, CourseOfAction, DataComponent, DataSource, Malware, Matrix, Tactic, Tool,
};

#[stix::declaration]
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Declaration {
    AttackPattern(AttackPattern),
    CourseOfAction(CourseOfAction),
    #[serde(rename = "x-mitre-data-component")]
    DataComponent(DataComponent),
    #[serde(rename = "x-mitre-data-source")]
    DataSource(DataSource),
    Malware(Malware),
    #[serde(rename = "x-mitre-matrix")]
    Matrix(Matrix),
    #[serde(rename = "x-mitre-tactic")]
    Tactic(Tactic),
    Tool(Tool),
}
