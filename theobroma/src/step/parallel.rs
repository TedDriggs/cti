use serde::{Deserialize, Serialize};
use stix::Id;

use crate::step_graph::ToStepRels;

use super::CommonProperties;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepParallel {
    #[serde(flatten)]
    common: CommonProperties,
    next_steps: Vec<Id>,
}

impl AsRef<CommonProperties> for StepParallel {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl<'a> ToStepRels<'a> for &'a StepParallel {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        self.common.to_step_rels(rels);
        rels.append_all_field("next_steps", &self.next_steps);
    }
}
