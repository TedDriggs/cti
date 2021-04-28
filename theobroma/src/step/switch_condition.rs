use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use stix::Id;

use crate::{step_graph::ToStepRels, Variable};

use super::CommonProperties;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepSwitchCondition {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(rename = "switch")]
    switch_var: Variable,
    cases: IndexMap<String, Vec<Id>>,
}

impl StepSwitchCondition {
    /// Get the steps associated with the default case, if one was provided.
    ///
    /// The default case has the literal key `"default"`.
    pub fn default_case(&self) -> Option<&[Id]> {
        self.cases.get("default").map(|s| s.as_slice())
    }
}

impl AsRef<CommonProperties> for StepSwitchCondition {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl<'data> ToStepRels<'data> for &'data StepSwitchCondition {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'data>) {
        self.common.to_step_rels(rels);
        rels.enter_field("cases");
        for (case, ids) in &self.cases {
            rels.append_all_field(case, ids);
        }
        rels.exit_field("cases");
    }
}
