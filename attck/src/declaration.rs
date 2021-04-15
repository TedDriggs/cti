use serde::Deserialize;

use crate::{AttackPattern, Malware, Matrix, Tactic, Tool};

#[stix::declaration]
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Declaration {
    AttackPattern(AttackPattern),
    Malware(Malware),
    #[serde(rename = "x-mitre-matrix")]
    Matrix(Matrix),
    #[serde(rename = "x-mitre-tactic")]
    Tactic(Tactic),
    Tool(Tool),
}
