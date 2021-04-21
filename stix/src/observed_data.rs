use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{CommonProperties, Id};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct ObservedData {
    #[serde(flatten)]
    common: CommonProperties,
    pub first_observed: DateTime<Utc>,
    pub last_observed: DateTime<Utc>,
    pub number_observed: u32,
    #[serde(default)]
    pub object_refs: BTreeSet<Id>,
}

impl AsRef<CommonProperties> for ObservedData {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
