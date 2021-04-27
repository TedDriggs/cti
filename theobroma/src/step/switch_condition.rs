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
