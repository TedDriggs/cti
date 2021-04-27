use serde::{Deserialize, Serialize};

use crate::Variable;

use super::{CommonProperties, Target};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepSingle<T = crate::StandardTarget, C = crate::Command> {
    #[serde(flatten)]
    common: CommonProperties,
    commands: Vec<C>,
    #[serde(flatten)]
    target: Target<T>,
    #[serde(default)]
    in_args: Vec<Variable>,
    #[serde(default)]
    out_args: Vec<Variable>,
}
