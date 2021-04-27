use serde::{Deserialize, Serialize};
use stix::Id;

use crate::Variable;

use super::{CommonProperties, Target};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepPlaybook<T = crate::StandardTarget> {
    #[serde(flatten)]
    common: CommonProperties,
    playbook_id: Id,
    #[serde(flatten)]
    target: Target<T>,
    #[serde(default)]
    in_args: Vec<Variable>,
    #[serde(default)]
    out_args: Vec<Variable>,
}
