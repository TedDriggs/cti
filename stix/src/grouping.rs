use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{vocab::GroupingContext, CommonProperties, Id};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Grouping {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub context: GroupingContext,
    pub object_refs: BTreeSet<Id>,
}

impl AsRef<CommonProperties> for Grouping {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
