use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    vocab::{AttackMotivation, AttackResourceLevel},
    CommonProperties,
};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct IntrusionSet {
    #[serde(flatten)]
    common: CommonProperties,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    aliases: BTreeSet<String>,
    #[serde(default)]
    pub first_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub goals: Vec<String>,
    #[serde(default)]
    pub resource_level: Option<AttackResourceLevel>,
    #[serde(default)]
    pub primary_motivation: Option<AttackMotivation>,
    #[serde(default)]
    pub secondary_motivations: BTreeSet<AttackMotivation>,
}

impl IntrusionSet {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn aliases(&self) -> &BTreeSet<String> {
        &self.aliases
    }
}

impl AsRef<CommonProperties> for IntrusionSet {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
