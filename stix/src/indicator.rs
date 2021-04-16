use std::{collections::BTreeSet, fmt};

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    vocab::{IndicatorType, PatternType},
    CommonProperties, KillChainPhase,
};

/// Detection pattern for an indicator.
///
/// In the future, this struct will provide conversion methods to attempt to produce
/// parsed representations of the pattern.
#[derive(Debug, Deserialize)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub pattern: String,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.pattern_type, self.pattern)
    }
}

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Indicator {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub indicator_types: BTreeSet<IndicatorType>,
    #[serde(flatten)]
    pub pattern: Pattern,
    #[serde(default)]
    pub pattern_version: String,
    pub valid_from: DateTime<Utc>,
    #[serde(default)]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(default)]
    pub kill_chain_phases: Vec<KillChainPhase>,
}

impl AsRef<CommonProperties> for Indicator {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
