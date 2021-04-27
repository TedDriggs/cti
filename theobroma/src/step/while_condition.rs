use serde::{Deserialize, Serialize};
use stix::Id;

use super::{CommonProperties, Condition};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepWhileCondition {
    #[serde(flatten)]
    common: CommonProperties,
    condition: Condition,
    on_true: Vec<Id>,
    on_false: Vec<Id>,
}
