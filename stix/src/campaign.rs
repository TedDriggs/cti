use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Campaign {
    #[serde(flatten)]
    common: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: BTreeSet<String>,
    #[serde(default)]
    pub first_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub objective: Option<String>,
}

impl AsRef<CommonProperties> for Campaign {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
