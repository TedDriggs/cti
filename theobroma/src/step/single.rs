use serde::{Deserialize, Serialize};

use crate::{step_graph::ToStepRels, Variable};

use super::{CommonProperties, Target};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepSingle<T = crate::StandardTarget, C = crate::Command> {
    #[serde(flatten)]
    common: CommonProperties,
    commands: Vec<C>,
    #[serde(flatten)]
    target: Option<Target<T>>,
    #[serde(default)]
    in_args: Vec<Variable>,
    #[serde(default)]
    out_args: Vec<Variable>,
}

impl<T, C> AsRef<CommonProperties> for StepSingle<T, C> {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl<'a, T, C> ToStepRels<'a> for &'a StepSingle<T, C> {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        self.common.to_step_rels(rels);
    }
}
