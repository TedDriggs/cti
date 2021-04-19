use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{vocab::ReportType, CommonProperties, Id};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Report {
    #[serde(flatten)]
    common: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub report_types: BTreeSet<ReportType>,
    pub published: DateTime<Utc>,
    pub object_refs: Vec<Id>,
}

impl AsRef<CommonProperties> for Report {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
