use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use stix::Id;

use crate::{variable::CommonDef, Step, Variable};

macro_rules! score {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
        pub struct $name($crate::bounded_u8::BoundedU8<0, 100>);

        impl $name {
            pub fn new(value: u8) -> Result<Self, $crate::OutOfBoundsError> {
                $crate::bounded_u8::BoundedU8::new(value).map($name)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<$name> for u8 {
            fn from(v: $name) -> Self {
                v.0.into()
            }
        }

        impl ::std::convert::TryFrom<u8> for $name {
            type Error = $crate::OutOfBoundsError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

score!(Priority);
score!(Severity);
score!(Impact);

#[derive(Deserialize, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum PlaybookType {
    Notification,
    Detection,
    Investigation,
    Prevention,
    Mitigation,
    Remediation,
    Attack,
}

/// Variable definition which allows for externally-defined variables.
#[derive(Deserialize, Serialize)]
pub struct PlaybookVariableDef {
    #[serde(flatten)]
    common: CommonDef,
    #[serde(default)]
    external: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybookFeature {
    ParallelProcessing,
    IfLogic,
    WhileLogic,
    SwitchLogic,
    TemporalLogic,
    DataMarkings,
    Extensions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PlaybookFeatures(IndexMap<PlaybookFeature, bool>);

#[derive(Deserialize)]
pub struct Playbook<T = crate::Target, C = crate::Command> {
    pub id: Id,
    pub spec_version: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub playbook_types: Vec<PlaybookType>,
    pub created_by: Id,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    #[serde(default)]
    pub revoked: Option<bool>,
    #[serde(default)]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(default)]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(default)]
    pub derived_from: Vec<Id>,
    #[serde(default)]
    pub priority: Option<Priority>,
    #[serde(default)]
    pub severity: Option<Severity>,
    #[serde(default)]
    pub impact: Option<Impact>,
    #[serde(default)]
    pub labels: BTreeSet<String>,
    #[serde(default)]
    pub features: PlaybookFeatures,
    #[serde(default)]
    pub markings: BTreeSet<Id>,
    #[serde(default)]
    pub playbook_variables: IndexMap<Variable, PlaybookVariableDef>,
    #[serde(default)]
    pub workflow_start: Option<Id>,
    #[serde(default)]
    pub workflow_exception: Option<Id>,
    #[serde(default = "IndexMap::new")]
    pub workflow: IndexMap<Id, Step<T, C>>,
    #[serde(default = "IndexMap::new")]
    pub target: IndexMap<Id, T>,
}
