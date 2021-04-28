use std::time::Duration;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use stix::{ExternalReference, Id};

use crate::{step_graph::ToStepRels, variable::CommonDef, Variable};

mod optional_duration {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Duration>, D::Error> {
        Ok(Option::<u64>::deserialize(deserializer)?.map(Duration::from_millis))
    }

    pub fn serialize<S: Serializer>(
        value: &Option<Duration>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value
            .as_ref()
            .map(Duration::as_millis)
            .serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonProperties {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub external_references: Vec<ExternalReference>,
    #[serde(default, with = "optional_duration")]
    pub delay: Option<Duration>,
    #[serde(default, with = "optional_duration")]
    pub timeout: Option<Duration>,
    #[serde(default)]
    pub step_variables: IndexMap<Variable, CommonDef>,
    #[serde(default)]
    pub owner: Option<Id>,
    // TODO represent these in a way that upholds the requirement
    // on_completion conflicts with on_success and on_failure
    #[serde(default)]
    on_completion: Option<Id>,
    #[serde(default)]
    on_success: Option<Id>,
    #[serde(default)]
    on_failure: Option<Id>,
}

impl<'a> ToStepRels<'a> for &'a CommonProperties {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        rels.append_all_field("on_completion", &self.on_completion);
        rels.append_all_field("on_success", &self.on_success);
        rels.append_all_field("on_failure", &self.on_failure);
    }
}
