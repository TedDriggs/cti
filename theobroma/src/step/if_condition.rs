use serde::{Deserialize, Serialize};
use stix::Id;

use crate::step_graph::ToStepRels;

use super::{CommonProperties, Condition};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepIfCondition {
    #[serde(flatten)]
    common: CommonProperties,
    condition: Condition,
    on_true: Vec<Id>,
    on_false: Vec<Id>,
}

impl AsRef<CommonProperties> for StepIfCondition {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl<'data> ToStepRels<'data> for &'data StepIfCondition {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'data>) {
        self.common.to_step_rels(rels);
        rels.append_all_field("on_true", &self.on_true);
        rels.append_all_field("on_false", &self.on_false);
    }
}
