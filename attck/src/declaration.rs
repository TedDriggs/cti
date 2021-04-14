use serde::Deserialize;

use crate::{AttackPattern, Matrix, Tactic};

#[stix::extension]
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Declaration {
    AttackPattern(AttackPattern),
    #[serde(rename = "x-mitre-matrix")]
    Matrix(Matrix),
    #[serde(rename = "x-mitre-tactic")]
    Tactic(Tactic),
}
