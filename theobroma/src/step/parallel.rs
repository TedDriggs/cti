use serde::{Deserialize, Serialize};
use stix::Id;

use super::CommonProperties;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepParallel {
    #[serde(flatten)]
    common: CommonProperties,
    next_steps: Vec<Id>,
}
