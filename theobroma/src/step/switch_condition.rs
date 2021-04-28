use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use stix::Id;

use crate::Variable;

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
