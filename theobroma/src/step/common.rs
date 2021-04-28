use std::{fmt, time::Duration};

use indexmap::IndexMap;
use serde::{de::Error as _, Deserialize, Serialize};
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

/// Transition on step completion, covering the success and failure cases.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OnCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_completion: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_success: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<Id>,
}

impl OnCompletion {
    /// Check whether this completion behaves differently based on the success or failure
    /// of the step.
    pub fn is_conditional(&self) -> bool {
        self.on_success.is_some() || self.on_failure.is_some()
    }

    /// The step to execute in the success case.
    pub fn on_success(&self) -> Option<&Id> {
        self.on_success
            .as_ref()
            .or_else(|| self.on_completion.as_ref())
    }

    /// The step to execute in the failure case.
    pub fn on_failure(&self) -> Option<&Id> {
        self.on_failure
            .as_ref()
            .or_else(|| self.on_completion.as_ref())
    }
}

impl<'de> Deserialize<'de> for OnCompletion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Default, Deserialize)]
        #[serde(default)]
        struct RawOnCompletion {
            on_completion: Option<Id>,
            on_success: Option<Id>,
            on_failure: Option<Id>,
        }

        impl From<RawOnCompletion> for OnCompletion {
            fn from(r: RawOnCompletion) -> Self {
                Self {
                    on_completion: r.on_completion,
                    on_success: r.on_success,
                    on_failure: r.on_failure,
                }
            }
        }

        let raw = RawOnCompletion::deserialize(deserializer)?;

        if raw.on_completion.is_some() && (raw.on_success.is_some() || raw.on_failure.is_some()) {
            return Err(D::Error::custom(
                "`on_completion` is mutually-exclusive with `on_success` and `on_failure`",
            ));
        }

        Ok(Self::from(raw))
    }
}

impl<'a> ToStepRels<'a> for &'a OnCompletion {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        rels.append_all_field("on_completion", &self.on_completion);
        rels.append_all_field("on_success", &self.on_success);
        rels.append_all_field("on_failure", &self.on_failure);
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
    /// Destination step IDs after this step completes. This includes multiple properties
    /// from the CACAO spec, as those properties are mutually-exclusive and serve the same
    /// purpose.
    #[serde(default, flatten)]
    pub on_completion: OnCompletion,
}

impl fmt::Display for CommonProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => name.fmt(f),
            None => "<unnamed>".fmt(f),
        }?;

        if let Some(description) = &self.description {
            write!(f, " ({})", description)?;
        }

        Ok(())
    }
}

impl<'a> ToStepRels<'a> for &'a CommonProperties {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        self.on_completion.to_step_rels(rels);
    }
}
