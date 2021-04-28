use serde::{Deserialize, Serialize};
use stix::Id;

use crate::step_graph::ToStepRels;

use super::{CommonProperties, Condition};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepWhileCondition {
    #[serde(flatten)]
    common: CommonProperties,
    condition: Condition,
    on_true: Vec<Id>,
    on_false: Vec<Id>,
}

impl<'a> ToStepRels<'a> for &'a StepWhileCondition {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        self.common.to_step_rels(rels);
        rels.append_all_field("on_true", &self.on_true);
        rels.append_all_field("on_false", &self.on_false);
    }
}
