use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{CommonProperties, Id};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Sighting {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub first_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub count: Option<u32>,
    pub sighting_of_ref: Id,
    #[serde(default)]
    pub observed_data_refs: BTreeSet<Id>,
    #[serde(default)]
    pub where_sighted_refs: BTreeSet<Id>,
    #[serde(default)]
    pub summary: Option<bool>,
}

impl AsRef<CommonProperties> for Sighting {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
