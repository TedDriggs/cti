use serde::{Deserialize, Serialize};
use stix::Id;

use crate::{step_graph::ToStepRels, Variable};

use super::{CommonProperties, Target};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepPlaybook<T = crate::StandardTarget> {
    #[serde(flatten)]
    common: CommonProperties,
    playbook_id: Id,
    #[serde(flatten)]
    target: Option<Target<T>>,
    #[serde(default)]
    in_args: Vec<Variable>,
    #[serde(default)]
    out_args: Vec<Variable>,
}

impl<T> AsRef<CommonProperties> for StepPlaybook<T> {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl<'a, T> ToStepRels<'a> for &'a StepPlaybook<T> {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        self.common.to_step_rels(rels);
    }
}
